import type { UserConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// TODO: Fix types?
// @ts-expect-error I'm not sure how to set up these types properly
import path from "path";

export default {
  server: {
    port: 5173,
    proxy: {
      '/websocket': {
        target: 'ws://localhost:3000',
        changeOrigin: true,
        ws: true,
        rewriteWsOrigin: true,
      },
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true
      },
    }
  },
  build: {
    outDir: '../static',
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      // @ts-expect-error I'm not sure how to set up these types properly
      "@shared-bindings/*": path.resolve(__dirname, "../../shared/bindings"),
      // @ts-expect-error I'm not sure how to set up these types properly
      "@bindings/*": path.resolve(__dirname, "../bindings/*")
    }
  },
  plugins: [svelte()],
} satisfies UserConfig;