import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    // Specify the development server port
    port: 9000,
  },
  // Base name of your app
  base: "/", 
})
