<template>
  <div style="width: 400px">
    <DoughnutChart ref="doughnutRef" :chartData="testData" :options="options" />
  </div>
</template>

<script lang='ts'>
import { computed, defineComponent, ref } from "vue";
import { DoughnutChart } from "vue-chart-3";
import { Chart, ChartData, registerables } from "chart.js";

Chart.register(...registerables);
export default defineComponent({
  name: 'MyChart',
  components: {
    DoughnutChart
  },
  setup() {
    const dataValues = ref([30, 40, 60, 70, 5]);
    const doughnutRef = ref();
    const testData = computed<ChartData<"doughnut">>(() => ({
      labels: ["Paris", "Nîmes", "Toulon", "Perpignan", "Autre"],
      datasets: [
        {
          data: dataValues.value,
          backgroundColor: [
            "#77CEFF",
            "#0079AF",
            "#123E6B",
            "#97B0C4",
            "#A5C8ED",
          ],
        },
      ],
    }));
    const options =  {};
    return {
      testData,
      options,
      doughnutRef,
      dataValues,
    };
  },

})
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c5850;
  margin-top: 60px;
}
</style>
