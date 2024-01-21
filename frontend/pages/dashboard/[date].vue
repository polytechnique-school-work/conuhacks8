<script lang="ts">
// import { useRoute } from "vue-router";
// const route = useRoute();
// console.log(route);
const data: {
  [key: string]: {
    revenue: number;
    revenue_variation: number;
    revenue_missed: number;
    revenue_missed_variation: number;
    clients_served: number;
    clients_served_variation: number;
  };
} = {
  "2024-01-01": {
    revenue: 230,
    revenue_variation: -15,
    revenue_missed: 612,
    revenue_missed_variation: 57,
    clients_served: 20,
    clients_served_variation: 5,
  },
};

export default {
  methods: {
    async changeDate(value: string) {
    },
    getData(): {
      revenue: number;
      revenue_variation: number;
      revenue_missed: number;
      revenue_missed_variation: number;
      clients_served: number;
      clients_served_variation: number;
    } {
      return data[this.$route.params.date as string];
    },
    receiveDate(date: string) {
      navigateTo(`/dashboard/${date}`);
    },
  },
};
</script>

<template>
  <div class="wrapper">
    <div class="dashboard">
      <DateSelector @date="receiveDate"></DateSelector>
      <div v-if="getData != null" class="displays">
        <SingleValueBox big-value="230$" value="2$" variation="increase" title="Revenue"></SingleValueBox>
        <SingleValueBox big-value="612$" value="4000$" variation="decrease" title="Revenue missed"></SingleValueBox>
        <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
        <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
        <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
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
