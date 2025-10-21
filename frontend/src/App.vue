<template>
  <div class="min-h-screen bg-gradient-to-br from-primary to-secondary">
    <!-- Header -->
    <header class="bg-white shadow-sm border-b border-gray-100">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center">
            <span class="text-white font-bold text-lg">S</span>
          </div>
          <h1 class="text-2xl font-bold text-gray-900">SyncSpace</h1>
        </div>
        <button @click="logout" class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
          Abmelden
        </button>
      </div>
    </header>

    <!-- Main Content -->
    <div class="flex h-screen overflow-hidden">
      <!-- Sidebar -->
      <nav class="w-64 bg-white shadow-lg overflow-y-auto">
        <div class="p-4 space-y-2">
          <a 
            @click="currentView = 'files'" 
            :class="['nav-item', currentView === 'files' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            📁 Dateien
          </a>
          <a 
            @click="currentView = 'favorites'" 
            :class="['nav-item', currentView === 'favorites' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            ⭐ Favoriten
          </a>
          <a 
            @click="currentView = 'search'" 
            :class="['nav-item', currentView === 'search' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            🔍 Suche
          </a>
          <a 
            @click="currentView = 'activity'" 
            :class="['nav-item', currentView === 'activity' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            📊 Aktivität
          </a>
          <a 
            @click="currentView = 'trash'" 
            :class="['nav-item', currentView === 'trash' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            🗑️ Papierkorb
          </a>
          <a 
            @click="currentView = 'sync'" 
            :class="['nav-item', currentView === 'sync' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            🔄 Synchronisation
          </a>
          <a 
            @click="currentView = 'settings'" 
            :class="['nav-item', currentView === 'settings' && 'active']"
            class="block px-4 py-3 rounded-lg cursor-pointer transition hover:bg-gray-100"
          >
            ⚙️ Einstellungen
          </a>
        </div>
      </nav>

      <!-- Main Content Area -->
      <main class="flex-1 overflow-auto bg-surface">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <!-- Files View -->
          <FilesView v-if="currentView === 'files'" />
          
          <!-- Favorites View -->
          <FavoritesView v-else-if="currentView === 'favorites'" />
          
          <!-- Search View -->
          <SearchView v-else-if="currentView === 'search'" />
          
          <!-- Activity View -->
          <ActivityView v-else-if="currentView === 'activity'" />
          
          <!-- Trash View -->
          <TrashView v-else-if="currentView === 'trash'" />
          
          <!-- Sync View -->
          <SyncView v-else-if="currentView === 'sync'" />
          
          <!-- Settings View -->
          <SettingsView v-else-if="currentView === 'settings'" />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import FilesView from './views/FilesView.vue'
import FavoritesView from './views/FavoritesView.vue'
import SearchView from './views/SearchView.vue'
import ActivityView from './views/ActivityView.vue'
import TrashView from './views/TrashView.vue'
import SyncView from './views/SyncView.vue'
import SettingsView from './views/SettingsView.vue'

const currentView = ref('files')

const logout = () => {
  localStorage.removeItem('authToken')
  window.location.href = '/?logout=true'
}

onMounted(() => {
  // Prüfe ob User authentifiziert ist
  const auth = localStorage.getItem('authToken')
  if (!auth) {
    window.location.href = '/'
  }
})
</script>

<style scoped>
.nav-item {
  font-weight: 500;
  color: #6b7280;
  border-left: 3px solid transparent;
}

.nav-item.active {
  background-color: #f3f4f6;
  color: #6750a4;
  border-left-color: #6750a4;
  font-weight: 600;
}
</style>
