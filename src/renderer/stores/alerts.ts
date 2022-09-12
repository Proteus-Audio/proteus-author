import { defineStore } from 'pinia'
import { ref } from 'vue'
import { Alert, AlertType } from '../typings/proteus'

export const useAlertStore = defineStore('alert', () => {
  /////////////
  //  STORE  //
  /////////////

  const alerts = ref([] as Alert[])

  /////////////
  // GETTERS //
  /////////////

  /////////////
  // SETTERS //
  /////////////

  const addAlert = (contents: string, type?: AlertType) => {
    type = type || 'info'
    alerts.value.push({ contents, type, autoClose: type !== 'error' })
  }

  return {
    alerts,
    addAlert,
  }
})
