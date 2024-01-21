<script lang="ts">
// const props = defineProps<{
//   percent_start: number;
//   percent_end: number;
// }>();
</script>
<template>
  <div class="element">
    <div class="element-inner"></div>
  </div>
</template>

<style scoped lang="scss">
.element {
  height: 100%;
  width: 100%;
  background-color: white;
}
.element-inner {
  position: absolute;
  height: 100%;
  background-color: red;
  border-radius: 5px;
  padding: 5px;
}
</style>
