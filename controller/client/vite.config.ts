import type { UserConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default {
  build: {
    outDir: '../static',
    emptyOutDir: true,
  },
  plugins: [svelte()],
} satisfies UserConfig;