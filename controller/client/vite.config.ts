import type { UserConfig } from 'vite'

export default {
  build: {
    outDir: '../static',
    emptyOutDir: true,
  }
} satisfies UserConfig;