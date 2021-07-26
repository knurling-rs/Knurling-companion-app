<template>
  <img alt="Rust logo" src="./assets/groundhog_bike.png" size=200/>
  <p>
 {{dataValues}}
 </p>
 <MyChart />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import MyChart from './components/MyChart.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'

export default defineComponent({
  name: 'App',
  components: {
    MyChart,
  },
  data(){
      return {
          msg: "",
          dataValues: [0,0,0,0,0],
      };
  },
  async mounted(){
    await invoke('init_process');
    listen("distance_emitter", x => {
        this.dataValues.push(Number((x as any).payload as string));
        if(this.dataValues.length > 10) {
          this.dataValues.shift();
        } 
    });
  }
});
</script>


<style scoped>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
