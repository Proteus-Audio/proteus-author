<template>
  <div class="grid h-32 place-items-center text-center">
    <div class="mx-auto flex w-fit items-center">
      <div class="inline-block w-[45px]">
        <img :src="logo" alt="Proteus" class="w-full" />
      </div>
      <h1 class="inline-block pl-2 text-2xl">Proteus.play</h1>
    </div>
  </div>
</template>

<script setup lang="ts">
import logo from '../../assets/images/icon.png'
</script>
