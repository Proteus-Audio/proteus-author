<template>
  <div class="title center">
    <div class="vertical-center">
      <div class="icon">
        <img :src="logo" alt="Proteus" />
      </div>
      <h1 class="h1">Proteus.play</h1>
    </div>
  </div>
</template>

<script setup lang="ts">
  import logo from "../../assets/images/icon.png"
</script>

<style lang="scss" scoped>
.center {
  text-align: center;
  height: 8em;
  display: grid;

  .vertical-center {
    display: flex;
    vertical-align: middle;
    align-content: center;
    align-items: center;
    width: fit-content;
    margin: auto;
    .h1 {
      display: inline-block;
      padding-left: 0.5em;
    }
    .icon {
      display: inline-block;
      width: 45px;

      img {
        width: 100%;
      }
    }
  }
}
</style>
