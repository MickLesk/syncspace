<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">⭐ Favoriten</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <div v-for="file in favorites" :key="file.id" class="card">
        <div class="text-4xl mb-3">🌟</div>
        <h3 class="font-semibold truncate">{{ file.name }}</h3>
        <p class="text-xs text-gray-500">{{ formatFileSize(file.size) }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface FileItem {
  id: string
  name: string
  size: number
}

const favorites = ref<FileItem[]>([])

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

onMounted(() => {
  // Load favorites from localStorage
  const saved = localStorage.getItem('favorites')
  if (saved) favorites.value = JSON.parse(saved)
})
</script>
