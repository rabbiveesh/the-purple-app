import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import { createVuetify } from 'vuetify'

export default createVuetify({
  theme: {
    defaultTheme: 'system',
    themes: {
      light: { colors: { primary: '#6a1b9a', secondary: '#ec407a' } },
      dark: { colors: { primary: '#ce93d8', secondary: '#f48fb1' } },
    },
  },
  defaults: {
    VBtn: { size: 'large' },
    VCard: { rounded: 'lg' },
  },
})
