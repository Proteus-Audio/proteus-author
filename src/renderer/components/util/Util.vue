<template>
    <div class="utils">
        <UtilSave />
        <UtilSaveAs />
        <UtilLoad />
        <UtilKeyStrokes />
    </div>
</template>

<script setup lang="ts">
import UtilSave from './UtilSave.vue';
import UtilLoad from './UtilLoad.vue';
import UtilKeyStrokes from './UtilKeyStrokes.vue';
import UtilSaveAs from './UtilSaveAs.vue';

</script>

<style scoped>
.utils {
    display: none;
}
</style>