import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Alert, AlertType } from '../typings/proteus'

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

  const addAlert = (
    contents: string,
    type?: AlertType,
    options?: Partial<Pick<Alert, 'id' | 'loading' | 'autoClose'>>,
  ) => {
    type = type || 'info'
    alerts.value.push({
      contents,
      type,
      id: options?.id,
      loading: options?.loading ?? false,
      autoClose: options?.autoClose ?? (options?.loading ? false : type !== 'error'),
      upsert: false,
    })
  }

  const upsertAlert = (
    id: string,
    contents: string,
    type?: AlertType,
    options?: Partial<Pick<Alert, 'loading' | 'autoClose'>>,
  ) => {
    type = type || 'info'
    alerts.value.push({
      id,
      contents,
      type,
      loading: options?.loading ?? false,
      autoClose: options?.autoClose ?? (options?.loading ? false : type !== 'error'),
      upsert: true,
    })
  }

  return {
    alerts,
    addAlert,
    upsertAlert,
  }
})
