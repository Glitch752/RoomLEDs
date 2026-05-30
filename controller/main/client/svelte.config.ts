import adapterStatic from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import path from 'node:path';

export default {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapterStatic({
      assets: '../static',
      fallback: 'index.html',
      pages: '../static'
    }),
    files: {
      assets: 'public'
    },
    alias: {
      '@shared-bindings': path.resolve('../../../shared/bindings'),
      '@bindings': path.resolve('../bindings')
    }
  }
}
