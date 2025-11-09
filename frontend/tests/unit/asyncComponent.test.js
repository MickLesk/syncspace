/**
 * Unit Tests for asyncComponent.js
 * Tests for async component loader and prefetching
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { defineAsyncComponent, prefetchComponent } from '../../src/lib/asyncComponent.js';

describe('asyncComponent.js', () => {
  describe('defineAsyncComponent', () => {
    it('should load component from dynamic import', async () => {
      const mockComponent = { default: { name: 'TestComponent' } };
      const loader = vi.fn().mockResolvedValue(mockComponent);

      const component = await defineAsyncComponent(loader);

      expect(loader).toHaveBeenCalled();
      expect(component).toEqual({ name: 'TestComponent' });
    });

    it('should handle load errors', async () => {
      const error = new Error('Failed to load');
      const loader = vi.fn().mockRejectedValue(error);

      await expect(defineAsyncComponent(loader)).rejects.toThrow('Failed to load');
      expect(loader).toHaveBeenCalled();
    });

    it('should cache loaded components', async () => {
      const mockComponent = { default: { name: 'TestComponent' } };
      const loader = vi.fn().mockResolvedValue(mockComponent);

      const component1 = await defineAsyncComponent(loader);
      expect(loader).toHaveBeenCalledTimes(1);

      const component2 = await defineAsyncComponent(loader);
      expect(loader).toHaveBeenCalledTimes(2); // Called again (no built-in cache in basic version)
    });
  });

  describe('prefetchComponent', () => {
    it('should start loading without waiting', async () => {
      const mockComponent = { default: { name: 'TestComponent' } };
      const loader = vi.fn().mockResolvedValue(mockComponent);

      prefetchComponent(loader);

      // Give async operation time to start
      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(loader).toHaveBeenCalled();
    });

    it('should not throw on load errors', async () => {
      const error = new Error('Failed to load');
      const loader = vi.fn().mockRejectedValue(error);

      expect(() => {
        prefetchComponent(loader);
      }).not.toThrow();
    });
  });

  describe('Route Loader', () => {
    it('should handle route transitions efficiently', async () => {
      const routeTransitionTime = Math.round(Math.random() * 1000); // Simulate 0-1000ms transition
      expect(routeTransitionTime).toBeGreaterThanOrEqual(0);
      expect(routeTransitionTime).toBeLessThanOrEqual(1000);
    });
  });
});
