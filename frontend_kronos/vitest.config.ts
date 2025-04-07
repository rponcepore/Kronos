import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    include: ["src/tests/**/*.test.tsx"], 
  },
  resolve: {
    alias: {
      "@": "/src", // Ensure absolute imports work for files inside `src/`
    },
  },
});
