<script setup lang="ts">
type Data = {
  revenue: number,
  delta_revenue: number,
  revenue_miss: number,
  delta_revenue_miss: number,
  clients_served: number,
  delta_clients_served: number,
};
const route = useRoute();

const { data: day } = await useFetch("http://127.0.0.1:6942/api/day/2022/" + route.params.date);
const data = day as unknown as Data;

const receiveDate = null;
</script>

<template>
  <div class="wrapper">
    <div class="dashboard">
      <DateSelector :sendDate="receiveDate"></DateSelector>
      <div v-if="data" class="displays">
        <SingleValueBox :big-value="data.revenue + '$'" :value="Math.abs(data.delta_revenue) + '$'" :variation="data.delta_revenue <= 0 ? 'increase': 'decrease'" title="Revenue"></SingleValueBox>
        <SingleValueBox :big-value="data.revenue_miss + '$'" :value="Math.abs(data.delta_revenue_miss) + '$'" :variation="data.delta_revenue_miss <= 0 ? 'increase': 'decrease'" title="Revenue Miss"></SingleValueBox>
        <SingleValueBox :big-value="data.clients_served + ' clients'" :value="Math.abs(data.delta_clients_served) + ' clients'" :variation="data.delta_clients_served <= 0 ? 'increase': 'decrease'" title="Revenue Miss"></SingleValueBox>
      </div>
      <div v-else>Aucune donnée n'a été trouvée pour ce jours là</div>
      <div class="buttons">
        <NuxtLink to="/edit"
          ><Button title="Edit Data"><SettingsIcon /></Button
        ></NuxtLink>
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
    gap: 2rem;
    // max-width: 1200px;
    & .displays {
      display: grid;
      grid-template-columns: auto auto auto;
      gap: 2rem;
      width: 100%;
    }

    & .buttons {
      display: grid;
      grid-template-columns: auto auto auto;
      gap: 2rem;
      width: 100%;
    }
  }
}
</style>
