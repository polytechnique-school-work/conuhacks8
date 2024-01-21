<script setup lang="ts">
const props = defineProps<{
  percent_start: number;
  percent_end: number;
}>();
</script>
<template>
  <div class="element">
    <div
      class="element-inner"
      :style="{
        top: percent_start + '%',
        height: percent_end - percent_start + '%',
      }"
    ></div>
  </div>
</template>

<style scoped lang="scss">
.element {
  height: 100%;
  width: 100%;
  background-color: white;
  position: relative;
}
.element-inner {
  position: absolute;
  width: 100%;
  background-color: #831e12;
  border-color: black;
  border-style: solid 1px;
}
</style>
