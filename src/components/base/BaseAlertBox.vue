<template>
  <div class="alert-box" :class="{ lower: y > 150 }">
    <div v-for="(al, i) in alerts" :key="i" :class="`alert ${al.class}`">
      <el-alert :title="al.contents" :type="al.type" @close="() => closeAlert(i)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUpdated, ref, watch } from 'vue'
import { useAlertStore } from '../../stores/alerts'
import { AlertView } from '../../typings/proteus'
import { useWindowScroll } from '@vueuse/core'

const alertStore = useAlertStore()
const alerts = ref([] as AlertView[])

const { y } = useWindowScroll()

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

const alertTop = computed(() => {
  return y.value > 150 ? '5em' : '2em'
})
</script>

<style lang="scss" scoped>
.alert-box {
  position: fixed;
  top: 2em;
  width: calc(100% - 4em);
  max-width: 600px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10;
  transition: top 0.3s;

  &.lower {
    top: 5em;
  }

  .alert {
    margin-bottom: 1em;
    transition: 0.5s opacity;

    &.stale {
      opacity: 0;
    }
  }
}
</style>
