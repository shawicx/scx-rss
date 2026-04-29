import { defineConfig, presetWind } from 'unocss'

export default defineConfig({
  presets: [
    presetWind(),
  ],
  shortcuts: {
    'flex-center': 'flex items-center justify-center',
    'flex-between': 'flex items-center justify-between',
  },
  theme: {
    colors: {
      ink: {
        dark: '#0f0e11',
        'dark-raised': '#1a181d',
        'dark-hover': '#24222a',
        'dark-active': '#2e2b36',
        paper: '#f6f2ec',
        'paper-bright': '#faf8f4',
        accent: '#c07a4a',
        'accent-hover': '#a86a3e',
        'accent-light': '#f5e8dc',
        border: '#e5e0d8',
        'border-light': '#ede9e2',
        'border-dark': '#2a2732',
        success: '#4a7c59',
        error: '#b84040',
        warning: '#a67c2e',
      },
    },
  },
})
