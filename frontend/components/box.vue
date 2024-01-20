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
  .title {
    color: map-get($map: $themes, $key: grey-text);
  }
  border-radius: 5px;
  background-color: white;
  padding: 5px;
}
</style>
