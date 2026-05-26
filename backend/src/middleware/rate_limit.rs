use crate::error::AppError;
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, ResponseError,
};
use futures::future::LocalBoxFuture;
use std::collections::HashMap;
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

struct RateLimitCounter {
    window: u64,
    count: u32,
}

struct RateLimitState {
    global: RateLimitCounter,
    per_ip: HashMap<String, RateLimitCounter>,
}

#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimitState>>,
    max_requests_per_minute: u32,
    max_requests_per_ip_per_minute: u32,
}

impl RateLimiter {
    pub fn new(max_requests_per_minute: u32, max_requests_per_ip_per_minute: u32) -> Self {
        let state = RateLimitState {
            global: RateLimitCounter { window: 0, count: 0 },
            per_ip: HashMap::new(),
        };

        RateLimiter {
            state: Arc::new(Mutex::new(state)),
            max_requests_per_minute,
            max_requests_per_ip_per_minute,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            state: self.state.clone(),
            max_requests_per_minute: self.max_requests_per_minute,
            max_requests_per_ip_per_minute: self.max_requests_per_ip_per_minute,
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    state: Arc<Mutex<RateLimitState>>,
    max_requests_per_minute: u32,
    max_requests_per_ip_per_minute: u32,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let state = self.state.clone();
        let max_global = self.max_requests_per_minute;
        let max_ip = self.max_requests_per_ip_per_minute;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let window = now / 60;
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let mut guard = state.lock().unwrap();

        if guard.global.window != window {
            guard.global.window = window;
            guard.global.count = 0;
            guard.per_ip.retain(|_, entry| entry.window == window);
        }

        let global_count = guard.global.count;
        let ip_count = {
            let ip_counter = guard.per_ip.entry(ip.clone()).or_insert(RateLimitCounter {
                window,
                count: 0,
            });

            if ip_counter.window != window {
                ip_counter.window = window;
                ip_counter.count = 0;
            }

            ip_counter.count
        };

        if ip_count + 1 > max_ip || global_count + 1 > max_global {
            let retry_after = 60 - (now % 60);
            let err = AppError::RateLimited { retry_after };
            let response = req.into_response(err.error_response().map_into_boxed_body());
            return Box::pin(async move { Ok(response) });
        }

        {
            let ip_counter = guard.per_ip.entry(ip.clone()).or_insert(RateLimitCounter {
                window,
                count: 0,
            });
            ip_counter.count += 1;
        }
        guard.global.count += 1;
        drop(guard);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
