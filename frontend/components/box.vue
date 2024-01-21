<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>
<template>
  <div class="box">
    <div class="title">{{ title }}</div>
    <slot />
  </div>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
.box {
  width: 100%;
  height: 100%;
  .title {
    color: map-get($map: $themes, $key: grey-text);
  }
  border-radius: 5px;
  background-color: white;
  padding: 5px;
  box-sizing: border-box;
}
</style>
