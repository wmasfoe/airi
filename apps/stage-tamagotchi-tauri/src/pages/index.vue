<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

const healthStatus = ref('checking...')

invoke<string>('health_check')
  .then((result) => {
    healthStatus.value = result
  })
  .catch((err) => {
    healthStatus.value = `error: ${err}`
  })
</script>

<template>
  <div :class="['h-screen w-screen', 'flex flex-col items-center justify-center', 'bg-white/80 dark:bg-black/80']">
    <h1 :class="['text-3xl font-bold', 'text-primary-600 dark:text-primary-400']">
      AIRI — Tauri
    </h1>
    <p :class="['mt-4 text-sm', 'text-gray-500 dark:text-gray-400']">
      Health: {{ healthStatus }}
    </p>
  </div>
</template>
