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
    safelist: [
        'bg-blue-100', 'text-blue-600', 'text-blue-800',
        'bg-green-100', 'text-green-600', 'text-green-700', 'text-green-800',
        'bg-purple-100', 'text-purple-600', 'text-purple-800',
        'bg-orange-100', 'text-orange-600', 'text-orange-800',
        'bg-pink-100', 'text-pink-600',
    ],
}
