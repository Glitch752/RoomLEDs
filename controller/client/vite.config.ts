import type { UserConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// TODO: Fix types?
// @ts-expect-error I'm not sure how to set up these types properly
import path from "path";

export default {
  build: {
    outDir: '../static',
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      // @ts-expect-error I'm not sure how to set up these types properly
      "@bindings": path.resolve(__dirname, "../../shared/bindings"),
    }
  },
  plugins: [svelte()],
} satisfies UserConfig;