<template>
    <div class="transport">
        <el-button @click="play">Play</el-button>
        <el-button @click="pause">Pause</el-button>
        <el-button @click="stop">Stop</el-button>
    </div>
</template>

<script setup lang="ts">

const play = () => {
    const players = document.getElementsByClassName('playable') as HTMLCollectionOf<HTMLAudioElement>;
    for (let i = 0; i < players.length; i++) {
        const player = players[i];
        player.play();
    }
}

const pause = () => {
    const players = document.getElementsByClassName('player') as HTMLCollectionOf<HTMLAudioElement>;
    for (let i = 0; i < players.length; i++) {
        const player = players[i];
        player.pause();
    }
}

const stop = () => {
    const players = document.getElementsByClassName('player') as HTMLCollectionOf<HTMLAudioElement>;
    for (let i = 0; i < players.length; i++) {
        const player = players[i];
        player.pause();
        player.currentTime = 0;
    }
}
</script>

<style scoped>
.transport {
    margin-bottom: 1em;
}
</style>