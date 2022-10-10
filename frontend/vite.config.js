import { sveltekit } from '@sveltejs/kit/vite';
/** @type {import('vite').UserConfig} */
const config = {
	server: {
		proxy: {
			'/api': 'http://localhost:8000'
		}
	},
	plugins: [sveltekit()]
};

export default config;
