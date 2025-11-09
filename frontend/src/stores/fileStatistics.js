import { writable, derived } from 'svelte/store';
import { t } from '$lib/i18n.js';

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
        // In production: const response = await api.files.getStatistics();
        const data = generateMockStatistics();
        set({
          fileTypeDistribution: data.fileTypeDistribution,
          fileAccessPatterns: data.fileAccessPatterns,
          sizeDistribution: data.sizeDistribution,
          accessHeatmap: data.accessHeatmap,
          mostAccessedFiles: data.mostAccessedFiles,
          isLoading: false,
          error: null,
          selectedMetric: 'byType',
        });
      } catch (error) {
        update(state => ({ ...state, error: error.message, isLoading: false }));
      }
    },

    // Select metric to display
    selectMetric(metric) {
      update(state => ({ ...state, selectedMetric: metric }));
    },
  };
}

/**
 * Generate mock statistics data
 */
function generateMockStatistics() {
  // File type distribution
  const fileTypeDistribution = {
    'Images': { count: 2150, size: 3.5 * 1024 * 1024 * 1024, color: '#3B82F6', percentage: 35 },
    'Videos': { count: 45, size: 4.2 * 1024 * 1024 * 1024, color: '#EF4444', percentage: 42 },
    'Documents': { count: 3420, size: 1.2 * 1024 * 1024 * 1024, color: '#F59E0B', percentage: 12 },
    'Archives': { count: 128, size: 800 * 1024 * 1024, color: '#10B981', percentage: 8 },
    'Audio': { count: 512, size: 300 * 1024 * 1024, color: '#8B5CF6', percentage: 3 },
    'Other': { count: 340, size: 180 * 1024 * 1024, color: '#6B7280', percentage: 0 },
  };

  // Size distribution (buckets)
  const sizeDistribution = [
    { bucket: '< 1MB', count: 2000, percentage: 25, files: 2000 },
    { bucket: '1-10MB', count: 1500, percentage: 19, files: 1500 },
    { bucket: '10-100MB', count: 1200, percentage: 15, files: 1200 },
    { bucket: '100MB-1GB', count: 1800, percentage: 23, files: 1800 },
    { bucket: '1-5GB', count: 800, percentage: 10, files: 800 },
    { bucket: '> 5GB', count: 400, percentage: 8, files: 400 },
  ];

  // File access patterns (per hour)
  const fileAccessPatterns = [];
  for (let hour = 0; hour < 24; hour++) {
    const baseAccess = 50 + Math.random() * 100;
    const peakHours = (hour >= 8 && hour <= 10) || (hour >= 14 && hour <= 16);
    fileAccessPatterns.push({
      hour: `${String(hour).padStart(2, '0')}:00`,
      accesses: Math.round(peakHours ? baseAccess * 2 : baseAccess),
    });
  }

  // Access heatmap (day of week -> hour -> count)
  const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
  const accessHeatmap = {};
  days.forEach(day => {
    accessHeatmap[day] = {};
    for (let hour = 0; hour < 24; hour++) {
      const isWeekend = day === 'Sat' || day === 'Sun';
      const isBusinessHours = hour >= 8 && hour <= 18;
      const base = isWeekend ? 10 : isBusinessHours ? 50 : 20;
      accessHeatmap[day][hour] = Math.round(base + Math.random() * 30);
    }
  });

  // Most accessed files
  const mostAccessedFiles = [
    { name: 'frequently_used_document.pdf', accesses: 1250, lastAccess: '2 hours ago' },
    { name: 'project_presentation.pptx', accesses: 980, lastAccess: '5 hours ago' },
    { name: 'team_photo_2024.jpg', accesses: 750, lastAccess: 'Yesterday' },
    { name: 'financial_report.xlsx', accesses: 650, lastAccess: '2 days ago' },
    { name: 'video_tutorial.mp4', accesses: 580, lastAccess: '3 days ago' },
  ];

  return {
    fileTypeDistribution,
    fileAccessPatterns,
    sizeDistribution,
    accessHeatmap,
    mostAccessedFiles,
  };
}

export const fileStatistics = createStatisticsStore();
