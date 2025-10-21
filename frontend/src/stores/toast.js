import { writable } from 'svelte/store';

// Toast store
export const toasts = writable([]);

let toastId = 0;

export function showToast(message, type = 'info', duration = 3000) {
  const id = toastId++;
  const toast = { id, message, type, duration };
  
  toasts.update(t => [...t, toast]);
  
  if (duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, duration);
  }
  
  return id;
}

export function removeToast(id) {
  toasts.update(t => t.filter(toast => toast.id !== id));
}

export function success(message, duration = 3000) {
  return showToast(message, 'success', duration);
}

export function error(message, duration = 4000) {
  return showToast(message, 'error', duration);
}

export function info(message, duration = 3000) {
  return showToast(message, 'info', duration);
}

export function warning(message, duration = 3000) {
  return showToast(message, 'warning', duration);
}
