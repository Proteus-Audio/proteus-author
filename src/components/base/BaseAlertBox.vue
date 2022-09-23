<template>
  <div class="alert-box">
    <div v-for="(al, i) in alerts" :key="i" :class="`alert ${al.class}`">
      <el-alert :title="al.contents" :type="al.type" @close="() => closeAlert(i)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUpdated, ref, watch } from 'vue'
import { useAlertStore } from '../../stores/alerts'
import { AlertView } from '../../typings/proteus'

const alertStore = useAlertStore()
const alerts = ref([] as AlertView[])

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

<style lang="scss" scoped>
.alert-box {
  position: absolute;
  top: 2em;
  width: calc(100% - 4em);
  max-width: 600px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10;

  .alert {
    margin-bottom: 1em;
    transition: 0.5s opacity;

    &.stale {
      opacity: 0;
    }
  }
}
</style>
