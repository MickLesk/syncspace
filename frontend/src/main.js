import { mount } from 'svelte'
import './styles/design-system.css'
import App from './App.svelte'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
