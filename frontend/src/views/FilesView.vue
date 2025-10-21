<template>
  <div>
    <div class="flex justify-between items-center mb-6">
      <h2 class="text-3xl font-bold text-gray-900">Dateien</h2>
      <div class="flex gap-2">
        <button @click="viewMode = 'grid'" :class="['px-3 py-2 rounded-lg', viewMode === 'grid' ? 'bg-primary text-white' : 'bg-gray-100']">
          🔲 Grid
        </button>
        <button @click="viewMode = 'list'" :class="['px-3 py-2 rounded-lg', viewMode === 'list' ? 'bg-primary text-white' : 'bg-gray-100']">
          📋 Liste
        </button>
        <button @click="isUploadOpen = true" class="px-4 py-2 bg-primary text-white rounded-lg hover:shadow-lg transition">
          ⬆️ Hochladen
        </button>
      </div>
    </div>

    <!-- Breadcrumb Navigation -->
    <div class="mb-4 flex gap-2 items-center text-sm">
      <a @click="currentPath = ''" class="text-primary cursor-pointer hover:underline">📂 Startseite</a>
      <span v-for="(part, i) in currentPath.split('/').filter(Boolean)" :key="i" class="flex items-center gap-2">
        <span class="text-gray-400">/</span>
        <a @click="goToPath(currentPath.split('/').slice(0, i+2).join('/'))" class="text-primary cursor-pointer hover:underline">
          {{ part }}
        </a>
      </span>
    </div>

    <!-- Files Grid/List View -->
    <div v-if="viewMode === 'grid'" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <div 
        v-for="file in files" 
        :key="file.id"
        @contextmenu.prevent="showContextMenu(file, $event)"
        class="card cursor-pointer hover:shadow-lg hover:scale-105 transition group"
      >
        <div class="text-4xl mb-3 group-hover:scale-125 transition">{{ getFileIcon(file.type) }}</div>
        <h3 class="font-semibold text-gray-900 truncate mb-1">{{ file.name }}</h3>
        <p class="text-xs text-gray-500 mb-3">{{ formatFileSize(file.size) }}</p>
        <div class="flex gap-2">
          <button @click="downloadFile(file)" class="flex-1 px-2 py-1 text-xs bg-primary text-white rounded hover:opacity-90">
            ⬇️
          </button>
          <button @click="toggleFavorite(file)" class="flex-1 px-2 py-1 text-xs bg-yellow-100 text-yellow-700 rounded hover:opacity-90">
            ⭐
          </button>
        </div>
      </div>
    </div>

    <!-- List View -->
    <div v-else class="space-y-2">
      <div v-for="file in files" :key="file.id" class="flex items-center gap-4 p-4 card hover:shadow-md transition cursor-pointer" @contextmenu.prevent="showContextMenu(file, $event)">
        <span class="text-2xl">{{ getFileIcon(file.type) }}</span>
        <div class="flex-1">
          <h3 class="font-semibold text-gray-900">{{ file.name }}</h3>
          <p class="text-xs text-gray-500">{{ formatFileSize(file.size) }} • {{ formatDate(file.modified) }}</p>
        </div>
        <button @click="toggleFavorite(file)" class="px-3 py-1 text-yellow-600 hover:bg-yellow-100 rounded">⭐</button>
        <button @click="downloadFile(file)" class="px-3 py-1 text-primary hover:bg-gray-100 rounded">⬇️</button>
      </div>
    </div>

    <!-- Upload Modal -->
    <div v-if="isUploadOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="card p-8 rounded-2xl max-w-md w-full">
        <h3 class="text-xl font-bold mb-4">Datei hochladen</h3>
        <div 
          @drop.prevent="handleDrop"
          @dragover.prevent
          class="border-2 border-dashed border-primary rounded-lg p-8 text-center hover:bg-primary hover:bg-opacity-5 transition cursor-pointer"
        >
          <p class="text-gray-600 mb-2">Ziehe Dateien hierher oder klicke zum Auswählen</p>
          <input type="file" @change="handleFileSelect" multiple class="hidden" ref="fileInput" />
          <button @click="$refs.fileInput?.click()" class="text-primary font-semibold hover:underline">
            Dateien auswählen
          </button>
        </div>
        <button @click="isUploadOpen = false" class="mt-4 w-full px-4 py-2 text-gray-600 border border-gray-300 rounded-lg hover:bg-gray-50">
          Abbrechen
        </button>
      </div>
    </div>

    <!-- Context Menu -->
    <div v-if="contextMenu.visible" :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }" class="fixed bg-white border border-gray-200 rounded-lg shadow-lg z-50">
      <button @click="renameFile" class="block w-full text-left px-4 py-2 hover:bg-gray-100">✏️ Umbenennen</button>
      <button @click="deleteFile" class="block w-full text-left px-4 py-2 text-red-600 hover:bg-gray-100">🗑️ Löschen</button>
      <button @click="copyToClipboard" class="block w-full text-left px-4 py-2 hover:bg-gray-100">📋 Kopieren</button>
      <button @click="compressFile" class="block w-full text-left px-4 py-2 hover:bg-gray-100">📦 Komprimieren</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import axios from 'axios'

interface FileItem {
  id: string
  name: string
  type: string
  size: number
  modified: string
  isFavorite?: boolean
}

const files = ref<FileItem[]>([])
const currentPath = ref('')
const viewMode = ref('grid')
const isUploadOpen = ref(false)
const fileInput = ref<HTMLInputElement>()
const contextMenu = ref({ visible: false, x: 0, y: 0, file: null as FileItem | null })

const getFileIcon = (type: string) => {
  const icons: Record<string, string> = {
    'folder': '📁',
    'image': '🖼️',
    'video': '🎬',
    'audio': '🎵',
    'document': '📄',
    'pdf': '📕',
    'zip': '📦',
    'code': '💻',
    'default': '📄'
  }
  return icons[type] || icons['default']
}

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

const formatDate = (date: string) => new Date(date).toLocaleDateString('de-DE')

const loadFiles = async () => {
  try {
    const response = await axios.get(`/api/files/${currentPath.value}`)
    files.value = response.data.map((f: any, i: number) => ({
      id: i.toString(),
      ...f
    }))
  } catch (err) {
    console.error('Fehler beim Laden:', err)
  }
}

const downloadFile = async (file: FileItem) => {
  const link = document.createElement('a')
  link.href = `/api/file/${currentPath.value}/${file.name}`
  link.download = file.name
  link.click()
}

const toggleFavorite = (file: FileItem) => {
  file.isFavorite = !file.isFavorite
}

const handleDrop = (e: DragEvent) => {
  const files = e.dataTransfer?.files
  if (files) handleUpload(files)
}

const handleFileSelect = (e: Event) => {
  const files = (e.target as HTMLInputElement).files
  if (files) handleUpload(files)
}

const handleUpload = async (fileList: FileList) => {
  for (const file of fileList) {
    try {
      const formData = new FormData()
      formData.append('file', file)
      await axios.post(`/api/upload/${currentPath.value}/${file.name}`, formData)
      await loadFiles()
    } catch (err) {
      console.error('Upload fehlgeschlagen:', err)
    }
  }
  isUploadOpen.value = false
}

const renameFile = () => {
  if (!contextMenu.value.file) return
  const newName = prompt('Neuer Name:', contextMenu.value.file.name)
  if (newName) {
    // API call to rename
  }
}

const deleteFile = async () => {
  if (!contextMenu.value.file) return
  if (confirm('Wirklich löschen?')) {
    try {
      await axios.delete(`/api/files/${currentPath.value}/${contextMenu.value.file.name}`)
      await loadFiles()
    } catch (err) {
      console.error('Fehler beim Löschen:', err)
    }
  }
}

const copyToClipboard = () => {
  if (contextMenu.value.file) {
    navigator.clipboard.writeText(contextMenu.value.file.name)
  }
}

const compressFile = () => {
  // Compression logic
}

const showContextMenu = (file: FileItem, e: MouseEvent) => {
  contextMenu.value = {
    visible: true,
    x: e.clientX,
    y: e.clientY,
    file
  }
}

const goToPath = (path: string) => {
  currentPath.value = path
}

onMounted(() => {
  loadFiles()
  document.addEventListener('click', () => {
    contextMenu.value.visible = false
  })
})

onUnmounted(() => {
  document.removeEventListener('click', () => {
    contextMenu.value.visible = false
  })
})
</script>
