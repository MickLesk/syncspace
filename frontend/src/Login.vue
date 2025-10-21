<template>
  <div class="min-h-screen bg-gradient-to-br from-primary via-secondary to-tertiary flex items-center justify-center p-4">
    <div class="card rounded-2xl shadow-2xl max-w-md w-full p-8 bg-white">
      <!-- Logo -->
      <div class="flex justify-center mb-6">
        <div class="w-16 h-16 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center">
          <span class="text-white font-bold text-3xl">S</span>
        </div>
      </div>

      <!-- Title -->
      <h1 class="text-3xl font-bold text-center text-gray-900 mb-2">SyncSpace</h1>
      <p class="text-center text-gray-500 mb-8">Dateiensynchronisierung reimagined</p>

      <!-- Login Form -->
      <form @submit.prevent="handleLogin" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Benutzername</label>
          <input 
            v-model="username" 
            type="text" 
            placeholder="admin" 
            class="input"
            required
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Passwort</label>
          <input 
            v-model="password" 
            type="password" 
            placeholder="••••••••" 
            class="input"
            required
          />
        </div>

        <button 
          type="submit"
          :disabled="isLoading"
          class="w-full py-3 bg-gradient-to-r from-primary to-secondary text-white rounded-full font-semibold hover:shadow-lg transition disabled:opacity-50"
        >
          {{ isLoading ? 'Wird angemeldet...' : 'Anmelden' }}
        </button>
      </form>

      <!-- Error Message -->
      <div v-if="errorMessage" class="mt-4 p-3 bg-red-100 text-red-700 rounded-lg text-sm">
        {{ errorMessage }}
      </div>

      <!-- Demo Credentials -->
      <div class="mt-6 p-4 bg-blue-50 border border-blue-200 rounded-lg">
        <p class="text-sm text-blue-900">
          <strong>Demo:</strong> admin / admin
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const username = ref('')
const password = ref('')
const isLoading = ref(false)
const errorMessage = ref('')

const handleLogin = async () => {
  isLoading.value = true
  errorMessage.value = ''

  try {
    // Demo auth - in production würde hier ein API-Call stattfinden
    if (username.value === 'admin' && password.value === 'admin') {
      localStorage.setItem('authToken', 'demo-token-' + Date.now())
      localStorage.setItem('username', username.value)
      window.location.href = '/app'
    } else {
      errorMessage.value = 'Ungültige Anmeldedaten'
    }
  } catch (err) {
    errorMessage.value = 'Anmeldefehler'
    console.error(err)
  } finally {
    isLoading.value = false
  }
}
</script>
