/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        serif: ['Crimson Text', 'Georgia', 'serif'],
      },
      colors: {
        // Monochrome color palette like Medium
        gray: {
          50: '#fafafa',
          100: '#f5f5f5',
          200: '#eeeeee',
          300: '#e0e0e0',
          400: '#bdbdbd',
          500: '#9e9e9e',
          600: '#757575',
          700: '#616161',
          800: '#424242',
          900: '#212121',
        },
      },
      typography: {
        DEFAULT: {
          css: {
            maxWidth: '680px', // Medium's article width
            color: 'inherit',
            lineHeight: '1.6',
            a: {
              color: 'inherit',
              textDecoration: 'underline',
              textDecorationColor: '#9e9e9e',
              textUnderlineOffset: '2px',
              '&:hover': {
                textDecorationColor: '#212121',
              },
            },
            b: { color: 'inherit' },
            strong: { color: 'inherit', fontWeight: '600' },
            em: { color: 'inherit' },
            h1: { color: 'inherit', fontWeight: '700' },
            h2: { color: 'inherit', fontWeight: '600' },
            h3: { color: 'inherit', fontWeight: '600' },
            h4: { color: 'inherit', fontWeight: '500' },
            code: { 
              color: 'inherit',
              backgroundColor: '#f5f5f5',
              padding: '0.125rem 0.25rem',
              borderRadius: '0.25rem',
              fontSize: '0.875em',
            },
          },
        },
      },
    },
  },
  plugins: [],
}
