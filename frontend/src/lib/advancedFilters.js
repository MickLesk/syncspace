/**
 * Advanced Search Filters
 * Comprehensive filtering system for files with:
 * - Date range, file type, size, owner, tags
 * - Preset management
 * - Real-time filtering
 */

import { writable, derived } from 'svelte/store';
import { api } from './api';
import { t } from './i18n';
import { toast } from '../stores/ui';

  /**
   * Create advanced filters store
   */
  export function createAdvancedFiltersStore() {
    // Filter state
    const filters = writable({
      query: '',
      fileType: null,
      dateRange: { from: null, to: null },
      sizeRange: { min: null, max: null }, // in bytes
      owner: null,
      tags: [],
      createdAfter: null,
      createdBefore: null,
      modifiedAfter: null,
      modifiedBefore: null,
      shared: null, // null, true, false
      isFavorite: null,
      isArchived: null,
      custom: {} // extensible
    });

    // Presets (saved filter combinations)
    const presets = writable([]);

    // Current preset
    const currentPreset = writable(null);

    // Active filter count
    const activeFilterCount = derived(filters, $filters => {
      let count = 0;
      if ($filters.query) count++;
      if ($filters.fileType) count++;
      if ($filters.dateRange.from || $filters.dateRange.to) count++;
      if ($filters.sizeRange.min || $filters.sizeRange.max) count++;
      if ($filters.owner) count++;
      if ($filters.tags.length > 0) count++;
      if ($filters.shared !== null) count++;
      if ($filters.isFavorite !== null) count++;
      if ($filters.isArchived !== null) count++;
      return count;
    });

    /**
     * Update single filter
     */
    function updateFilter(key, value) {
      filters.update(f => ({
        ...f,
        [key]: value
      }));
    }

    /**
     * Update nested filter (e.g., dateRange.from)
     */
    function updateNestedFilter(path, value) {
      filters.update(f => {
        const keys = path.split('.');
        const target = keys.slice(0, -1).reduce((obj, k) => obj[k], f);
        target[keys[keys.length - 1]] = value;
        return f;
      });
    }

    /**
     * Clear all filters
     */
    function clearFilters() {
      filters.set({
        query: '',
        fileType: null,
        dateRange: { from: null, to: null },
        sizeRange: { min: null, max: null },
        owner: null,
        tags: [],
        createdAfter: null,
        createdBefore: null,
        modifiedAfter: null,
        modifiedBefore: null,
        shared: null,
        isFavorite: null,
        isArchived: null,
        custom: {}
      });
      currentPreset.set(null);
    }

    /**
     * Save current filters as preset
     */
    async function savePreset(name) {
      try {
        const currentFilters = {};
        filters.subscribe(f => Object.assign(currentFilters, f))();

        const preset = {
          id: `preset_${Date.now()}`,
          name,
          filters: currentFilters,
          createdAt: new Date().toISOString()
        };

        presets.update(p => [...p, preset]);
        toast.show(t('filters.presetSaved', { name }), 'success');
        return preset;
      } catch (error) {
        console.error('Error saving preset:', error);
        toast.show(t('errors.failedToSavePreset'), 'error');
      }
    }

    /**
     * Load preset
     */
    function loadPreset(presetId) {
      presets.subscribe(p => {
        const preset = p.find(pr => pr.id === presetId);
        if (preset) {
          filters.set(preset.filters);
          currentPreset.set(presetId);
          toast.show(t('filters.presetLoaded', { name: preset.name }), 'success');
        }
      })();
    }

    /**
     * Delete preset
     */
    function deletePreset(presetId) {
      presets.update(p => p.filter(pr => pr.id !== presetId));
      if (presets.subscribe(p => p)()[0].find(pr => pr.id === presetId) === undefined) {
        currentPreset.set(null);
      }
    }

    /**
     * Apply filters to file list
     */
    function applyFilters(files) {
      return files.reduce((filtered, file) => {
        let $filters, $tags = [];
        filters.subscribe(f => $filters = f)();

        // Query/Search
        if ($filters.query && !file.name.toLowerCase().includes($filters.query.toLowerCase())) {
          return filtered;
        }

        // File Type
        if ($filters.fileType) {
          const ext = file.name.split('.').pop().toLowerCase();
          if (ext !== $filters.fileType && file.mimeType?.split('/')[0] !== $filters.fileType) {
            return filtered;
          }
        }

        // Size Range
        if ($filters.sizeRange.min && file.size < $filters.sizeRange.min) {
          return filtered;
        }
        if ($filters.sizeRange.max && file.size > $filters.sizeRange.max) {
          return filtered;
        }

        // Owner
        if ($filters.owner && file.ownerId !== $filters.owner) {
          return filtered;
        }

        // Tags
        if ($filters.tags.length > 0) {
          const fileTags = file.tags?.map(t => t.id) || [];
          const hasAllTags = $filters.tags.every(tag => fileTags.includes(tag));
          if (!hasAllTags) {
            return filtered;
          }
        }

        // Shared Status
        if ($filters.shared !== null) {
          const isShared = !!file.shares?.length;
          if (isShared !== $filters.shared) {
            return filtered;
          }
        }

        // Favorite
        if ($filters.isFavorite !== null && file.isFavorite !== $filters.isFavorite) {
          return filtered;
        }

        // Date Range
        if ($filters.dateRange.from || $filters.dateRange.to) {
          const fileDate = new Date(file.createdAt);
          if ($filters.dateRange.from && fileDate < $filters.dateRange.from) {
            return filtered;
          }
          if ($filters.dateRange.to && fileDate > $filters.dateRange.to) {
            return filtered;
          }
        }

        filtered.push(file);
        return filtered;
      }, []);
    }

    /**
     * Get filter summary text
     */
    function getFilterSummary() {
      let $filters, count = 0;
      filters.subscribe(f => $filters = f)();
      activeFilterCount.subscribe(c => count = c)();

      if (count === 0) return null;

      const parts = [];
      if ($filters.query) parts.push($filters.query);
      if ($filters.fileType) parts.push(t('filters.fileType', { type: $filters.fileType }));
      if ($filters.owner) parts.push(t('filters.owner'));
      if ($filters.tags.length > 0) parts.push(t('filters.tags', { count: $filters.tags.length }));
      if ($filters.shared !== null) parts.push($filters.shared ? t('filters.shared') : t('filters.notShared'));

      return parts.join(', ');
    }

    return {
      filters,
      presets,
      currentPreset,
      activeFilterCount,
      updateFilter,
      updateNestedFilter,
      clearFilters,
      savePreset,
      loadPreset,
      deletePreset,
      applyFilters,
      getFilterSummary
    };
  }

  // Export singleton instance
  export const advancedFilters = createAdvancedFiltersStore();
