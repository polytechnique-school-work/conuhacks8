<script setup lang="ts">
type Data = {
  revenue: number,
  delta_revenue: number,
  revenue_miss: number,
  delta_revenue_miss: number,
  clients_served: number,
  delta_clients_served: number,
  vehicules_decline: Array<5>,
  delta_vehicules_decline: Array<5>,
  vehicules_served: Array<5>,
  delta_vehicules_served: Array<5>,
};
const route = useRoute();

const response = await fetch("https://api.gagolino.com/api/day/2022/" + route.params.date);
const data: Data = await response.json();

</script>

<template>
  <div class="wrapper">
    <div class="dashboard">
      <DateSelector></DateSelector>
      <div v-if="data" class="displays">
        <div class="row">
          <SingleValueBox :big-value="data.revenue + '$'" :value="Math.abs(data.delta_revenue) + '$'" :variation="data.delta_revenue <= 0 ? 'increase': 'decrease'" title="Revenue" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.revenue_miss + '$'" :value="Math.abs(data.delta_revenue_miss) + '$'" :variation="data.delta_revenue_miss <= 0 ? 'increase': 'decrease'" title="Revenue Miss" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.clients_served + ' clients'" :value="Math.abs(data.delta_clients_served) + ' clients'" :variation="data.delta_clients_served <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
        </div>
        
        <div class="row">
          <SingleValueBox :big-value="data.vehicules_served[0] + ''" :value="Math.abs(data.vehicules_served[0]) + ''" :variation="data.delta_vehicules_served[0] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_served[1] + ''" :value="Math.abs(data.vehicules_served[1]) + ''" :variation="data.delta_vehicules_served[1] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_served[2] + ''" :value="Math.abs(data.vehicules_served[2]) + ''" :variation="data.delta_vehicules_served[2] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_served[3] + ''" :value="Math.abs(data.vehicules_served[3]) + ''" :variation="data.delta_vehicules_served[3] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_served[4] + ''" :value="Math.abs(data.vehicules_served[4]) + ''" :variation="data.delta_vehicules_served[4] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
        </div>
        <div class="row">
          <SingleValueBox :big-value="data.vehicules_decline[0] + ''" :value="Math.abs(data.vehicules_decline[0]) + ''" :variation="data.delta_vehicules_decline[0] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_decline[1] + ''" :value="Math.abs(data.vehicules_decline[1]) + ''" :variation="data.delta_vehicules_decline[1] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_decline[2] + ''" :value="Math.abs(data.vehicules_decline[2]) + ''" :variation="data.delta_vehicules_decline[2] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_decline[3] + ''" :value="Math.abs(data.vehicules_decline[3]) + ''" :variation="data.delta_vehicules_decline[3] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
          <SingleValueBox :big-value="data.vehicules_decline[4] + ''" :value="Math.abs(data.vehicules_decline[4]) + ''" :variation="data.delta_vehicules_decline[4] <= 0 ? 'increase': 'decrease'" title="Client served" class="box"></SingleValueBox>
        </div>
      </div>
      <div v-else>Aucune donnée n'a été trouvée pour ce jours là</div>
      <div class="buttons">
        <NuxtLink to="/garage"
          ><Button title="View Garage"><SettingsIcon /></Button
        ></NuxtLink>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  display: flex;
  height: 100%;
  width: 100%;
  align-items: center;
  justify-content: center;
  & .dashboard {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 75%;
    gap: 2em;
  }
}
.row {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  gap: 1em;
}
.box {
  width: 14em;
}
.displays {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 1em;
}
</style>
