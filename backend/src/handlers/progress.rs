use actix_web::{web, HttpResponse};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

use crate::jobs::{JobEvent, JobStore};

pub async fn progress_sse(
    job_id: web::Path<String>,
    jobs: web::Data<JobStore>,
) -> HttpResponse {
    let rx = match jobs.subscribe(&job_id).await {
        Some(rx) => rx,
        None => {
            return HttpResponse::NotFound()
                .json(serde_json::json!({"error": "job_not_found", "message": "Job not found"}))
        }
    };

    // Bridge broadcast → mpsc so we can use ReceiverStream
    let (mpsc_tx, mpsc_rx) = tokio::sync::mpsc::channel::<actix_web::web::Bytes>(64);
    let mut broadcast_rx = rx;

    tokio::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(event) => {
                    let is_terminal = matches!(event, JobEvent::Done { .. } | JobEvent::Error { .. });
                    if let Ok(data) = serde_json::to_string(&event) {
                        let bytes = actix_web::web::Bytes::from(format!("data: {}\n\n", data));
                        if mpsc_tx.send(bytes).await.is_err() {
                            break;
                        }
                    }
                    if is_terminal {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    let stream = ReceiverStream::new(mpsc_rx).map(Ok::<_, actix_web::Error>);

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("X-Accel-Buffering", "no"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(Box::pin(stream))
}
