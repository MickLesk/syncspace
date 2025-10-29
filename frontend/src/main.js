import { mount } from 'svelte'
import './styles/tailwind.css'
import './styles/design-system.css'
import './styles/mobile.css'
import App from './App.svelte'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
