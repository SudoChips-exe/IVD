use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JobEvent {
    Progress { percent: f32, speed: Option<String>, eta: Option<String> },
    Authenticating { method: String },
    Merging,
    Done { filename: String },
    Cancelled,
    Error { message: String },
}

pub struct JobHandle {
    pub tx: broadcast::Sender<JobEvent>,
    pub result: Arc<Mutex<Option<JobResult>>>,
    pub cancelled: Arc<AtomicBool>,
}

pub struct JobResult {
    pub file_path: String,
    pub filename: String,
}

#[derive(Clone)]
pub struct JobStore(Arc<Mutex<HashMap<String, JobHandle>>>);

impl JobStore {
    pub fn new() -> Self {
        JobStore(Arc::new(Mutex::new(HashMap::new())))
    }

    pub async fn create(&self, job_id: &str) -> (broadcast::Sender<JobEvent>, Arc<Mutex<Option<JobResult>>>, Arc<AtomicBool>) {
        let (tx, _) = broadcast::channel(128);
        let result = Arc::new(Mutex::new(None));
        let cancelled = Arc::new(AtomicBool::new(false));
        self.0.lock().await.insert(job_id.to_string(), JobHandle {
            tx: tx.clone(),
            result: result.clone(),
            cancelled: cancelled.clone(),
        });
        (tx, result, cancelled)
    }

    pub async fn subscribe(&self, job_id: &str) -> Option<broadcast::Receiver<JobEvent>> {
        self.0.lock().await.get(job_id).map(|j| j.tx.subscribe())
    }

    pub async fn cancel(&self, job_id: &str) -> bool {
        let guard = self.0.lock().await;
        if let Some(handle) = guard.get(job_id) {
            handle.cancelled.store(true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub async fn take_result(&self, job_id: &str) -> Option<JobResult> {
        let result_arc = {
            let guard = self.0.lock().await;
            guard.get(job_id)?.result.clone()
        };
        let mut inner = result_arc.lock().await;
        inner.take()
    }

    pub async fn cleanup(&self, job_id: &str) {
        self.0.lock().await.remove(job_id);
    }
}
