<template>
    <div class="fx-icon" @click.stop="doSomething">
        {{type}}
        {{randomText}}
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Effect from '../../typings/effects';

interface Props {
  type: Effect;
}

const props = defineProps<Props>();

const randomText = ref("");

const doSomething = () => {
    // randomText.value = "KAJH";
}
</script>

<style lang="scss" scoped>
.fx-icon {
    width: 100%;
    height: 100%;
    background-color: rgb(69, 69, 69);
    margin-top: 0em;
    border-radius: .5em;
    padding: 1em;
    color: white;
    display: block;
    overflow: hidden;
    transition: height .3s, margin .3s;
    
    &:hover {
        height: 110%;
        margin-top: -2.5%;
    }
}

</style>