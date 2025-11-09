/**
 * Image Optimization Utilities
 * Responsive images, format conversion, lazy loading, performance monitoring
 * 
 * Usage:
 * import { getImageUrls, preloadImage } from './lib/imageOptimization.js';
 * 
 * const urls = getImageUrls('/images/photo.jpg');
 * // { avif: "...", webp: "...", original: "...", width: 1280, height: 720 }
 */

// Image CDN configuration (fallback to direct paths)
const CDN_BASE =
  typeof process !== 'undefined' && process.env.VITE_CDN_URL
    ? process.env.VITE_CDN_URL
    : '';

// Breakpoints for responsive images
const BREAKPOINTS = {
  mobile: 360,
  mobileLarge: 540,
  tablet: 768,
  tabletLarge: 1024,
  desktop: 1280,
  desktopLarge: 1920,
  ultra: 2560,
};

// Track supported formats (cached after first check)
let supportedFormats = null;

/**
 * Generate srcset string for images with multiple sizes and DPR
 * @param {string} imagePath - Image path or URL
 * @param {string} format - Image format (avif, webp, jpg, png)
 * @returns {string} srcset string
 */
export function generateSrcset(imagePath, format = 'webp') {
  const sizes = [
    { width: 180, dpr: 1 },
    { width: 360, dpr: 1 },
    { width: 360, dpr: 2 },
    { width: 540, dpr: 1 },
    { width: 640, dpr: 1 },
    { width: 640, dpr: 2 },
    { width: 1024, dpr: 1 },
    { width: 1280, dpr: 1 },
    { width: 1920, dpr: 1 },
  ];

  // If no CDN, use direct path with simple sizes
  if (!CDN_BASE) {
    return sizes
      .map(({ width, dpr }) => `${imagePath} ${width * dpr}w`)
      .join(', ')
      .split(', ')
      .filter((v, i, a) => a.indexOf(v) === i)
      .join(', ');
  }

  // Use CDN with format conversion
  return sizes
    .map(({ width, dpr }) => {
      const url = `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&width=${width}&format=${format}&dpr=${dpr}`;
      return `${url} ${width * dpr}w`;
    })
    .join(', ');
}

/**
 * Get image URLs for all supported formats
 * @param {string} imagePath - Image path
 * @param {Object} options - Configuration options
 * @returns {Object} URLs for avif, webp, original formats
 */
export function getImageUrls(
  imagePath,
  options = { width: 1280, height: 720 }
) {
  return {
    avif: generateSrcset(imagePath, 'avif'),
    webp: generateSrcset(imagePath, 'webp'),
    original: generateSrcset(imagePath, 'original'),
    width: options.width,
    height: options.height,
  };
}

/**
 * Generate low-quality placeholder image (blur hash)
 * @param {string} imagePath - Image path
 * @param {number} width - Placeholder width (default 50)
 * @returns {Promise<string>} Data URL or object URL
 */
export async function generateBlurHash(imagePath, width = 50) {
  try {
    if (CDN_BASE) {
      const url = `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&width=${width}&format=jpg&quality=10`;
      return url;
    }

    // Fallback: create a small canvas placeholder
    const canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = Math.round((width * 9) / 16); // 16:9 aspect
    const ctx = canvas.getContext('2d');

    // Create a gradient placeholder
    const gradient = ctx.createLinearGradient(0, 0, width, canvas.height);
    gradient.addColorStop(0, '#e5e7eb');
    gradient.addColorStop(1, '#d1d5db');
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, width, canvas.height);

    return canvas.toDataURL('image/jpeg', 0.7);
  } catch (error) {
    console.warn('Failed to generate blur hash:', error);
    return '';
  }
}

/**
 * Preload image in browser cache
 * @param {string} imagePath - Image path
 * @param {string} format - Format to preload (webp, avif, original)
 */
export function preloadImage(imagePath, format = 'webp') {
  if (typeof document === 'undefined') return;

  const link = document.createElement('link');
  link.rel = 'preload';
  link.as = 'image';

  if (CDN_BASE) {
    link.href = `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&format=${format}`;
  } else {
    link.href = imagePath;
  }

  if (format === 'webp') link.type = 'image/webp';
  else if (format === 'avif') link.type = 'image/avif';

  document.head.appendChild(link);
}

/**
 * Batch preload multiple images
 * @param {string[]} imagePaths - Array of image paths
 * @param {string} format - Format to preload
 */
export function preloadImages(imagePaths, format = 'webp') {
  imagePaths.forEach((path) => preloadImage(path, format));
}

/**
 * Check if browser supports specific image format
 * @param {string} mimeType - MIME type (image/webp, image/avif, etc)
 * @returns {Promise<boolean>} Support status
 */
export async function supportsFormat(mimeType) {
  return new Promise((resolve) => {
    const img = new Image();
    img.onload = () => resolve(true);
    img.onerror = () => resolve(false);

    if (mimeType === 'image/webp') {
      img.src =
        'data:image/webp;base64,UklGRjoIAABXRUJQVlA4IC4IAAAQAQCDAG0Abk44CgCdASoBAAEA/v3AgAA=';
    } else if (mimeType === 'image/avif') {
      img.src =
        'data:image/avif;base64,AAAAIGZ0eXBhdmlmAAAAAG1pZGEA';
    } else {
      resolve(true); // Assume support for standard formats
    }
  });
}

/**
 * Get browser's supported image formats (cached)
 * @returns {Promise<string[]>} Supported formats in preference order
 */
export async function getSupportedFormats() {
  if (supportedFormats !== null) {
    return supportedFormats;
  }

  const supported = [];

  try {
    // Check AVIF support (smallest)
    if (await supportsFormat('image/avif')) {
      supported.push('avif');
    }

    // Check WebP support (mid-size)
    if (await supportsFormat('image/webp')) {
      supported.push('webp');
    }
  } catch (error) {
    console.warn('Error checking image formats:', error);
  }

  // All browsers support JPG/PNG (fallback)
  supported.push('original');

  supportedFormats = supported;
  return supported;
}

/**
 * Create optimized thumbnail with CDN support
 * @param {string} imagePath - Image path
 * @param {number} size - Thumbnail size in pixels
 * @returns {string} Thumbnail URL
 */
export function getThumbnail(imagePath, size = 200) {
  if (CDN_BASE) {
    return `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&width=${size}&format=webp&quality=85`;
  }
  return imagePath; // Fallback to original
}

/**
 * Get multiple thumbnail sizes
 * @param {string} imagePath - Image path
 * @param {number[]} sizes - Array of sizes to generate
 * @returns {Object} Map of size to URL
 */
export function getThumbnails(imagePath, sizes = [100, 200, 400]) {
  return Object.fromEntries(
    sizes.map((size) => [size, getThumbnail(imagePath, size)])
  );
}

/**
 * Monitor image loading performance
 * @param {HTMLImageElement} img - Image element
 * @returns {Promise<Object>} Performance metrics
 */
export async function monitorImageLoad(img) {
  return new Promise((resolve) => {
    if (!img) {
      resolve({
        success: false,
        error: 'Image element not found',
      });
      return;
    }

    const startTime = performance.now();

    function onLoad() {
      const duration = performance.now() - startTime;
      cleanup();

      resolve({
        success: true,
        duration,
        width: img.naturalWidth,
        height: img.naturalHeight,
        size: img.naturalWidth * img.naturalHeight,
        aspect: img.naturalWidth / img.naturalHeight,
      });
    }

    function onError(error) {
      const duration = performance.now() - startTime;
      cleanup();

      resolve({
        success: false,
        duration,
        error: error?.message || 'Image load failed',
      });
    }

    function cleanup() {
      img.removeEventListener('load', onLoad);
      img.removeEventListener('error', onError);
    }

    img.addEventListener('load', onLoad);
    img.addEventListener('error', onError);

    // Timeout after 30 seconds
    setTimeout(() => {
      cleanup();
      if (!img.complete) {
        resolve({
          success: false,
          duration: 30000,
          error: 'Image load timeout',
        });
      }
    }, 30000);
  });
}

/**
 * Calculate optimal image size for container
 * @param {HTMLElement} container - Container element
 * @param {number} aspectRatio - Image aspect ratio (width/height)
 * @returns {Object} Width and height
 */
export function calculateImageSize(container, aspectRatio = 16 / 9) {
  if (!container) {
    return { width: 1280, height: 720 };
  }

  const rect = container.getBoundingClientRect();
  const width = Math.round(rect.width);
  const height = Math.round(width / aspectRatio);

  return { width, height };
}

/**
 * Convert image path to WebP format
 * @param {string} imagePath - Original image path
 * @returns {string} WebP image URL
 */
export function toWebP(imagePath) {
  if (CDN_BASE) {
    return `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&format=webp`;
  }
  return imagePath;
}

/**
 * Convert image path to AVIF format
 * @param {string} imagePath - Original image path
 * @returns {string} AVIF image URL
 */
export function toAVIF(imagePath) {
  if (CDN_BASE) {
    return `${CDN_BASE}/image?path=${encodeURIComponent(imagePath)}&format=avif`;
  }
  return imagePath;
}

/**
 * Create an IntersectionObserver for lazy loading images
 * @param {string} selector - CSS selector for images to lazy load
 * @param {Object} options - IntersectionObserver options
 */
export function lazyLoadImages(
  selector = 'img[loading="lazy"]',
  options = { rootMargin: '100px' }
) {
  if (typeof window === 'undefined' || !('IntersectionObserver' in window)) {
    // Fallback for browsers without IntersectionObserver
    const images = document.querySelectorAll(selector);
    images.forEach((img) => {
      if (img.dataset.src) {
        img.src = img.dataset.src;
      }
      if (img.dataset.srcset) {
        img.srcset = img.dataset.srcset;
      }
    });
    return;
  }

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        const img = entry.target;

        if (img.dataset.src) {
          img.src = img.dataset.src;
        }
        if (img.dataset.srcset) {
          img.srcset = img.dataset.srcset;
        }

        // Remove loading attribute
        img.removeAttribute('loading');

        observer.unobserve(img);
      }
    });
  }, options);

  const images = document.querySelectorAll(selector);
  images.forEach((img) => observer.observe(img));

  return observer;
}

/**
 * Get image metadata without loading full image
 * @param {string} imagePath - Image path
 * @returns {Promise<Object>} Image metadata
 */
export async function getImageMetadata(imagePath) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    const startTime = performance.now();

    img.onload = () => {
      resolve({
        width: img.naturalWidth,
        height: img.naturalHeight,
        aspect: img.naturalWidth / img.naturalHeight,
        loadTime: performance.now() - startTime,
      });
    };

    img.onerror = () => {
      reject(new Error(`Failed to load image: ${imagePath}`));
    };

    img.src = imagePath;
  });
}

/**
 * Format bytes to human-readable size
 * @param {number} bytes - Size in bytes
 * @returns {string} Formatted size
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

export default {
  generateSrcset,
  getImageUrls,
  generateBlurHash,
  preloadImage,
  preloadImages,
  getSupportedFormats,
  supportsFormat,
  getThumbnail,
  getThumbnails,
  monitorImageLoad,
  calculateImageSize,
  toWebP,
  toAVIF,
  lazyLoadImages,
  getImageMetadata,
  formatFileSize,
};
