<template>
  <!-- <div class="lds-ring">
    <div></div>
    <div></div>
    <div></div>
    <div></div>
  </div> -->
  <div class="loader">
    <div class="lds-pixel-ring">
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
      <div></div>
    </div>
    <div v-if="message" class="message">{{ message }}</div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  message?: string
}

defineProps<Props>()
</script>

<style lang="scss" scoped>
@keyframes fade {
  0% {
    opacity: 0;
  }
  25% {
    opacity: 1;
  }
  50% {
    opacity: 1;
  }
  90% {
    opacity: 0;
  }
}

.loader {
  display: inline-block;
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  z-index: 20;
  background-color: rgba(100, 100, 100, 0.6);
  display: flex;
  flex-direction: column;
  gap: 0.5em;
  justify-content: center;
  align-items: center;

  .message {
    color: white;
    font-size: 0.8em;
    text-align: center;
  }

  .lds-pixel-ring {
    display: inline-block;
    position: relative;
    width: 2em;
    height: 2em;
    // border: 1px solid white;

    div {
      box-sizing: border-box;
      display: block;
      position: absolute;
      width: 0.5em;
      height: 0.5em;
      background-color: white;
      animation: fade 1.4s cubic-bezier(0.5, 0, 0.5, 1) infinite;

      &:nth-child(1),
      &:nth-child(2) {
        top: 0;
      }

      &:nth-child(3),
      &:nth-child(8) {
        top: 0.5em;
      }

      &:nth-child(4),
      &:nth-child(7) {
        top: 1em;
      }

      &:nth-child(5),
      &:nth-child(6) {
        top: 1.5em;
      }

      &:nth-child(1),
      &:nth-child(6) {
        left: 1em;
      }

      &:nth-child(2),
      &:nth-child(5) {
        left: 0.5em;
      }

      &:nth-child(7),
      &:nth-child(8) {
        left: 1.5em;
      }

      &:nth-child(1) {
        animation-delay: 0s;
      }
      &:nth-child(2) {
        animation-delay: -0.1s;
      }
      &:nth-child(3) {
        animation-delay: -0.2s;
      }
      &:nth-child(4) {
        animation-delay: -0.3s;
      }
      &:nth-child(5) {
        animation-delay: -0.4s;
      }
      &:nth-child(6) {
        animation-delay: -0.5s;
      }
      &:nth-child(7) {
        animation-delay: -0.6s;
      }
      &:nth-child(8) {
        animation-delay: -0.7s;
      }
      &:nth-child(9) {
        animation-delay: -0.8s;
      }
      &:nth-child(10) {
        animation-delay: -0.9s;
      }
      &:nth-child(11) {
        animation-delay: -1s;
      }
      &:nth-child(12) {
        animation-delay: -1.1s;
      }
    }
  }
}
.lds-ring {
  display: inline-block;
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  z-index: 2;
  background-color: rgba(100, 100, 100, 0.6);

  div {
    left: calc(50% - 1em - 5px);
    top: calc(50% - 1em - 5px);
    box-sizing: border-box;
    display: block;
    position: absolute;
    width: 2em;
    height: 2em;
    margin: 5px;
    border: 5px solid #fff;
    border-radius: 50%;
    animation: lds-ring 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
    border-color: #fff transparent transparent transparent;

    &:nth-child(1) {
      animation-delay: -0.45s;
    }
    &:nth-child(2) {
      animation-delay: -0.3s;
    }
    &:nth-child(3) {
      animation-delay: -0.15s;
    }
  }
}
@keyframes lds-ring {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
