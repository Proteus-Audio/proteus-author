import { defineStore } from 'pinia'
import { ref } from 'vue'
import { Menu } from '@tauri-apps/api/menu'
import { defaultMenu } from '../utils/menu.js'

export const useMenuStore = defineStore('menu', () => {
  /////////////
  //  STORE  //
  /////////////

  const menu = ref(null as Menu | null)

  /////////////
  // GETTERS //
  /////////////

  /////////////
  // SETTERS //
  /////////////

  const init = async () => {
    console.log('init menu')
    const newMenu = await defaultMenu()
    console.log('new menu', newMenu)
    await newMenu.setAsAppMenu()
    menu.value = newMenu
  }

  return {
    menu,
    init,
  }
})
