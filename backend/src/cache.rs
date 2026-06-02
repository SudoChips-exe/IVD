#![allow(dead_code)]

use crate::models::VideoMetadata;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct CacheEntry {
    metadata: VideoMetadata,
    expires_at: Instant,
}

pub struct MetadataCache {
    ttl: Duration,
    entries: Mutex<HashMap<String, CacheEntry>>,
}

impl MetadataCache {
    pub fn new(ttl_seconds: u64) -> Self {
        MetadataCache {
            ttl: Duration::from_secs(ttl_seconds),
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<VideoMetadata> {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();

        entries.retain(|_, entry| entry.expires_at > now);

        entries.get(key).map(|entry| entry.metadata.clone())
    }

    pub fn insert(&self, key: String, metadata: VideoMetadata) {
        let mut entries = self.entries.lock().unwrap();
        let expires_at = Instant::now() + self.ttl;

        entries.insert(
            key,
            CacheEntry {
                metadata,
                expires_at,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_returns_cached_metadata_within_ttl() {
        let cache = MetadataCache::new(60);
        let metadata = VideoMetadata {
            title: "Test".to_string(),
            duration_seconds: 1,
            author: "Author".to_string(),
            video_url: "https://example.com/video.mp4".to_string(),
            audio_url: None,
            thumbnail_url: String::new(),
            original_platform: "test".to_string(),
            file_size_bytes: Some(1024),
        };

        cache.insert("https://example.com/test".to_string(), metadata.clone());
        let cached = cache.get("https://example.com/test");

        assert!(cached.is_some());
        assert_eq!(cached.unwrap().video_url, metadata.video_url);
    }
}
