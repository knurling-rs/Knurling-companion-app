<template>
  <img alt="Rust logo" src="./assets/groundhog_bike.png" size=200/>
 <HelloWorld msg="A biking Sensor app" />
 <MyChart />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import MyChart from './components/MyChart.vue'

export default defineComponent({
  name: 'App',
  components: {
    HelloWorld,
    MyChart,
  }
})
</script>


<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
