import { describe, expect, it } from 'vitest'
import { screen } from '@testing-library/dom'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import unstorageMemory from 'unstorage/drivers/memory'

import App from '@/App.vue'
import { routes } from '@/router'
import { useDriver } from '@/storage'

describe('App shell', () => {
  it('opens on the home screen with the Rust core loaded', async () => {
    useDriver(unstorageMemory())
    const router = createRouter({ history: createMemoryHistory(), routes })
    await router.push('/')
    mount(App, {
      global: { plugins: [createPinia(), router, createVuetify({ components, directives })] },
      attachTo: document.body,
    })
    await flushPromises()
    expect(await screen.findByText(/Core v\d+\.\d+\.\d+/)).toBeInTheDocument()
  })
})
