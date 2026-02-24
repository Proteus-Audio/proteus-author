<template>
  <div
    class="fixed left-1/2 z-[60] w-[calc(100%-2rem)] max-w-[600px] -translate-x-1/2 transition-[top] duration-300"
    :class="y > 150 ? 'top-20' : 'top-8'"
  >
    <div
      v-for="(al, i) in alerts"
      :key="al.id || `${al.added.getTime()}-${i}`"
      class="mb-4 rounded-lg bg-white transition-opacity duration-500"
      :class="al.class === 'stale' ? 'opacity-0' : 'opacity-100'"
    >
      <div
        class="grid grid-cols-[auto_1fr_auto] items-center gap-3 rounded-lg border px-4 py-3"
        :class="alertClasses(al.type)"
      >
        <span
          v-if="al.loading"
          class="inline-block size-4 animate-spin rounded-full border-2 border-current border-r-transparent opacity-90"
          aria-hidden="true"
        ></span>
        <span v-else class="inline-block size-4 rounded-full opacity-80" :class="alertDot(al.type)"></span>

        <div class="min-w-0 text-sm leading-snug">{{ al.contents }}</div>

        <button
          type="button"
          class="rounded px-2 py-1 text-xs opacity-70 transition-opacity hover:opacity-100"
          @click="closeAlert(i)"
        >
          Close
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useWindowScroll } from '@vueuse/core'
import { onMounted, onUpdated, ref, watch } from 'vue'
import { useAlertStore } from '../../stores/alerts'
import type { AlertType, AlertView } from '../../typings/proteus'

const alertStore = useAlertStore()
const alerts = ref([] as AlertView[])
// const alerts = ref([
//   { class: 'fresh', type: 'success', added: new Date(), contents: 'Success!' },
//   { class: 'fresh', type: 'info', added: new Date(), contents: 'Info!' },
//   { class: 'fresh', type: 'warning', added: new Date(), contents: 'Warning!' },
//   { class: 'fresh', type: 'error', added: new Date(), contents: 'Error!' }
// ] as AlertView[])

const { y } = useWindowScroll()

const alertClasses = (type: AlertType): string => {
  switch (type) {
    case 'success':
      return 'border-green-300 bg-green-50 text-green-900'
    case 'warning':
      return 'border-amber-300 bg-amber-50 text-amber-900'
    case 'error':
      return 'border-red-300 bg-red-50 text-red-900'
    default:
      return 'border-sky-300 bg-sky-50 text-sky-900'
  }
}

const alertDot = (type: AlertType): string => {
  switch (type) {
    case 'success':
      return 'bg-green-500'
    case 'warning':
      return 'bg-amber-500'
    case 'error':
      return 'bg-red-500'
    default:
      return 'bg-sky-500'
  }
}

const closeAlert = (index: number) => {
  alerts.value[index].autoClose = true
  alerts.value[index].loading = false
}

const checkAlerts = (time?: Date) => {
  const now: Date = time || new Date()

  alerts.value.forEach((alert, i) => {
    if (alert.autoClose && now.getTime() - alert.added.getTime() > 5 * 1000) {
      alerts.value[i].class = 'stale'
      setTimeout(() => {
        alerts.value.splice(i, 1)
      }, 550)
    }
  })
}

const processAlerts = () => {
  const now = new Date()
  while (alertStore.alerts.length > 0) {
    const alert = alertStore.alerts.shift()
    if (!alert) continue

    if (alert.upsert && alert.id) {
      const existing = alerts.value.find((item) => item.id === alert.id)
      if (existing) {
        existing.contents = alert.contents
        existing.type = alert.type
        existing.autoClose = alert.autoClose
        existing.loading = alert.loading ?? false
        existing.class = 'fresh'
        existing.added = now
        continue
      }
    }

    alerts.value.push({ ...alert, class: 'fresh', added: now })
  }

  checkAlerts(now)
  setTimeout(() => {
    checkAlerts()
  }, 5 * 1000)
}

watch(alertStore.alerts, () => {
  processAlerts()
})

onUpdated(() => {
  processAlerts()
})

onMounted(() => {
  processAlerts()
})
</script>
