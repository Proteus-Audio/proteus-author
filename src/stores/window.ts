import { defineStore } from 'pinia'
import { ref } from 'vue'
import { Window } from '@tauri-apps/api/window'

export const useWindowStore = defineStore('window', () => {
  /////////////
  //  STORE  //
  /////////////

  const window = ref(null as Window | null)

  /////////////
  // GETTERS //
  /////////////

  /////////////
  // SETTERS //
  /////////////

  const init = async () => {
    const appWindow = new Window('main')
    appWindow.isFocused()
    window.value = appWindow
  }

  return {
    window,
    init,
  }
})
