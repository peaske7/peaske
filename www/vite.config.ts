import { enhancedImages } from "@sveltejs/enhanced-img";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [enhancedImages(), sveltekit(), tailwindcss()],
	build: {
		chunkSizeWarningLimit: 1000,
		minify: "terser",
		rollupOptions: {
			output: {
				manualChunks: {
					vendor: ["svelte"],
				},
			},
		},
	},
	server: {
		fs: {
			allow: ["static"],
		},
	},
});
