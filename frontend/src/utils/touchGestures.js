/**
 * Touch gesture handler for mobile interactions
 */

export class TouchGestureHandler {
  constructor() {
    this.touchStartX = 0;
    this.touchStartY = 0;
    this.touchEndX = 0;
    this.touchEndY = 0;
    this.minSwipeDistance = 50;
    this.tapTimeout = null;
    this.lastTap = 0;
    this.doubleTapDelay = 300;
  }

  /**
   * Handle touch start
   */
  handleTouchStart(e, callback) {
    this.touchStartX = e.changedTouches[0].screenX;
    this.touchStartY = e.changedTouches[0].screenY;

    if (callback) {
      callback({ x: this.touchStartX, y: this.touchStartY });
    }
  }

  /**
   * Handle touch end
   */
  handleTouchEnd(e, callbacks = {}) {
    this.touchEndX = e.changedTouches[0].screenX;
    this.touchEndY = e.changedTouches[0].screenY;

    const deltaX = this.touchEndX - this.touchStartX;
    const deltaY = this.touchEndY - this.touchStartY;

    // Check for swipe gestures
    if (Math.abs(deltaX) > this.minSwipeDistance || Math.abs(deltaY) > this.minSwipeDistance) {
      this.handleSwipe(deltaX, deltaY, callbacks);
    } else {
      this.handleTap(e, callbacks);
    }
  }

  /**
   * Handle swipe gesture
   */
  handleSwipe(deltaX, deltaY, callbacks) {
    if (Math.abs(deltaX) > Math.abs(deltaY)) {
      // Horizontal swipe
      if (deltaX > 0 && callbacks.onSwipeRight) {
        callbacks.onSwipeRight(deltaX);
      } else if (deltaX < 0 && callbacks.onSwipeLeft) {
        callbacks.onSwipeLeft(Math.abs(deltaX));
      }
    } else {
      // Vertical swipe
      if (deltaY > 0 && callbacks.onSwipeDown) {
        callbacks.onSwipeDown(deltaY);
      } else if (deltaY < 0 && callbacks.onSwipeUp) {
        callbacks.onSwipeUp(Math.abs(deltaY));
      }
    }
  }

  /**
   * Handle tap gesture
   */
  handleTap(e, callbacks) {
    const now = Date.now();
    const timeSinceLastTap = now - this.lastTap;

    if (timeSinceLastTap < this.doubleTapDelay && timeSinceLastTap > 0) {
      // Double tap
      if (callbacks.onDoubleTap) {
        callbacks.onDoubleTap(e);
      }
      this.lastTap = 0;
    } else {
      // Single tap
      this.lastTap = now;
      if (this.tapTimeout) {
        clearTimeout(this.tapTimeout);
      }
      
      this.tapTimeout = setTimeout(() => {
        if (callbacks.onTap) {
          callbacks.onTap(e);
        }
        this.tapTimeout = null;
      }, this.doubleTapDelay);
    }
  }

  /**
   * Handle long press
   */
  handleLongPress(e, callback, duration = 500) {
    let pressTimer;

    const touchStart = (evt) => {
      pressTimer = setTimeout(() => {
        if (callback) {
          callback(evt);
        }
      }, duration);
    };

    const touchEnd = () => {
      clearTimeout(pressTimer);
    };

    e.addEventListener('touchstart', touchStart);
    e.addEventListener('touchend', touchEnd);
    e.addEventListener('touchmove', touchEnd);

    return () => {
      e.removeEventListener('touchstart', touchStart);
      e.removeEventListener('touchend', touchEnd);
      e.removeEventListener('touchmove', touchEnd);
    };
  }

  /**
   * Detect pinch zoom gesture
   */
  handlePinch(e, callback) {
    if (e.touches.length === 2) {
      const touch1 = e.touches[0];
      const touch2 = e.touches[1];

      const distance = Math.hypot(
        touch2.clientX - touch1.clientX,
        touch2.clientY - touch1.clientY
      );

      if (!this.initialPinchDistance) {
        this.initialPinchDistance = distance;
      } else {
        const scale = distance / this.initialPinchDistance;
        if (callback) {
          callback(scale);
        }
      }
    } else {
      this.initialPinchDistance = null;
    }
  }

  /**
   * Check if device is mobile
   */
  static isMobile() {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
      navigator.userAgent
    );
  }

  /**
   * Check if device is tablet
   */
  static isTablet() {
    return /iPad|Android/i.test(navigator.userAgent) && !TouchGestureHandler.isMobile();
  }

  /**
   * Get device type
   */
  static getDeviceType() {
    if (TouchGestureHandler.isMobile()) return 'mobile';
    if (TouchGestureHandler.isTablet()) return 'tablet';
    return 'desktop';
  }
}

export const touchGestures = new TouchGestureHandler();
