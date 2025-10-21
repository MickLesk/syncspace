<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">📊 Aktivitätslog</h2>
    <div class="space-y-2">
      <div v-for="activity in activities" :key="activity.id" class="card flex items-start gap-4">
        <div class="text-2xl mt-1">{{ getActivityIcon(activity.type) }}</div>
        <div class="flex-1">
          <h3 class="font-semibold text-gray-900">{{ activity.description }}</h3>
          <p class="text-xs text-gray-500">{{ formatDate(activity.timestamp) }}</p>
        </div>
        <span class="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded">{{ activity.type }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Activity {
  id: string
  type: string
  description: string
  timestamp: string
}

const activities = ref<Activity[]>([])

const getActivityIcon = (type: string) => {
  const icons: Record<string, string> = {
    'upload': '⬆️',
    'download': '⬇️',
    'delete': '🗑️',
    'create': '✨',
    'modify': '✏️'
  }
  return icons[type] || '📌'
}

const formatDate = (date: string) => new Date(date).toLocaleString('de-DE')

onMounted(() => {
  // Load mock activities
  activities.value = [
    { id: '1', type: 'upload', description: 'Datei hochgeladen: document.pdf', timestamp: new Date().toISOString() },
    { id: '2', type: 'modify', description: 'Datei bearbeitet: presentation.pptx', timestamp: new Date().toISOString() },
    { id: '3', type: 'delete', description: 'Datei gelöscht: old_file.txt', timestamp: new Date().toISOString() }
  ]
})
</script>
