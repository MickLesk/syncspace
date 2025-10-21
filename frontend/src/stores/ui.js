import { writable } from 'svelte/store';

export const currentView = writable('files');
export const currentTheme = writable(localStorage.getItem('theme') || 'light');
export const currentLang = writable(localStorage.getItem('lang') || 'de');
export const files = writable([]);
export const currentPath = writable('');
