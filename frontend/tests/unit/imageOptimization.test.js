/**
 * Unit Tests for imageOptimization.js
 * Tests for responsive image utilities
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  generateSrcset,
  getImageUrls,
  getThumbnail,
  formatBytes,
  supportsFormat,
} from '../../src/lib/imageOptimization.js';

describe('imageOptimization.js', () => {
  describe('formatBytes', () => {
    it('should format bytes correctly', () => {
      expect(formatBytes(0)).toBe('0 Bytes');
      expect(formatBytes(1024)).toBe('1 KB');
      expect(formatBytes(1024 * 1024)).toBe('1 MB');
      expect(formatBytes(1024 * 1024 * 1024)).toBe('1 GB');
    });

    it('should handle decimal values', () => {
      expect(formatBytes(1536)).toBe('1.5 KB');
      expect(formatBytes(2621440)).toBe('2.5 MB');
    });
  });

  describe('generateSrcset', () => {
    it('should generate srcset string', () => {
      const srcset = generateSrcset('/images/photo.jpg', 'webp');
      expect(srcset).toContain('180w');
      expect(srcset).toContain('360w');
      expect(srcset).toContain(',');
    });

    it('should include correct format in CDN URL', () => {
      const srcset = generateSrcset('/images/photo.jpg', 'avif');
      expect(srcset).toContain('format=avif');
    });
  });

  describe('getImageUrls', () => {
    it('should return urls for all formats', () => {
      const urls = getImageUrls('/images/photo.jpg');
      expect(urls).toHaveProperty('avif');
      expect(urls).toHaveProperty('webp');
      expect(urls).toHaveProperty('original');
      expect(urls).toHaveProperty('width');
      expect(urls).toHaveProperty('height');
    });

    it('should respect custom dimensions', () => {
      const urls = getImageUrls('/images/photo.jpg', { width: 800, height: 600 });
      expect(urls.width).toBe(800);
      expect(urls.height).toBe(600);
    });
  });

  describe('getThumbnail', () => {
    it('should generate thumbnail URL', () => {
      const thumb = getThumbnail('/images/photo.jpg', 200);
      expect(thumb).toContain('width=200');
      expect(thumb).toContain('format=webp');
    });

    it('should use default size if not specified', () => {
      const thumb = getThumbnail('/images/photo.jpg');
      expect(thumb).toContain('width=200');
    });
  });

  describe('supportsFormat', () => {
    it('should return true for supported formats', async () => {
      const supported = await supportsFormat('image/webp');
      expect(typeof supported).toBe('boolean');
    });

    it('should always support standard formats', async () => {
      const jpeg = await supportsFormat('image/jpeg');
      expect(jpeg).toBe(true);
    });
  });
});
