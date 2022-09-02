<template>
    <div class="utils">
        <UtilSave />
        <UtilLoad />
        <UtilKeyStrokes />
    </div>
</template>

<script setup lang="ts">
import UtilSave from './UtilSave.vue';
import UtilLoad from './UtilLoad.vue';
import UtilKeyStrokes from './UtilKeyStrokes.vue';

</script>

<style scoped>
.utils {
    display: none;
}
</style>