<template>
chartSensorData: {{chartSensorData}}<br/>
<br/>
chartData: {{chartData}}
MyChart.vue <LineChart :chartData="chartData"/>
</template>

<script lang="ts">

import { defineComponent, computed, toRefs } from 'vue'
import { LineChart } from "vue-chart-3";
import { Chart, ChartData, registerables } from "chart.js";

Chart.register(...registerables);

export default defineComponent({
  name: 'MyChart',
  components: { LineChart },
  props: {
    chartSensorData: {
      type: Array,
      required: true
    },
  },  
  setup(props) {
    const {chartSensorData} = toRefs(props);
    const options = {};

    const chartData = computed<ChartData<'line'>>(() => ({
      labels: chartSensorData.value.map((_, i) => i),
      datasets: [{
        label: 'Dataset',
        data: chartSensorData.value.map((y: any, x) => ({x, y})),
        fill: true,
        borderColor: 'rgb(75, 80, 192)',
      }]
    }));
    return { chartData };
  },
})
</script>
