<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">🔍 Suche</h2>
    <div class="mb-6">
      <input 
        v-model="searchQuery" 
        @input="performSearch"
        placeholder="Suche nach Dateien..." 
        class="input w-full max-w-md"
      />
    </div>
    <div class="space-y-2">
      <div v-for="result in searchResults" :key="result.id" class="card flex items-center gap-4 hover:shadow-lg">
        <span class="text-2xl">📄</span>
        <div class="flex-1">
          <h3 class="font-semibold">{{ result.name }}</h3>
          <p class="text-xs text-gray-500">{{ result.path }}</p>
        </div>
        <button class="px-3 py-1 bg-primary text-white rounded hover:opacity-90">Öffnen</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'

interface SearchResult {
  id: string
  name: string
  path: string
}

const searchQuery = ref('')
const searchResults = ref<SearchResult[]>([])

const performSearch = async () => {
  if (searchQuery.value.length < 2) {
    searchResults.value = []
    return
  }
  try {
    const response = await axios.get(`/api/search?q=${searchQuery.value}`)
    searchResults.value = response.data
  } catch (err) {
    console.error('Suche fehlgeschlagen:', err)
  }
}
</script>
