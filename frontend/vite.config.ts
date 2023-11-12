import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:52525',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // ...
          codemirror: [
            // Split CodeMirror code.
            'vue-codemirror6'
          ],
          'codemirror-lang': [
            // Add the following as needed.
            '@codemirror/lang-markdown',
          ],
          // ...
        },
      },
    },
  },
})
