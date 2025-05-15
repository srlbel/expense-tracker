// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
	vite: {
		server: {
			proxy: {
				'/api': {
					target: 'http://localhost:8080',
					changeOrigin: true,
					secure: false,
				},
			},
		},
	},
	output: 'static',
});
