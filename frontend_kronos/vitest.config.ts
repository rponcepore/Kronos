import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    include: ["src/tests/**/*.test.tsx"], // Ensures tests outside `src/` are included
  },
  resolve: {
    alias: {
      "@": "/src", // Ensure absolute imports work for files inside `src/`
    },
  },
});
