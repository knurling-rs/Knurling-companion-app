<template>
  <img alt="Rust logo" src="./assets/groundhog_bike.png" size=200/>
  <h2> ⚠️ Important security information ⚠️ </h2>
  <h3> ... brought to you by a rodent 🐿️</h3>
  <div class="chart-container" style="margin:0 auto; width:70vw">
    <MyChart :chartSensorData="dataValues"/>
  </div>

</template>

<script lang="ts">
import { defineComponent } from 'vue'
import MyChart from './components/MyChart.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

export default defineComponent({
  name: 'App',
  components: {
    MyChart,
  },
  data(){
      return {
          msg: "",
          dataValues: new Array(40).fill(10),
      };
  },
  async mounted(){
    await invoke('init_process');
    listen("distance_emitter", x => {
        this.dataValues.push(Number((x as any).payload as string));
        if(this.dataValues.length > 40) {
          this.dataValues.shift();
        } 
    });
  }
});
</script>

<style scoped>
img{
  display: block;
  margin-left: auto;
  margin-right: auto;
  margin-top: 60px;

}
h2, h3 {
  text-align: center;
  color: rgb(150, 46, 5);
  margin-top: 1em;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

</style>