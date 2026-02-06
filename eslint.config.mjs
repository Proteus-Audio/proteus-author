import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript'
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended'
import pluginVue from 'eslint-plugin-vue'

export default defineConfigWithVueTs(
  {
    ignores: ['src/components/playground/**', 'src/vite-env.d.ts'],
  },
  eslintPluginPrettierRecommended,
  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommendedTypeChecked,
)
