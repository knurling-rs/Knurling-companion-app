# Knurling companion app

This is a companion app built with Vite + Vue + Tauri, to track the data send by HC-SR04 sensor.

Tauri 🐂, the back-end part, is built in VITE. You can find all the files in `src-tauri`.

`src` contain the front-end files.

It has two modes, a DEMO mode if you don't have a sensor, that spawn random points between 20 and 500, and a LIVE mode when working with your sensor.


<p align="center">
    <img src=./demo_live.gif width="400"/>
</p>

[Btleplug](https://docs.rs/btleplug/0.8.0/btleplug/) is inserted into a `#[tauri::command]` to receive data. 
## Dependencies

Sytem dependencies for Tauri are described [here](https://tauri.studio/en/docs/getting-started/intro)

## Small tweaks

All the tweaks for the chart can be found in src/components/MyChart.vue, 
`const options = ref<ChartOptions<'line'>>` and `const chartData = computed<ChartData<'line'>>(()`

In particular, if you love the look of a Bezier function, change the `tension` value:

```javascript
const chartData = computed<ChartData<'line'>>(() => ({
    tension: 0.1,
```
<p align="center">
    <img src=./bezier_demo.gif width="400"/>
</p>

## To run the project:

In a terminal, run the command:
1. npm run dev

In another terminal, run:

2. npm run tauri dev

This needs to be done **in this order**!