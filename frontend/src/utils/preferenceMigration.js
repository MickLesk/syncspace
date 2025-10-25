/**
 * Example: Migrating from localStorage to Backend Preferences
 * 
 * BEFORE (localStorage - BAD for multi-platform):
 * localStorage.setItem('viewMode', 'grid');
 * const viewMode = localStorage.getItem('viewMode') || 'grid';
 * 
 * AFTER (Backend preferences - GOOD for multi-platform):
 */

import { userPreferences, preferencesHelpers } from '../stores/preferences.js';

export class PreferenceMigrationExample {
  
  // Initialize preferences on app start
  static async initializePreferences() {
    try {
      // Load user preferences from backend
      await userPreferences.load();
      console.log('✅ Preferences loaded from backend');
    } catch (error) {
      console.warn('⚠️ Failed to load preferences, using defaults:', error);
    }
  }
  
  // Example: Setting view mode (Grid/List)
  static async setViewMode(mode) {
    try {
      // OLD WAY: localStorage.setItem('viewMode', mode);
      // NEW WAY: Backend API call
      await preferencesHelpers.setViewMode(mode);
      console.log(`✅ View mode updated to: ${mode}`);
    } catch (error) {
      console.error('❌ Failed to update view mode:', error);
    }
  }
  
  // Example: Adding recent search
  static async addSearch(query) {
    try {
      // OLD WAY: 
      // const searches = JSON.parse(localStorage.getItem('recentSearches') || '[]');
      // searches.unshift(query);
      // localStorage.setItem('recentSearches', JSON.stringify(searches.slice(0, 10)));
      
      // NEW WAY: Backend API call
      await userPreferences.addRecentSearch(query);
      console.log(`✅ Added search to recent: ${query}`);
    } catch (error) {
      console.error('❌ Failed to add recent search:', error);
    }
  }
  
  // Example: Toggle sidebar
  static async toggleSidebar() {
    try {
      // OLD WAY: localStorage.setItem('sidebarCollapsed', !isCollapsed);
      // NEW WAY: Backend API call
      await preferencesHelpers.toggleSidebar();
      console.log('✅ Sidebar state toggled');
    } catch (error) {
      console.error('❌ Failed to toggle sidebar:', error);
    }
  }
  
  // Example: Using preferences in Svelte component
  static getComponentUsageExample() {
    return `
<script>
  import { userPreferences } from '../stores/preferences.js';
  import { onMount } from 'svelte';
  
  let viewMode = 'grid';
  let sidebarCollapsed = false;
  
  // Subscribe to preferences
  $: ({ view_mode: viewMode, sidebar_collapsed: sidebarCollapsed } = $userPreferences);
  
  onMount(async () => {
    // Load preferences on component mount
    await userPreferences.load();
  });
  
  async function handleViewChange(newMode) {
    // Update via backend (automatically updates store)
    await userPreferences.updatePreference('view_mode', newMode);
  }
</script>

<div class="file-view" class:list-mode={viewMode === 'list'}>
  <button on:click={() => handleViewChange('grid')}>Grid View</button>
  <button on:click={() => handleViewChange('list')}>List View</button>
</div>
`;
  }
}

// Migration helper for existing localStorage data
export class LocalStorageMigration {
  
  // One-time migration from localStorage to backend
  static async migrateExistingData() {
    try {
      const migrations = [];
      
      // Migrate view mode
      const viewMode = localStorage.getItem('viewMode');
      if (viewMode) {
        migrations.push({ view_mode: viewMode });
        localStorage.removeItem('viewMode');
      }
      
      // Migrate recent searches
      const recentSearches = localStorage.getItem('recentSearches');
      if (recentSearches) {
        try {
          const searches = JSON.parse(recentSearches);
          migrations.push({ recent_searches: searches });
          localStorage.removeItem('recentSearches');
        } catch (e) {
          console.warn('Failed to parse recent searches from localStorage');
        }
      }
      
      // Migrate sidebar state
      const sidebarCollapsed = localStorage.getItem('sidebarCollapsed');
      if (sidebarCollapsed !== null) {
        migrations.push({ sidebar_collapsed: sidebarCollapsed === 'true' });
        localStorage.removeItem('sidebarCollapsed');
      }
      
      // Apply all migrations at once
      if (migrations.length > 0) {
        const mergedMigrations = Object.assign({}, ...migrations);
        await userPreferences.updatePreferences(mergedMigrations);
        console.log('✅ Successfully migrated localStorage data to backend');
      }
      
    } catch (error) {
      console.error('❌ Failed to migrate localStorage data:', error);
    }
  }
}