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

async function openApp(path = '/') {
  useDriver(unstorageMemory())
  const router = createRouter({ history: createMemoryHistory(), routes })
  await router.push(path)
  mount(App, {
    global: { plugins: [createPinia(), router, createVuetify({ components, directives })] },
    attachTo: document.body,
  })
  await flushPromises()
  return router
}

describe('the app', () => {
  it('opens on the pretzel jar with one big button to start', async () => {
    await openApp()
    expect(await screen.findByRole('link', { name: /pretzel time/i })).toBeInTheDocument()
    expect(screen.getByRole('img', { name: /pretzel jar: 0 of 100/i })).toBeInTheDocument()
  })

  it('never gives away what the exercises are for', async () => {
    await openApp()
    await screen.findByRole('link', { name: /pretzel time/i })
    expect(document.body.textContent).not.toMatch(/kegel|pelvic/i)
    expect(document.title).not.toMatch(/kegel|pelvic/i)
  })

  it('asks which coach to use before starting', async () => {
    await openApp('/session')
    expect(await screen.findByText(/ready to start/i)).toBeInTheDocument()
    expect(screen.getByText('Silent')).toBeInTheDocument()
    expect(screen.getByText('Voice only')).toBeInTheDocument()
  })
})
