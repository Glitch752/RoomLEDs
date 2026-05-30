import type { UserConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import compileTime from "vite-plugin-compile-time";

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
    },
    allowedHosts: true
  },
  plugins: [sveltekit(), compileTime()],
} satisfies UserConfig;