<template>
<LineChart :chartData="chartData" :options="options" />
</template>

<script lang="ts">

import { defineComponent, computed, toRefs, ref } from 'vue'
import { LineChart } from "vue-chart-3";
import { Chart, ChartData, ChartOptions, registerables } from "chart.js";

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
    const options = ref<ChartOptions<'line'>>({
      normalized: true,
      parsing: false,
      responsive: true,
      spanGaps: false,
      scales: {  
        x: {
          display:false,
        },
        y: {
          ticks: {
                stepSize: 50
            },

          title: {
              display: true,
              text: 'Centimeters',
              font: {
              family: 'Helvetica',
              size: 20,
              weight: 'bold',
              lineHeight: 1,
            },
          },
          grid: {
            drawBorder: false,
            color: function(context) {
              if (context.tick.value > 250) {
                return "#80c904";
              } else if (context.tick.value < 250) {
                return "#ff0000";
              }
              return '#000000';
            },
          },
        },
      },
      elements: {
         line: {
            borderWidth: 2
          },
          point: {
              radius: 0.5,
              borderColor: "#ffffff",
          },  
           
      },
      line: {},
    });

    const chartData = computed<ChartData<'line'>>(() => ({
      labels: chartSensorData.value.map((_, i) => i),
      datasets: [
        {
          label: 'Distance',
          data: chartSensorData.value.map((y: any, x) => ({x, y})),
          fill: true,
          borderColor: 'rgb(150, 46, 5)',
          // change to 0.1 if getting seasick
          tension: 1.3,
        },
        {
          legend: {
            display: false,
          },
          label: '... Too close!',
          data: chartSensorData.value.map((_, x) => ({x, y: 250})),
          fill: false,
          borderDash: [10,5],
          borderColor: 'rgb(255, 0, 0)',
        }
      ]
    }));
    return { chartData, options};
  },
})
</script>
