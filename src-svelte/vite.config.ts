import { sveltekit } from '@sveltejs/kit/vite';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite'


export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte',
			}),
]
});
