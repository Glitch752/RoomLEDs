import { mount } from 'svelte';
import './style.css';
import SimpleApp from './pages/Simple.svelte';
import FullApp from './pages/Full.svelte';
import "./websocket";

// If on any path except /full, load the simple app
const appComponent = window.location.pathname === '/full' ? FullApp : SimpleApp;
const app = mount(appComponent, {
  target: document.getElementById('app')!,
});

export default app;