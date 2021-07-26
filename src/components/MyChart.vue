
<script lang="ts">
import { defineComponent } from 'vue';
import { DoughnutChart } from 'vue-chart-3';

export default defineComponent({
  name: 'MyChart',
  components: { DoughnutChart },
  props: {
    chartData: Array,
  },
  mounted() {
    let testData = {
      datasets: [
        {
          data: this.chartData,
          backgroundColor: ['#77CEFF', '#0079AF', '#123E6B', '#97B0C4', '#A5C8ED'],
        },
      ],
    };

    return { testData };
  },
});
</script>
