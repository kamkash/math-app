import { defineConfig } from "vite";
import { viteStaticCopy } from 'vite-plugin-static-copy'; // we will use this plugin

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  base: './', // very important for Tauri!
  build: {
    assetsDir: 'assets', // so fonts/css/js go to 'dist/assets'
  },
  optimizeDeps: {
    exclude: ['@mathlive/mathlive'], // Exclude MathLive from Vite optimization
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**", "**/math-parser/**", "**/symengine-rs/**"],
    },
  },
  plugins: [
    viteStaticCopy({
      targets: [
        {
          src: 'node_modules/mathlive/fonts', // Copy from node_modules
          dest: 'node_modules/.vite/deps',    // Copy to deps where MathLive expects
        },
      ],
    }),
  ],
}));
