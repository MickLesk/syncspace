<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">⚙️ Einstellungen</h2>
    <div class="grid gap-6 max-w-2xl">
      <!-- Theme Settings -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">🎨 Erscheinungsbild</h3>
        <div class="space-y-3">
          <label class="flex items-center gap-3 cursor-pointer">
            <input v-model="theme" value="light" type="radio" />
            <span>☀️ Helles Design</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer">
            <input v-model="theme" value="dark" type="radio" />
            <span>🌙 Dunkles Design</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer">
            <input v-model="theme" value="auto" type="radio" />
            <span>🔄 Automatisch</span>
          </label>
        </div>
      </div>

      <!-- Storage Settings -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">💾 Speicher</h3>
        <div class="space-y-3">
          <div>
            <div class="flex justify-between mb-2">
              <span class="text-sm font-medium">Speicherverbrauch</span>
              <span class="text-sm text-gray-600">{{ storageUsed }} / {{ storageTotal }}</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-3">
              <div class="bg-primary h-3 rounded-full transition-all" :style="{ width: storagePercent + '%' }"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Sync Settings -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">🔄 Synchronisierung</h3>
        <div class="space-y-3">
          <div>
            <label class="text-sm font-medium block mb-2">Synchronisierungsintervall</label>
            <select v-model="syncInterval" class="input">
              <option value="1">Jede Minute</option>
              <option value="5">Alle 5 Minuten</option>
              <option value="30">Alle 30 Minuten</option>
              <option value="60">Stündlich</option>
            </select>
          </div>
          <label class="flex items-center gap-3">
            <input v-model="autoSync" type="checkbox" />
            <span>Automatische Synchronisierung aktivieren</span>
          </label>
        </div>
      </div>

      <!-- API Tokens -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">🔐 API Tokens</h3>
        <div class="space-y-3">
          <div v-for="token in apiTokens" :key="token.id" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div>
              <h4 class="font-semibold text-sm">{{ token.name }}</h4>
              <p class="text-xs text-gray-500">Erstellt vor {{ token.daysOld }} Tagen</p>
            </div>
            <button @click="deleteToken(token.id)" class="px-3 py-1 text-red-600 hover:bg-red-100 rounded">🗑️</button>
          </div>
          <button @click="generateToken" class="w-full px-4 py-2 border border-primary text-primary rounded-lg hover:bg-primary hover:bg-opacity-5 transition">
            ➕ Neuer Token
          </button>
        </div>
      </div>

      <!-- About -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">ℹ️ Über</h3>
        <div class="space-y-2 text-sm text-gray-600">
          <p><strong>Version:</strong> 1.0.0</p>
          <p><strong>Backend:</strong> Rust/Warp</p>
          <p><strong>Frontend:</strong> Vue 3 + Tailwind CSS</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface ApiToken {
  id: string
  name: string
  daysOld: number
}

const theme = ref('auto')
const syncInterval = ref('5')
const autoSync = ref(true)
const apiTokens = ref<ApiToken[]>([])
const storageUsed = ref('2.5 GB')
const storageTotal = ref('10 GB')
const storagePercent = ref(25)

const generateToken = () => {
  const newToken: ApiToken = {
    id: Math.random().toString(),
    name: `Token ${apiTokens.value.length + 1}`,
    daysOld: 0
  }
  apiTokens.value.push(newToken)
}

const deleteToken = (id: string) => {
  apiTokens.value = apiTokens.value.filter(t => t.id !== id)
}

onMounted(() => {
  // Load settings from localStorage
  const saved = localStorage.getItem('settings')
  if (saved) {
    const settings = JSON.parse(saved)
    theme.value = settings.theme || 'auto'
    syncInterval.value = settings.syncInterval || '5'
    autoSync.value = settings.autoSync !== false
  }

  // Load mock API tokens
  apiTokens.value = [
    { id: '1', name: 'Mobile App', daysOld: 5 },
    { id: '2', name: 'Desktop Sync', daysOld: 12 }
  ]
})
</script>
