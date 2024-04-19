import { sveltekit } from '@sveltejs/kit/vite';
import preprocess from 'svelte-preprocess';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()]
});
