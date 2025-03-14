import { sveltekit } from "@sveltejs/kit/vite";
import { enhancedImages } from "@sveltejs/enhanced-img";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [
		enhancedImages(),
		sveltekit()
	],
	build: {
		chunkSizeWarningLimit: 1000,
		minify: 'terser',
		rollupOptions: {
			output: {
				manualChunks: {
					vendor: ['svelte']
				}
			}
		}
	},
	server: {
		fs: {
			allow: ['static']
		}
	}
});
