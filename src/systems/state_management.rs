//! Phase 4: State Management
//! 
//! Implements state management and persistence for macOS GUI.

use crate::core::error::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Session state - global state management
pub struct SessionState {
    state: Arc<Mutex<HashMap<String, String>>>,
}

impl SessionState {
    /// Create a new session state
    pub fn new() -> Self {
        SessionState {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a value in session state
    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| "Failed to lock state")?;
        state.insert(key.into(), value.into());
        Ok(())
    }

    /// Get a value from session state
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let state = self.state.lock().map_err(|_| "Failed to lock state")?;
        Ok(state.get(key).cloned())
    }

    /// Remove a value from session state
    pub fn remove(&self, key: &str) -> Result<Option<String>> {
        let mut state = self.state.lock().map_err(|_| "Failed to lock state")?;
        Ok(state.remove(key))
    }

    /// Clear all state
    pub fn clear(&self) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| "Failed to lock state")?;
        state.clear();
        Ok(())
    }

    /// Get all keys
    pub fn keys(&self) -> Result<Vec<String>> {
        let state = self.state.lock().map_err(|_| "Failed to lock state")?;
        Ok(state.keys().cloned().collect())
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &str) -> Result<bool> {
        let state = self.state.lock().map_err(|_| "Failed to lock state")?;
        Ok(state.contains_key(key))
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SessionState {
    fn clone(&self) -> Self {
        SessionState {
            state: Arc::clone(&self.state),
        }
    }
}

/// Query parameters - URL query string binding
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    /// Create a new query params from string
    pub fn from_string(query_string: &str) -> Result<Self> {
        let mut params = HashMap::new();
        
        if query_string.is_empty() {
            return Ok(QueryParams { params });
        }

        for param in query_string.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            }
        }

        Ok(QueryParams { params })
    }

    /// Get a parameter value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Set a parameter value
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.params.insert(key.into(), value.into());
    }

    /// Get all parameters
    pub fn all(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Convert to query string
    pub fn to_string(&self) -> String {
        self.params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams {
            params: HashMap::new(),
        }
    }
}

/// Cache entry with TTL
pub struct CacheEntry<T> {
    value: T,
    created_at: std::time::Instant,
    ttl_seconds: Option<u64>,
}

impl<T: Clone> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(value: T, ttl_seconds: Option<u64>) -> Self {
        CacheEntry {
            value,
            created_at: std::time::Instant::now(),
            ttl_seconds,
        }
    }

    /// Check if entry is expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_seconds {
            self.created_at.elapsed().as_secs() > ttl
        } else {
            false
        }
    }

    /// Get the value if not expired
    pub fn get(&self) -> Option<T> {
        if self.is_expired() {
            None
        } else {
            Some(self.value.clone())
        }
    }

    /// Get age in seconds
    pub fn age_seconds(&self) -> u64 {
        self.created_at.elapsed().as_secs()
    }
}

/// Data cache - caching system for data
pub struct DataCache<T: Clone> {
    cache: Arc<Mutex<HashMap<String, CacheEntry<T>>>>,
}

impl<T: Clone> DataCache<T> {
    /// Create a new data cache
    pub fn new() -> Self {
        DataCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a cached value
    pub fn set(&self, key: impl Into<String>, value: T, ttl_seconds: Option<u64>) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.insert(key.into(), CacheEntry::new(value, ttl_seconds));
        Ok(())
    }

    /// Get a cached value
    pub fn get(&self, key: &str) -> Result<Option<T>> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        
        if let Some(entry) = cache.get(key) {
            if let Some(value) = entry.get() {
                return Ok(Some(value));
            } else {
                cache.remove(key);
            }
        }
        
        Ok(None)
    }

    /// Remove a cached value
    pub fn remove(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.remove(key);
        Ok(())
    }

    /// Clear all cache
    pub fn clear(&self) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.clear();
        Ok(())
    }

    /// Get cache size
    pub fn size(&self) -> Result<usize> {
        let cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        Ok(cache.len())
    }
}

impl<T: Clone> Default for DataCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for DataCache<T> {
    fn clone(&self) -> Self {
        DataCache {
            cache: Arc::clone(&self.cache),
        }
    }
}

/// Resource cache - caching system for resources
pub struct ResourceCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry<Vec<u8>>>>>,
}

impl ResourceCache {
    /// Create a new resource cache
    pub fn new() -> Self {
        ResourceCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a cached resource
    pub fn set(&self, key: impl Into<String>, data: Vec<u8>, ttl_seconds: Option<u64>) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.insert(key.into(), CacheEntry::new(data, ttl_seconds));
        Ok(())
    }

    /// Get a cached resource
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        
        if let Some(entry) = cache.get(key) {
            if let Some(value) = entry.get() {
                return Ok(Some(value));
            } else {
                cache.remove(key);
            }
        }
        
        Ok(None)
    }

    /// Remove a cached resource
    pub fn remove(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.remove(key);
        Ok(())
    }

    /// Clear all cache
    pub fn clear(&self) -> Result<()> {
        let mut cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        cache.clear();
        Ok(())
    }

    /// Get total cache size in bytes
    pub fn size_bytes(&self) -> Result<usize> {
        let cache = self.cache.lock().map_err(|_| "Failed to lock cache")?;
        Ok(cache.values().map(|e| e.value.len()).sum())
    }
}

impl Default for ResourceCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ResourceCache {
    fn clone(&self) -> Self {
        ResourceCache {
            cache: Arc::clone(&self.cache),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state() {
        let state = SessionState::new();
        state.set("key1", "value1").unwrap();
        assert_eq!(state.get("key1").unwrap(), Some("value1".to_string()));
    }

    #[test]
    fn test_session_state_remove() {
        let state = SessionState::new();
        state.set("key1", "value1").unwrap();
        let removed = state.remove("key1").unwrap();
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(state.get("key1").unwrap(), None);
    }

    #[test]
    fn test_session_state_keys() {
        let state = SessionState::new();
        state.set("key1", "value1").unwrap();
        state.set("key2", "value2").unwrap();
        let keys = state.keys().unwrap();
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_query_params() {
        let params = QueryParams::from_string("name=John&age=30").unwrap();
        assert_eq!(params.get("name"), Some("John"));
        assert_eq!(params.get("age"), Some("30"));
    }

    #[test]
    fn test_cache_entry_no_ttl() {
        let entry = CacheEntry::new("value", None);
        assert!(!entry.is_expired());
        assert_eq!(entry.get(), Some("value"));
    }

    #[test]
    fn test_data_cache() {
        let cache: DataCache<String> = DataCache::new();
        cache.set("key1", "value1".to_string(), None).unwrap();
        assert_eq!(cache.get("key1").unwrap(), Some("value1".to_string()));
    }

    #[test]
    fn test_resource_cache() {
        let cache = ResourceCache::new();
        let data = vec![1, 2, 3, 4, 5];
        cache.set("resource1", data.clone(), None).unwrap();
        assert_eq!(cache.get("resource1").unwrap(), Some(data));
    }
}
