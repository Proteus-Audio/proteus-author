import ui from '@nuxt/ui/vite'
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    ui({
      router: false,
      ui: {
        colors: {
          primary: 'sky',
          secondary: 'slate',
          success: 'emerald',
          info: 'cyan',
          warning: 'amber',
          error: 'rose',
          neutral: 'zinc',
        },
        button: {
          defaultVariants: {
            color: 'neutral',
            size: 'sm',
            variant: 'ghost',
          },
        },
        alert: {
          defaultVariants: {
            variant: 'outline',
          },
        },
      },
      theme: {
        defaultVariants: {
          color: 'neutral',
          size: 'sm',
        },
      },
    }),
  ],

  // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    // Pre-transform hot startup modules so the first webview navigation does
    // less on-demand work before `main.ts` executes.
    warmup: {
      clientFiles: [
        './index.html',
        './src/main.ts',
        './src/App.vue',
        './src/assets/index.css',
        './src/assets/theme.css',
        './src/assets/fonts.css',
      ],
    },
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  // Pre-bundle frequently used dependencies to reduce first-request transform
  // latency in `tauri dev` cold starts.
  optimizeDeps: {
    include: [
      'vue',
      'pinia',
      '@vueuse/core',
      'vuedraggable',
      '@tauri-apps/api/core',
      '@tauri-apps/api/event',
      '@tauri-apps/api/window',
      '@nuxt/ui/runtime/components/Button.vue',
      '@nuxt/ui/runtime/components/Alert.vue',
      '@nuxt/ui/runtime/components/Drawer.vue',
      '@nuxt/ui/runtime/components/DropdownMenu.vue',
      '@nuxt/ui/runtime/components/Input.vue',
      '@nuxt/ui/runtime/components/Modal.vue',
    ],
  },
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (!id.includes('node_modules')) return

          // Keep major ecosystems isolated so the entry chunk stays small and
          // hot-path parsing can start sooner.
          if (id.includes('/node_modules/vue/') || id.includes('/node_modules/@vue/')) {
            return 'vendor-vue'
          }
          if (id.includes('/node_modules/@tauri-apps/')) return 'vendor-tauri'
          if (id.includes('/node_modules/vuedraggable/')) return 'vendor-dnd'
          return 'vendor-misc'
        },
      },
    },
  },
})
