import { writable, derived } from 'svelte/store';
import { t } from '$lib/i18n.js';
import api from '$lib/api.js';

/**
 * File Statistics Store
 * Manages file type distribution, size heatmap, access patterns
 */

function createStatisticsStore() {
  const { subscribe, set, update } = writable({
    fileTypeDistribution: {},
    fileAccessPatterns: [],
    sizeDistribution: [],
    accessHeatmap: {}, // day -> hour -> count
    mostAccessedFiles: [],
    isLoading: false,
    error: null,
    selectedMetric: 'byType', // byType, bySize, byAccess
  });

  return {
    subscribe,

    // Load statistics
    async loadStatistics() {
      update(state => ({ ...state, isLoading: true, error: null }));
      try {
        const response = await api.files?.getStatistics?.();
        if (!response) {
          throw new Error('File statistics API not available');
        }
        set({
          fileTypeDistribution: response.fileTypeDistribution || {},
          fileAccessPatterns: response.fileAccessPatterns || [],
          sizeDistribution: response.sizeDistribution || [],
          accessHeatmap: response.accessHeatmap || {},
          mostAccessedFiles: response.mostAccessedFiles || [],
          isLoading: false,
          error: null,
          selectedMetric: 'byType',
        });
      } catch (error) {
        console.error('Failed to load file statistics:', error);
        update(state => ({ 
          ...state, 
          error: error.message || 'Failed to load statistics', 
          isLoading: false 
        }));
      }
    },

    // Select metric to display
    selectMetric(metric) {
      update(state => ({ ...state, selectedMetric: metric }));
    },
  };
}

export const fileStatistics = createStatisticsStore();
