// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import tailwind from '@astrojs/tailwind';

// https://astro.build/config
export default defineConfig({
  site: 'http://localhost:3003', // Add site URL for canonical URLs
  integrations: [
    react(),
    tailwind({
      applyBaseStyles: false, // We'll handle base styles in global.css
    })
  ],
  output: 'hybrid', // Enable server-side rendering for protected routes
  build: {
    inlineStylesheets: 'auto',
  },
  prefetch: {
    prefetchAll: true,
    defaultStrategy: 'viewport',
  },
  compressHTML: true,
  server: {
    port: 3003,
    host: true
  },
  // API proxy to Rust backend
  vite: {
    server: {
      proxy: {
        '/api': {
          target: 'http://localhost:3001',
          changeOrigin: true,
          secure: false,
        }
      }
    }
  }
});