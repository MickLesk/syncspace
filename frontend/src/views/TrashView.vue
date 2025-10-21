<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">🗑️ Papierkorb</h2>
    <div class="space-y-2">
      <div v-for="item in trashedItems" :key="item.id" class="card flex items-center justify-between">
        <div class="flex items-center gap-4 flex-1">
          <span class="text-2xl">{{ getFileIcon(item.type) }}</span>
          <div>
            <h3 class="font-semibold">{{ item.name }}</h3>
            <p class="text-xs text-gray-500">Gelöscht vor {{ getDaysAgo(item.deletedAt) }} Tagen</p>
          </div>
        </div>
        <div class="flex gap-2">
          <button @click="restoreItem(item)" class="px-3 py-1 bg-blue-100 text-blue-700 rounded hover:bg-blue-200">↩️ Wiederherstellen</button>
          <button @click="permanentlyDelete(item)" class="px-3 py-1 bg-red-100 text-red-700 rounded hover:bg-red-200">🔥 Endgültig</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface TrashItem {
  id: string
  name: string
  type: string
  deletedAt: string
}

const trashedItems = ref<TrashItem[]>([])

const getFileIcon = (type: string) => '📄'

const getDaysAgo = (date: string) => {
  const days = Math.floor((Date.now() - new Date(date).getTime()) / (1000 * 60 * 60 * 24))
  return days
}

const restoreItem = (item: TrashItem) => {
  trashedItems.value = trashedItems.value.filter(i => i.id !== item.id)
}

const permanentlyDelete = (item: TrashItem) => {
  if (confirm('Wirklich endgültig löschen?')) {
    trashedItems.value = trashedItems.value.filter(i => i.id !== item.id)
  }
}

onMounted(() => {
  // Mock data
  trashedItems.value = [
    { id: '1', name: 'old_project.zip', type: 'zip', deletedAt: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString() }
  ]
})
</script>
