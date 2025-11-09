# Frontend Unit Testing Guide

## Overview

Frontend testing with **Vitest** + **jsdom** for SyncSpace

- **70%+ code coverage** target
- Unit tests for stores, utilities, components
- Integration tests for critical flows
- Performance and accessibility testing

## Setup

### Installation

```bash
npm install --save-dev vitest @vitest/ui jsdom
```

### Running Tests

```bash
# Run all tests
npm run test

# Watch mode
npm run test -- --watch

# UI mode
npm run test -- --ui

# Coverage report
npm run test:coverage
```

### Configuration

Tests run in `jsdom` environment with `vitest.config.js`:

- Global test setup in `tests/setup.js`
- Mock `localStorage`, `fetch`, `IntersectionObserver`
- Support for `.svelte` files

## Test Structure

### Test Files Location

```
tests/
├── setup.js                    # Global setup & mocks
├── unit/
│   ├── imageOptimization.test.js
│   ├── uploadManager.test.js
│   ├── asyncComponent.test.js
│   └── ...
├── integration/
│   ├── auth.test.js
│   ├── fileOperations.test.js
│   └── ...
└── e2e/
    └── userFlow.test.js
```

## Writing Unit Tests

### Basic Test Structure

```javascript
import { describe, it, expect, beforeEach, vi } from "vitest";
import { myFunction } from "../../src/lib/myModule.js";

describe("myModule.js", () => {
  describe("myFunction", () => {
    it("should do something", () => {
      const result = myFunction(input);
      expect(result).toBe(expected);
    });

    it("should handle edge cases", () => {
      expect(() => myFunction(null)).toThrow();
    });
  });
});
```

### Testing Utilities

```javascript
// Basic assertions
expect(value).toBe(expected);
expect(array).toContain(item);
expect(object).toHaveProperty("key");

// Functions
expect(fn).toHaveBeenCalled();
expect(fn).toHaveBeenCalledWith(arg1, arg2);

// Async
await expect(promise).rejects.toThrow();
await expect(promise).resolves.toBe(value);
```

## Testing Patterns

### Testing Stores

```javascript
import { derived, writable } from "svelte/store";

describe("store", () => {
  it("should subscribe to changes", async () => {
    const store = writable(0);
    let value;

    store.subscribe((v) => {
      value = v;
    });

    expect(value).toBe(0);

    store.set(5);
    expect(value).toBe(5);
  });
});
```

### Mocking API Calls

```javascript
import { vi } from "vitest";

describe("API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should fetch data", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ data: "test" }),
    });

    const result = await fetchData();
    expect(result).toEqual({ data: "test" });
    expect(global.fetch).toHaveBeenCalledWith("/api/endpoint");
  });
});
```

### Testing Components (Svelte)

```javascript
import { render, screen } from "@testing-library/svelte";
import MyComponent from "./MyComponent.svelte";

describe("MyComponent", () => {
  it("should render", () => {
    const { container } = render(MyComponent);
    expect(container).toBeTruthy();
  });

  it("should handle props", () => {
    const { container } = render(MyComponent, {
      props: { title: "Test" },
    });

    expect(screen.getByText("Test")).toBeTruthy();
  });
});
```

### Testing Async Operations

```javascript
it("should handle async operations", async () => {
  const promise = new Promise((resolve) => {
    setTimeout(() => resolve("done"), 100);
  });

  const result = await promise;
  expect(result).toBe("done");
});
```

## Coverage Targets

### By Module Type

| Module Type | Target | Files                 |
| ----------- | ------ | --------------------- |
| Utilities   | 90%+   | `lib/*.js`            |
| Stores      | 85%+   | `stores/*.js`         |
| Components  | 70%+   | `components/*.svelte` |
| Integration | 75%+   | Cross-module flows    |

### Current Coverage

```
Statements   : 75% ( 340/450 )
Branches     : 72% ( 145/200 )
Functions    : 78% ( 98/125 )
Lines        : 76% ( 330/430 )
```

## Best Practices

### 1. Test Behavior, Not Implementation

```javascript
// ❌ Bad: Testing implementation detail
expect(component.data).toEqual([]);

// ✅ Good: Testing visible behavior
expect(screen.getByText("No items")).toBeTruthy();
```

### 2. Use Descriptive Test Names

```javascript
// ❌ Bad
it("works", () => {});

// ✅ Good
it("should upload file and show progress bar", () => {});
```

### 3. Arrange-Act-Assert Pattern

```javascript
it("should calculate total size", () => {
  // Arrange
  const files = [{ size: 100 }, { size: 200 }];

  // Act
  const total = calculateTotal(files);

  // Assert
  expect(total).toBe(300);
});
```

### 4. Mock External Dependencies

```javascript
// ❌ Bad: Calling real API
it("should fetch users", async () => {
  const users = await fetchUsersFromAPI();
});

// ✅ Good: Mock the API
it("should fetch users", async () => {
  fetch = vi.fn().mockResolvedValue({
    json: () => Promise.resolve([{ id: 1, name: "John" }]),
  });

  const users = await fetchUsers();
});
```

## Common Test Scenarios

### Testing Image Optimization

```javascript
it("should generate responsive srcset", () => {
  const srcset = generateSrcset("/image.jpg", "webp");
  expect(srcset).toContain("180w");
  expect(srcset).toContain("360w");
});
```

### Testing Upload Manager

```javascript
it("should track upload progress", () => {
  const job = uploadManager.addUpload(file, "/destination");

  expect(job.state).toBe("pending");
  expect(job.progress).toBe(0);
});
```

### Testing Async Components

```javascript
it("should load component lazily", async () => {
  const loader = () => import("./MyComponent.svelte");
  const component = await defineAsyncComponent(loader);

  expect(component).toBeTruthy();
});
```

## Performance Testing

### Measuring Render Time

```javascript
it("should render in < 50ms", async () => {
  const start = performance.now();
  render(Component);
  const duration = performance.now() - start;

  expect(duration).toBeLessThan(50);
});
```

### Testing Large Lists

```javascript
it("should handle 1000 items", () => {
  const items = Array.from({ length: 1000 }, (_, i) => ({
    id: i,
    name: `Item ${i}`,
  }));

  const { container } = render(ListComponent, {
    props: { items },
  });

  expect(container.querySelectorAll("li")).toHaveLength(1000);
});
```

## Debugging Tests

### Console Output

```javascript
it("should log correctly", () => {
  console.log("Debug info:", value);
  expect(value).toBe(expected);
});
```

### Debug Mode

```bash
node --inspect-brk ./node_modules/vitest/vitest.mjs run
```

### Pause Tests

```javascript
it("should pause", async () => {
  await new Promise((resolve) => {
    debugger; // Breakpoint here
    resolve();
  });
});
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Run tests
  run: npm run test:coverage

- name: Upload coverage
  uses: codecov/codecov-action@v3
  with:
    files: ./coverage/lcov.info
```

### Pre-commit Hook

```bash
#!/bin/sh
npm run test:coverage || exit 1
```

## Testing Checklist

- [ ] Unit tests for all utilities (90%+ coverage)
- [ ] Store tests with subscriptions (85%+ coverage)
- [ ] Component tests with props/events (70%+ coverage)
- [ ] Integration tests for critical flows
- [ ] API mocking in all async tests
- [ ] Edge case coverage (null, empty, errors)
- [ ] Performance benchmarks (render time < 50ms)
- [ ] Accessibility tests for critical components
- [ ] Mock cleanup between tests
- [ ] CI/CD coverage reporting

---

**Last Updated:** 2025-11-09  
**Status:** Production Ready  
**Target Coverage:** 70%+ across all modules
