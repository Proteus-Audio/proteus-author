import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript'
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended'
import pluginVue from 'eslint-plugin-vue'

export default defineConfigWithVueTs(
  eslintPluginPrettierRecommended,
  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommendedTypeChecked,
)
