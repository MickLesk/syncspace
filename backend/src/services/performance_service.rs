//! Performance service - system info and cache statistics

use crate::AppState;
use anyhow::Result;
use serde_json::{json, Value};
use sysinfo::{System, Disks};

/// Get cache statistics
pub async fn get_cache_stats(state: &AppState) -> Result<Value> {
    let has_redis = state.cache_manager.has_redis();
    
    // Get memory cache stats (approximation)
    let memory_entries = state.cache_manager.memory_cache.entry_count();
    let weighted_size = state.cache_manager.memory_cache.weighted_size();
    
    Ok(json!({
        "redis_connected": has_redis,
        "memory_cache": {
            "entries": memory_entries,
            "weighted_size_bytes": weighted_size,
            "max_capacity": state.cache_manager.config.memory_cache_size,
        },
        "hit_rate": 0.0, // TODO: Implement hit rate tracking
        "total_hits": 0,
        "total_misses": 0,
    }))
}

/// Get system information
pub async fn get_system_info(state: &AppState) -> Result<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let disks = Disks::new_with_refreshed_list();
    
    // Get total and available disk space
    let mut total_space = 0u64;
    let mut available_space = 0u64;
    
    for disk in disks.iter() {
        total_space += disk.total_space();
        available_space += disk.available_space();
    }
    
    // Get CPU and memory info
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    
    // CPU usage (average across all cores)
    let cpu_count = sys.cpus().len();
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpu_count as f32;
    
    Ok(json!({
        "cpu": {
            "cores": cpu_count,
            "usage_percent": cpu_usage,
            "brand": sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown"),
        },
        "memory": {
            "total_bytes": total_memory,
            "used_bytes": used_memory,
            "available_bytes": available_memory,
            "usage_percent": (used_memory as f64 / total_memory as f64) * 100.0,
        },
        "disk": {
            "total_bytes": total_space,
            "available_bytes": available_space,
            "used_bytes": total_space - available_space,
            "usage_percent": ((total_space - available_space) as f64 / total_space as f64) * 100.0,
        },
        "os": {
            "name": System::name().unwrap_or_else(|| "Unknown".to_string()),
            "version": System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            "kernel_version": System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        },
        "uptime_seconds": System::uptime(),
    }))
}
