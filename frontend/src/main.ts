import { createApp } from 'vue'
import App from './App.vue'
import Login from './Login.vue'
import './style.css'

const app = createApp({
  template: `
    <div id="app">
      <Login v-if="!isLoggedIn" @login="isLoggedIn = true" />
      <App v-else />
    </div>
  `,
  components: { Login, App },
  data() {
    return {
      isLoggedIn: !!localStorage.getItem('authToken')
    }
  },
  watch: {
    isLoggedIn(val) {
      if (!val) {
        localStorage.removeItem('authToken')
      }
    }
  }
})

app.mount('#app')
