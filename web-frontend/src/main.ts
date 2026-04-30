import './app.css';
import { mount } from 'svelte';
import App from './App.svelte';

// Mount the root component using Svelte 5 API
const app = mount(App, {
	target: document.getElementById('app')!,
});

export default app;
