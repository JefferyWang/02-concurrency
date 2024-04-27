use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("Key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("Key {} not found", key))?;
        counter.fetch_sub(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {:?}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
