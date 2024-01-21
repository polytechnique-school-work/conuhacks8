<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>
<template>
  <div class="button hover">
    <div class="content"><slot /></div>
    <div>{{ title }}</div>
  </div>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
.content {
  height: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
}
.button {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  height: 60px;
  width: 100%;
  color: white;
  background-color: map-get($map: $themes, $key: primary);
  border-radius: 5px;
  font-size: 1.5rem;
  text-align: center;
  gap: 0.5rem;
  .svg {
    align-items: center;
  }
}
</style>
