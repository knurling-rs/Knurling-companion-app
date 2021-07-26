<template>
  <h1>{{ "Important information from your 🐿️" }}</h1>
  <!--  <button type="button" @click="count++">count is: {{ count }}</button> -->
  <p>
    Can I have a measurement ? 
    {{msg}}
  </p>
</template>

<script lang="ts">
import {defineComponent } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'

export default defineComponent({
  name: 'HelloWorld',
  async mounted(){ 
    await invoke('init_process');
    listen('distance_emitter', x => this.msg = (x as any).payload as string + " cm")
    //    listen('distance_emitter', x => this.msg = x as unknown as string)

    },
  data(){return {msg: "" };}
})

</script>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>

