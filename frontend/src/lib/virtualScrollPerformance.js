/**
 * Performance Monitoring Utility for Virtual Scrolling
 * Tracks FPS, render times, and memory usage
 */

class VirtualScrollPerformanceMonitor {
  constructor(name = 'VirtualScroll') {
    this.name = name;
    this.frameCount = 0;
    this.lastFrameTime = performance.now();
    this.fps = 0;
    this.renderTimes = [];
    this.isMonitoring = false;
    this.rafId = null;
  }

  /**
   * Start monitoring performance
   */
  start() {
    if (this.isMonitoring) return;
    
    this.isMonitoring = true;
    this.frameCount = 0;
    this.lastFrameTime = performance.now();
    this.renderTimes = [];
    
    console.log(`[${this.name}] Performance monitoring started`);
    this.measureFrame();
  }

  /**
   * Stop monitoring performance
   */
  stop() {
    if (!this.isMonitoring) return;
    
    this.isMonitoring = false;
    if (this.rafId) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
    
    this.printStats();
  }

  /**
   * Measure frame rate
   */
  measureFrame() {
    if (!this.isMonitoring) return;

    const now = performance.now();
    const delta = now - this.lastFrameTime;
    
    if (delta >= 1000) {
      this.fps = Math.round((this.frameCount * 1000) / delta);
      this.frameCount = 0;
      this.lastFrameTime = now;
    }
    
    this.frameCount++;
    this.rafId = requestAnimationFrame(() => this.measureFrame());
  }

  /**
   * Mark start of render
   */
  startRender() {
    if (!this.isMonitoring) return;
    this.renderStartTime = performance.now();
  }

  /**
   * Mark end of render
   */
  endRender() {
    if (!this.isMonitoring || !this.renderStartTime) return;
    
    const renderTime = performance.now() - this.renderStartTime;
    this.renderTimes.push(renderTime);
    
    // Keep only last 100 render times
    if (this.renderTimes.length > 100) {
      this.renderTimes.shift();
    }
    
    // Warn if render took too long (>16ms = <60fps)
    if (renderTime > 16) {
      console.warn(`[${this.name}] Slow render: ${renderTime.toFixed(2)}ms (target: <16ms)`);
    }
    
    this.renderStartTime = null;
  }

  /**
   * Get current FPS
   */
  getFPS() {
    return this.fps;
  }

  /**
   * Get average render time
   */
  getAverageRenderTime() {
    if (this.renderTimes.length === 0) return 0;
    const sum = this.renderTimes.reduce((a, b) => a + b, 0);
    return sum / this.renderTimes.length;
  }

  /**
   * Get max render time
   */
  getMaxRenderTime() {
    if (this.renderTimes.length === 0) return 0;
    return Math.max(...this.renderTimes);
  }

  /**
   * Get min render time
   */
  getMinRenderTime() {
    if (this.renderTimes.length === 0) return 0;
    return Math.min(...this.renderTimes);
  }

  /**
   * Get memory usage (if available)
   */
  getMemoryUsage() {
    if (performance.memory) {
      return {
        used: (performance.memory.usedJSHeapSize / 1048576).toFixed(2) + ' MB',
        total: (performance.memory.totalJSHeapSize / 1048576).toFixed(2) + ' MB',
        limit: (performance.memory.jsHeapSizeLimit / 1048576).toFixed(2) + ' MB',
      };
    }
    return null;
  }

  /**
   * Print performance statistics
   */
  printStats() {
    console.group(`[${this.name}] Performance Statistics`);
    console.log(`FPS: ${this.fps}`);
    console.log(`Avg Render Time: ${this.getAverageRenderTime().toFixed(2)}ms`);
    console.log(`Min Render Time: ${this.getMinRenderTime().toFixed(2)}ms`);
    console.log(`Max Render Time: ${this.getMaxRenderTime().toFixed(2)}ms`);
    console.log(`Total Renders: ${this.renderTimes.length}`);
    
    const memory = this.getMemoryUsage();
    if (memory) {
      console.log(`Memory Used: ${memory.used} / ${memory.total} (Limit: ${memory.limit})`);
    }
    
    // Performance assessment
    const avgRender = this.getAverageRenderTime();
    if (avgRender < 8) {
      console.log('✅ Performance: EXCELLENT (120+ fps capable)');
    } else if (avgRender < 16) {
      console.log('✅ Performance: GOOD (60+ fps)');
    } else if (avgRender < 33) {
      console.log('⚠️ Performance: ACCEPTABLE (30-60 fps)');
    } else {
      console.log('❌ Performance: POOR (<30 fps)');
    }
    
    console.groupEnd();
  }

  /**
   * Create a performance report object
   */
  getReport() {
    return {
      name: this.name,
      fps: this.fps,
      renderTimes: {
        avg: this.getAverageRenderTime(),
        min: this.getMinRenderTime(),
        max: this.getMaxRenderTime(),
        count: this.renderTimes.length,
      },
      memory: this.getMemoryUsage(),
      timestamp: new Date().toISOString(),
    };
  }
}

/**
 * Global performance monitor instance
 */
let globalMonitor = null;

/**
 * Get or create global monitor
 */
export function getPerformanceMonitor(name = 'VirtualScroll') {
  if (!globalMonitor) {
    globalMonitor = new VirtualScrollPerformanceMonitor(name);
  }
  return globalMonitor;
}

/**
 * Start monitoring performance
 */
export function startPerformanceMonitoring(name) {
  const monitor = getPerformanceMonitor(name);
  monitor.start();
  return monitor;
}

/**
 * Stop monitoring performance
 */
export function stopPerformanceMonitoring() {
  if (globalMonitor) {
    globalMonitor.stop();
  }
}

/**
 * Measure render performance of a function
 */
export function measureRender(fn, name = 'Render') {
  const start = performance.now();
  const result = fn();
  const end = performance.now();
  const duration = end - start;
  
  if (duration > 16) {
    console.warn(`[${name}] Slow render: ${duration.toFixed(2)}ms`);
  } else {
    console.debug(`[${name}] Render time: ${duration.toFixed(2)}ms`);
  }
  
  return result;
}

/**
 * Measure async render performance
 */
export async function measureRenderAsync(fn, name = 'AsyncRender') {
  const start = performance.now();
  const result = await fn();
  const end = performance.now();
  const duration = end - start;
  
  if (duration > 16) {
    console.warn(`[${name}] Slow render: ${duration.toFixed(2)}ms`);
  } else {
    console.debug(`[${name}] Render time: ${duration.toFixed(2)}ms`);
  }
  
  return result;
}

/**
 * Generate test data for performance testing
 */
export function generateTestFiles(count = 10000) {
  const files = [];
  const extensions = ['txt', 'pdf', 'jpg', 'png', 'mp4', 'docx', 'xlsx', 'zip', 'js', 'css'];
  const folders = ['Documents', 'Images', 'Videos', 'Downloads', 'Projects', 'Archive'];
  
  console.log(`Generating ${count} test files...`);
  const start = performance.now();
  
  for (let i = 0; i < count; i++) {
    const isDirectory = Math.random() < 0.1; // 10% folders
    const ext = extensions[Math.floor(Math.random() * extensions.length)];
    const folderName = folders[Math.floor(Math.random() * folders.length)];
    
    files.push({
      id: `file_${i}`,
      name: isDirectory ? `${folderName}_${i}` : `file_${i}.${ext}`,
      file_path: `/test/${isDirectory ? folderName : 'file'}_${i}`,
      is_directory: isDirectory,
      size_bytes: isDirectory ? 0 : Math.floor(Math.random() * 10000000),
      modified_at: new Date(Date.now() - Math.random() * 31536000000).toISOString(),
      created_at: new Date(Date.now() - Math.random() * 63072000000).toISOString(),
      mime_type: isDirectory ? 'directory' : `application/${ext}`,
      owner: 'test_user',
    });
  }
  
  const end = performance.now();
  console.log(`Generated ${count} test files in ${(end - start).toFixed(2)}ms`);
  
  return files;
}

/**
 * Test virtual scrolling performance
 */
export async function testVirtualScrollingPerformance(itemCount = 10000, duration = 10000) {
  console.log(`Starting virtual scrolling performance test (${itemCount} items, ${duration}ms)`);
  
  const files = generateTestFiles(itemCount);
  const monitor = startPerformanceMonitoring(`VirtualScroll_${itemCount}_items`);
  
  // Simulate scrolling
  return new Promise((resolve) => {
    let scrollPosition = 0;
    const maxScroll = itemCount * 72; // Assuming 72px item height
    const scrollSpeed = 1000; // pixels per second
    
    const scrollInterval = setInterval(() => {
      scrollPosition += scrollSpeed / 60; // 60 fps
      
      if (scrollPosition >= maxScroll) {
        scrollPosition = 0; // Loop back
      }
      
      // Simulate scroll event
      monitor.startRender();
      // Simulate render work
      setTimeout(() => {
        monitor.endRender();
      }, Math.random() * 10);
    }, 1000 / 60); // 60 fps
    
    setTimeout(() => {
      clearInterval(scrollInterval);
      monitor.stop();
      
      const report = monitor.getReport();
      resolve(report);
    }, duration);
  });
}

/**
 * Compare performance between regular and virtual scrolling
 */
export async function comparePerformance() {
  console.group('Performance Comparison: Regular vs Virtual Scrolling');
  
  const testSizes = [100, 500, 1000, 5000, 10000];
  const results = [];
  
  for (const size of testSizes) {
    console.log(`\nTesting with ${size} items...`);
    
    const report = await testVirtualScrollingPerformance(size, 5000);
    results.push({
      size,
      ...report,
    });
  }
  
  console.table(results);
  console.groupEnd();
  
  return results;
}

export default {
  VirtualScrollPerformanceMonitor,
  getPerformanceMonitor,
  startPerformanceMonitoring,
  stopPerformanceMonitoring,
  measureRender,
  measureRenderAsync,
  generateTestFiles,
  testVirtualScrollingPerformance,
  comparePerformance,
};
