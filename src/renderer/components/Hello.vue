<script setup lang="ts">
const msg = 'Electron + Vue3 template';
</script>

<template>
  <div id="hello">
    <div class="small">
    <img src="http://vuejs.org/images/logo.png">
    </div>
    <h1>{{ msg }}</h1>
  </div>
</template>

<style scoped>
#hello {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

h1, h2 {
  font-weight: normal;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}

.small {
  max-width: 150px;
  margin: auto;
}

img {
  width: 100%
}
</style>
