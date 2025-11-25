import { writable, derived } from 'svelte/store';
import { t } from '$lib/i18n.js';
import api from '$lib/api.js';

/**
 * Activity Feed Store
 * Manages user activity log with filtering, searching, and statistics
 */

const ACTIVITY_TYPES = {
  uploaded: { label: 'activity.uploaded', icon: 'bi-upload', color: '#3B82F6' },
  downloaded: { label: 'activity.downloaded', icon: 'bi-download', color: '#10B981' },
  deleted: { label: 'activity.deleted', icon: 'bi-trash', color: '#EF4444' },
  renamed: { label: 'activity.renamed', icon: 'bi-pencil', color: '#F59E0B' },
  moved: { label: 'activity.moved', icon: 'bi-arrow-right', color: '#8B5CF6' },
  created: { label: 'activity.created', icon: 'bi-plus-circle', color: '#06B6D4' },
  shared: { label: 'activity.shared', icon: 'bi-share', color: '#EC4899' },
  commented: { label: 'activity.commented', icon: 'bi-chat-left', color: '#14B8A6' },
};

function createActivityStore() {
  const { subscribe, set, update } = writable({
    activities: [],
    filtered: [],
    selectedFilter: 'all',
    searchQuery: '',
    dateRange: { start: null, end: null },
    isLoading: false,
    error: null,
    stats: {
      totalActivities: 0,
      today: 0,
      thisWeek: 0,
      thisMonth: 0,
      byType: {},
    },
  });

  return {
    subscribe,

    // Load activities from API
    async loadActivities() {
      update(state => ({ ...state, isLoading: true, error: null }));
      try {
        const response = await api.activity?.getActivities?.();
        if (!response) {
          throw new Error('Activity API not available');
        }
        const stats = calculateStats(response);
        set({
          activities: response,
          filtered: response,
          selectedFilter: 'all',
          searchQuery: '',
          dateRange: { start: null, end: null },
          isLoading: false,
          error: null,
          stats,
        });
      } catch (error) {
        console.error('Failed to load activities:', error);
        update(state => ({ 
          ...state, 
          error: error.message || 'Failed to load activities', 
          isLoading: false 
        }));
      }
    },

    // Filter activities by type
    filterByType(type) {
      update(state => {
        const filtered = type === 'all' 
          ? state.activities 
          : state.activities.filter(a => a.type === type);
        return { ...state, selectedFilter: type, filtered };
      });
    },

    // Search activities
    search(query) {
      update(state => {
        const filtered = query.trim() === ''
          ? state.activities
          : state.activities.filter(a =>
              a.filename.toLowerCase().includes(query.toLowerCase()) ||
              a.filePath.toLowerCase().includes(query.toLowerCase()) ||
              a.user.toLowerCase().includes(query.toLowerCase())
            );
        return { ...state, searchQuery: query, filtered };
      });
    },

    // Filter by date range
    filterByDateRange(start, end) {
      update(state => {
        const filtered = state.activities.filter(a => {
          const date = new Date(a.timestamp);
          const isAfterStart = !start || date >= start;
          const isBeforeEnd = !end || date <= end;
          return isAfterStart && isBeforeEnd;
        });
        return { ...state, dateRange: { start, end }, filtered };
      });
    },

    // Clear all filters
    clearFilters() {
      update(state => ({
        ...state,
        selectedFilter: 'all',
        searchQuery: '',
        dateRange: { start: null, end: null },
        filtered: state.activities,
      }));
    },

    // Format timestamp for display
    formatTime(timestamp) {
      const now = new Date();
      const then = new Date(timestamp);
      const diffMs = now.getTime() - then.getTime();
      const diffMins = Math.floor(diffMs / 60000);
      const diffHours = Math.floor(diffMs / 3600000);
      const diffDays = Math.floor(diffMs / 86400000);

      if (diffMins < 1) return t('activity.just_now');
      if (diffMins < 60) return `${diffMins}m ${t('activity.ago')}`;
      if (diffHours < 24) return `${diffHours}h ${t('activity.ago')}`;
      if (diffDays < 7) return `${diffDays}d ${t('activity.ago')}`;
      return then.toLocaleDateString();
    },

    // Export activities
    export() {
      let state = {};
      const unsubscribe = subscribe(s => {
        state = s;
      });
      unsubscribe();

      const json = JSON.stringify(state.filtered, null, 2);
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `activity-${new Date().getTime()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    },
  };
}

/**
 * Calculate activity statistics
 */
function calculateStats(activities) {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const weekAgo = new Date(today);
  weekAgo.setDate(weekAgo.getDate() - 7);
  const monthAgo = new Date(today);
  monthAgo.setMonth(monthAgo.getMonth() - 1);

  const stats = {
    totalActivities: activities.length,
    today: activities.filter(a => new Date(a.timestamp) >= today).length,
    thisWeek: activities.filter(a => new Date(a.timestamp) >= weekAgo).length,
    thisMonth: activities.filter(a => new Date(a.timestamp) >= monthAgo).length,
    byType: {},
  };

  // Count by type
  activities.forEach(a => {
    stats.byType[a.type] = (stats.byType[a.type] || 0) + 1;
  });

  return stats;
}

  return stats;
}

export const activityFeed = createActivityStore();
export { ACTIVITY_TYPES };
