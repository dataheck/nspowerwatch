import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const config: UserConfig = {
	server: {
		fs: {
			allow: [
				'..'
			]
		}
	},
	plugins: [
		sveltekit(),
		wasm(),
		topLevelAwait()
	]
};

export default config;
