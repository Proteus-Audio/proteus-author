<template>
  <div
    class="fixed left-1/2 z-10 w-[calc(100%-2rem)] max-w-[600px] -translate-x-1/2 transition-[top] duration-300"
    :class="y > 150 ? 'top-20' : 'top-8'"
  >
    <div
      v-for="(al, i) in alerts"
      :key="i"
      class="mb-4 transition-opacity duration-500"
      :class="al.class === 'stale' ? 'opacity-0' : 'opacity-100'"
    >
      <UAlert
        :title="al.contents"
        :color="alertColor(al.type)"
        variant="outline"
        :close="{ color: 'neutral', variant: 'ghost' }"
        @update:open="(open: boolean) => !open && closeAlert(i)"
      />
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

const { y } = useWindowScroll()

const alertColor = (type: AlertType): 'success' | 'warning' | 'info' | 'error' => {
  return type
}

const closeAlert = (index: number) => {
  alerts.value[index].autoClose = true
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
    if (alert) alerts.value.push({ ...alert, class: 'fresh', added: now })
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
