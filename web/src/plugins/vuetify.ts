import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'
import { createVuetify } from 'vuetify'

// Fun colours: purple first, with pink and pretzel gold.
export default createVuetify({
  theme: {
    defaultTheme: 'system',
    themes: {
      light: { colors: { primary: '#8e24aa', secondary: '#ec407a', accent: '#ffb300' } },
      dark: { colors: { primary: '#ce93d8', secondary: '#f48fb1', accent: '#ffca28' } },
    },
  },
  defaults: {
    VBtn: { size: 'large' },
    VCard: { rounded: 'xl' },
  },
})
