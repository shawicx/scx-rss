import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const materialLight = {
  dark: false,
  colors: {
    background: '#FFFFFF',
    surface: '#FFFFFF',
    'surface-variant': '#F5F5F5',
    'on-surface-variant': '#757575',
    primary: '#1867C6',
    'primary-darken-1': '#1256A3',
    secondary: '#5CBBF6',
    'secondary-darken-1': '#3DA6E3',
    accent: '#1867C6',
    error: '#B00020',
    info: '#2196F3',
    success: '#4CAF50',
    warning: '#FB8C00',
  },
}

const materialDark = {
  dark: true,
  colors: {
    background: '#121212',
    surface: '#1E1E1E',
    'surface-variant': '#2C2C2C',
    'on-surface-variant': '#AAAAAA',
    primary: '#90CAF9',
    'primary-darken-1': '#6EA8E0',
    secondary: '#CE93D8',
    'secondary-darken-1': '#B06EB8',
    accent: '#90CAF9',
    error: '#CF6679',
    info: '#64B5F6',
    success: '#81C784',
    warning: '#FFB74D',
  },
}

export default createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: 'materialLight',
    themes: {
      materialLight,
      materialDark,
    },
  },
  defaults: {
    VBtn: { density: 'comfortable' },
    VTextField: { density: 'comfortable', variant: 'outlined' },
    VCard: { rounded: 'lg' },
  },
})
