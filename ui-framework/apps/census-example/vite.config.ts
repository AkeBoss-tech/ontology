import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: parseInt(process.env.PORT || '3000', 10),
    proxy: {
      '/graphql': {
        target: process.env.VITE_GRAPHQL_URL || 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
});





