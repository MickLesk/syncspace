/**
 * Vitest Setup File
 * Global test configuration and mocks
 */

import { vi } from 'vitest';

// Mock fetch
global.fetch = vi.fn();

// Mock localStorage
const localStorageMock = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
};

global.localStorage = localStorageMock;

// Mock sessionStorage
global.sessionStorage = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
};

// Mock IntersectionObserver
global.IntersectionObserver = class IntersectionObserver {
  constructor() {}
  disconnect() {}
  observe() {}
  takeRecords() {}
  unobserve() {}
};

// Mock PerformanceObserver
global.PerformanceObserver = class PerformanceObserver {
  constructor() {}
  disconnect() {}
  observe() {}
  takeRecords() {}
};

// Mock navigator
Object.defineProperty(global.navigator, 'serviceWorker', {
  value: { register: vi.fn() },
  writable: true,
});

Object.defineProperty(global.navigator, 'onLine', {
  value: true,
  writable: true,
});
