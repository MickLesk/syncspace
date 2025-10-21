<template>
  <div>
    <h2 class="text-3xl font-bold text-gray-900 mb-6">🔄 Synchronisation</h2>
    <div class="grid gap-6">
      <!-- Connected Devices -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">🖥️ Verbundene Geräte</h3>
        <div class="space-y-3">
          <div v-for="device in devices" :key="device.id" class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div>
              <h4 class="font-semibold">{{ device.name }}</h4>
              <p class="text-xs text-gray-500">{{ device.status }}</p>
            </div>
            <span :class="['px-3 py-1 rounded text-xs font-medium', device.online ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700']">
              {{ device.online ? '🟢 Online' : '🔴 Offline' }}
            </span>
          </div>
        </div>
      </div>

      <!-- Sync Status -->
      <div class="card">
        <h3 class="text-lg font-bold mb-4">📊 Synchronisierungsstatus</h3>
        <div class="space-y-4">
          <div>
            <div class="flex justify-between mb-2">
              <span class="text-sm font-medium">Gesamtfortschritt</span>
              <span class="text-sm text-gray-600">{{ syncProgress }}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-primary h-2 rounded-full transition-all" :style="{ width: syncProgress + '%' }"></div>
            </div>
          </div>
          <p class="text-sm text-gray-600">Zuletzt synchronisiert: {{ lastSyncTime }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Device {
  id: string
  name: string
  status: string
  online: boolean
}

const devices = ref<Device[]>([])
const syncProgress = ref(85)
const lastSyncTime = ref('vor 2 Minuten')

onMounted(() => {
  devices.value = [
    { id: '1', name: 'Arbeits-PC', status: 'Synchronisiert', online: true },
    { id: '2', name: 'Laptop', status: 'Wartet...', online: true },
    { id: '3', name: 'Tablet', status: 'Offline', online: false }
  ]
})
</script>
