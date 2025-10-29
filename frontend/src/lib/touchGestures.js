/**
 * Touch Gesture Handler for Mobile Devices
 * Provides long-press, swipe, and tap gesture detection
 */

export class TouchGestureHandler {
  constructor(element, options = {}) {
    this.element = element;
    this.options = {
      longPressDuration: options.longPressDuration || 500,
      swipeThreshold: options.swipeThreshold || 50,
      tapThreshold: options.tapThreshold || 10,
      ...options
    };

    this.touchStartX = 0;
    this.touchStartY = 0;
    this.touchStartTime = 0;
    this.longPressTimer = null;
    this.isSwiping = false;
    this.isLongPress = false;

    this.handlers = {
      tap: [],
      longPress: [],
      swipeLeft: [],
      swipeRight: [],
      swipeUp: [],
      swipeDown: []
    };

    this.init();
  }

  init() {
    this.element.addEventListener('touchstart', this.handleTouchStart.bind(this), { passive: false });
    this.element.addEventListener('touchmove', this.handleTouchMove.bind(this), { passive: false });
    this.element.addEventListener('touchend', this.handleTouchEnd.bind(this), { passive: false });
    this.element.addEventListener('touchcancel', this.handleTouchCancel.bind(this));
  }

  handleTouchStart(e) {
    const touch = e.touches[0];
    this.touchStartX = touch.clientX;
    this.touchStartY = touch.clientY;
    this.touchStartTime = Date.now();
    this.isSwiping = false;
    this.isLongPress = false;

    // Start long press timer
    this.longPressTimer = setTimeout(() => {
      this.isLongPress = true;
      this.trigger('longPress', {
        x: this.touchStartX,
        y: this.touchStartY,
        target: e.target,
        originalEvent: e
      });
    }, this.options.longPressDuration);
  }

  handleTouchMove(e) {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
      this.longPressTimer = null;
    }

    const touch = e.touches[0];
    const deltaX = touch.clientX - this.touchStartX;
    const deltaY = touch.clientY - this.touchStartY;
    const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

    if (distance > this.options.tapThreshold) {
      this.isSwiping = true;
    }
  }

  handleTouchEnd(e) {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
      this.longPressTimer = null;
    }

    if (this.isLongPress) {
      // Long press already triggered
      return;
    }

    const touch = e.changedTouches[0];
    const deltaX = touch.clientX - this.touchStartX;
    const deltaY = touch.clientY - this.touchStartY;
    const duration = Date.now() - this.touchStartTime;

    if (this.isSwiping) {
      // Detect swipe direction
      if (Math.abs(deltaX) > Math.abs(deltaY)) {
        // Horizontal swipe
        if (Math.abs(deltaX) > this.options.swipeThreshold) {
          if (deltaX > 0) {
            this.trigger('swipeRight', { deltaX, deltaY, duration, target: e.target, originalEvent: e });
          } else {
            this.trigger('swipeLeft', { deltaX, deltaY, duration, target: e.target, originalEvent: e });
          }
        }
      } else {
        // Vertical swipe
        if (Math.abs(deltaY) > this.options.swipeThreshold) {
          if (deltaY > 0) {
            this.trigger('swipeDown', { deltaX, deltaY, duration, target: e.target, originalEvent: e });
          } else {
            this.trigger('swipeUp', { deltaX, deltaY, duration, target: e.target, originalEvent: e });
          }
        }
      }
    } else {
      // Tap
      this.trigger('tap', {
        x: touch.clientX,
        y: touch.clientY,
        duration,
        target: e.target,
        originalEvent: e
      });
    }
  }

  handleTouchCancel() {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
      this.longPressTimer = null;
    }
  }

  on(eventType, handler) {
    if (this.handlers[eventType]) {
      this.handlers[eventType].push(handler);
    }
    return this;
  }

  off(eventType, handler) {
    if (this.handlers[eventType]) {
      this.handlers[eventType] = this.handlers[eventType].filter(h => h !== handler);
    }
    return this;
  }

  trigger(eventType, data) {
    if (this.handlers[eventType]) {
      this.handlers[eventType].forEach(handler => handler(data));
    }
  }

  destroy() {
    if (this.longPressTimer) {
      clearTimeout(this.longPressTimer);
    }
    this.element.removeEventListener('touchstart', this.handleTouchStart);
    this.element.removeEventListener('touchmove', this.handleTouchMove);
    this.element.removeEventListener('touchend', this.handleTouchEnd);
    this.element.removeEventListener('touchcancel', this.handleTouchCancel);
  }
}

/**
 * Svelte action for touch gestures
 * Usage: <div use:touchGesture={{ onLongPress: handleLongPress }}>
 */
export function touchGesture(node, options = {}) {
  const handler = new TouchGestureHandler(node, options);

  // Setup event listeners from options
  if (options.onTap) handler.on('tap', options.onTap);
  if (options.onLongPress) handler.on('longPress', options.onLongPress);
  if (options.onSwipeLeft) handler.on('swipeLeft', options.onSwipeLeft);
  if (options.onSwipeRight) handler.on('swipeRight', options.onSwipeRight);
  if (options.onSwipeUp) handler.on('swipeUp', options.onSwipeUp);
  if (options.onSwipeDown) handler.on('swipeDown', options.onSwipeDown);

  return {
    destroy() {
      handler.destroy();
    }
  };
}
