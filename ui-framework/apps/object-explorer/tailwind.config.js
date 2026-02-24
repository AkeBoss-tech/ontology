/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Foundry/Blueprint-inspired palette
        dark: {
          100: '#394b59',
          200: '#30404d', // Sidebar bg
          300: '#293742',
          400: '#202b33',
          500: '#182026', // Main text
        },
        light: {
          100: '#ffffff',
          200: '#f5f8fa', // App bg
          300: '#ebf1f5', // Boder
          400: '#CED9E0',
        },
        foundry: {
          blue: '#1bd8f5', // Bright accent
          core: '#2D72D2', // Primary blue
          hover: '#48aff0',
        }
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
