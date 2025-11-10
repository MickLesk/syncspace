// Performance Store - Backend-synchronized performance monitoring
import { writable, derived, get } from 'svelte/store';
import api from '../lib/api.js';

// Performance metrics state
export const performanceMetrics = writable({
    cache_hit_ratio: 0,
    average_response_time: 0,
    active_connections: 0,
    memory_usage: 0,
    disk_usage: 0,
    cpu_usage: 0,
    last_updated: null
});

// Performance history for charts
export const performanceHistory = writable([]);

// Cache statistics
export const cacheStats = writable({
    memory_cache_entries: 0,
    redis_connected: false,
    cache_hit_ratio: 0,
    total_requests: 0,
    cache_hits: 0
});

// Background jobs status
export const backgroundJobs = writable({
    jobs: [],
    queue_length: 0,
    active_workers: 0
});

// System information
export const systemInfo = writable({
    cpu_cores: 0,
    memory_total: "Unknown",
    disk_space: "Unknown",
    uptime: "Unknown",
    version: "0.3.0",
    rust_version: "Unknown",
    features: {}
});

// Performance monitor class
class PerformanceMonitor {
    constructor() {
        this.isMonitoring = false;
        this.monitoringInterval = null;
        this.updateInterval = 30000; // 30 seconds
        
        this.init();
    }

    async init() {
        // Load initial data
        await this.loadSystemInfo();
        await this.loadCacheStats();
        
        // Start monitoring if enabled
        const savedSettings = localStorage.getItem('performanceMonitorSettings');
        if (savedSettings) {
            const settings = JSON.parse(savedSettings);
            if (settings.autoStart) {
                this.startMonitoring();
            }
        }
    }

    // Load current performance metrics
    async loadMetrics() {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getMetrics();
            // if (response) {
            //     performanceMetrics.set({
            //         ...response,
            //         last_updated: new Date()
            //     });
            //     
            //     performanceHistory.update(history => {
            //         const newHistory = [...history, {
            //             timestamp: new Date(),
            //             ...response
            //         }];
            //         
            //         if (newHistory.length > 100) {
            //             newHistory.shift();
            //         }
            //         
            //         return newHistory;
            //     });
            // }
        } catch (error) {
            console.error('Failed to load performance metrics:', error);
        }
    }

    // Load performance history
    async loadHistory(limit = 50) {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getHistory(limit);
            // if (response) {
            //     const formattedHistory = response.map(item => ({
            //         timestamp: new Date(item.timestamp || Date.now()),
            //         ...item
            //     }));
            //     performanceHistory.set(formattedHistory);
            // }
        } catch (error) {
            console.error('Failed to load performance history:', error);
        }
    }

    // Load background jobs status
    async loadBackgroundJobs() {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getBackgroundJobs();
            // if (response) {
            //     backgroundJobs.set(response);
            // }
            console.warn('[Performance] API not available - skipping background jobs');
        } catch (error) {
            console.error('Failed to load background jobs:', error);
        }
    }

    // Load system information
    async loadSystemInfo() {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getSystemInfo();
            // if (response) {
            //     systemInfo.set(response);
            // }
            console.warn('[Performance] API not available - skipping system info');
        } catch (error) {
            console.error('Failed to load system info:', error);
        }
    }

    // Load cache statistics
    async loadCacheStats() {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getCacheStats();
            // if (response) {
            //     cacheStats.set(response);
            // }
            console.warn('[Performance] API not available - skipping cache stats');
        } catch (error) {
            console.error('Failed to load cache stats:', error);
        }
    }

    // Start performance monitoring
    startMonitoring() {
        if (this.isMonitoring) return;
        
        this.isMonitoring = true;
        this.loadMetrics(); // Load immediately
        
        this.monitoringInterval = setInterval(() => {
            this.loadMetrics();
            this.loadCacheStats();
            this.loadBackgroundJobs();
        }, this.updateInterval);
        
        console.log('Performance monitoring started');
    }

    // Stop performance monitoring
    stopMonitoring() {
        if (!this.isMonitoring) return;
        
        this.isMonitoring = false;
        
        if (this.monitoringInterval) {
            clearInterval(this.monitoringInterval);
            this.monitoringInterval = null;
        }
        
        console.log('Performance monitoring stopped');
    }

    // Clear cache
    async clearCache() {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.clearCache();
            // if (response && response.status) {
            //     console.log('Cache cleared successfully');
            //     await this.loadCacheStats();
            //     return true;
            // }
            console.warn('[Performance] API not available - cannot clear cache');
            return false;
        } catch (error) {
            console.error('Failed to clear cache:', error);
            throw error;
        }
    }

    // Queue background job
    async queueJob(jobType, payload, priority = 0) {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.queueJob({
            //     job_type: jobType,
            //     payload: payload,
            //     priority: priority
            // });
            // if (response && response.job_id) {
            //     console.log(`Job queued: ${response.job_id}`);
            //     await this.loadBackgroundJobs();
            //     return response.job_id;
            // }
            console.warn('[Performance] API not available - cannot queue job');
            return null;
        } catch (error) {
            console.error('Failed to queue job:', error);
            throw error;
        }
    }

    // Get job status
    async getJobStatus(jobId) {
        try {
            // TODO: Re-enable when performance API is available
            // const response = await api.performance.getJobStatus(jobId);
            // return response;
            console.warn('[Performance] API not available - cannot get job status');
            return null;
        } catch (error) {
            console.error('Failed to get job status:', error);
            throw error;
        }
    }

    // Set monitoring interval
    setUpdateInterval(intervalMs) {
        this.updateInterval = intervalMs;
        
        if (this.isMonitoring) {
            this.stopMonitoring();
            this.startMonitoring();
        }
    }

    // Export performance data
    exportData() {
        const currentMetrics = get(performanceMetrics);
        const currentHistory = get(performanceHistory);
        const currentCacheStats = get(cacheStats);
        const currentSystemInfo = get(systemInfo);
        
        return {
            timestamp: new Date().toISOString(),
            current_metrics: currentMetrics,
            history: currentHistory,
            cache_stats: currentCacheStats,
            system_info: currentSystemInfo
        };
    }

    // Save monitoring settings
    saveSettings(settings) {
        localStorage.setItem('performanceMonitorSettings', JSON.stringify(settings));
    }

    // Load monitoring settings
    loadSettings() {
        const saved = localStorage.getItem('performanceMonitorSettings');
        return saved ? JSON.parse(saved) : {
            autoStart: false,
            updateInterval: 30000,
            enableAlerts: true,
            thresholds: {
                cpu_usage: 80,
                memory_usage: 85,
                cache_hit_ratio: 0.5
            }
        };
    }
}

// Create performance monitor instance
export const performanceMonitor = new PerformanceMonitor();

// Derived stores for convenience
export const isMonitoring = derived(
    performanceMetrics,
    ($metrics) => $metrics.last_updated !== null
);

export const performanceScore = derived(
    performanceMetrics,
    ($metrics) => {
        if (!$metrics.last_updated) return 0;
        
        // Calculate overall performance score (0-100)
        const cacheScore = $metrics.cache_hit_ratio * 100;
        const cpuScore = Math.max(0, 100 - $metrics.cpu_usage);
        const responseTimeScore = Math.max(0, 100 - ($metrics.average_response_time / 10));
        
        return Math.round((cacheScore + cpuScore + responseTimeScore) / 3);
    }
);

export const performanceStatus = derived(
    performanceScore,
    ($score) => {
        if ($score >= 80) return { level: 'excellent', color: 'success' };
        if ($score >= 60) return { level: 'good', color: 'info' };
        if ($score >= 40) return { level: 'fair', color: 'warning' };
        return { level: 'poor', color: 'error' };
    }
);

// Helper functions for performance monitoring
export const performanceUtils = {
    formatBytes(bytes) {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    },

    formatDuration(ms) {
        if (ms < 1000) return `${ms}ms`;
        const seconds = Math.floor(ms / 1000);
        if (seconds < 60) return `${seconds}s`;
        const minutes = Math.floor(seconds / 60);
        if (minutes < 60) return `${minutes}m ${seconds % 60}s`;
        const hours = Math.floor(minutes / 60);
        return `${hours}h ${minutes % 60}m`;
    },

    formatPercentage(ratio) {
        return `${(ratio * 100).toFixed(1)}%`;
    },

    getPerformanceColor(value, thresholds) {
        if (value >= thresholds.excellent) return 'success';
        if (value >= thresholds.good) return 'info';
        if (value >= thresholds.fair) return 'warning';
        return 'error';
    }
};