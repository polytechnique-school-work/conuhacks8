<script setup lang="ts">
const props = defineProps<{
  type: string;
  amount: number;
  isTruck: boolean;
}>();
</script>

<template>
  <div class="card-container">
    <div class="card">
      <div class="text-container">
        {{ props.type }}
        <truck-icon v-if="props.isTruck" style="padding-left: 5px" />
        <car-icon v-else style="padding-left: 5px" />
      </div>
      <edit-card :amount="props.amount" class="edit-card" />
    </div>
  </div>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
.card {
  width: 100%;
  text-align: center;
  justify-content: space-between;
  display: flex;
  flex-direction: row;
}
.text-container {
  width: 100%;
  max-width: 280px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
  height: 60px;
  background-color: map-get($map: $themes, $key: grey-box);
  border-radius: 5px;
  color: map-get($map: $themes, $key: text);
}

.edit-card {
  flex: 1;
}

.card-container {
  display: flex;
  flex-direction: row;
  width: 100%;
}
</style>
