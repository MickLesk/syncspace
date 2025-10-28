import { mount } from 'svelte'
import './styles/tailwind.css'
import './styles/light-mode.css'
import './styles/dark-mode.css'
import App from './App.svelte'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
