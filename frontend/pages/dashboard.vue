<template>
  <div class="dashboard">
    <DateSelector></DateSelector>
    <div class="displays">
      <SingleValueBox big-value="230$" value="2$" variation="increase" title="Revenue"></SingleValueBox>
      <SingleValueBox big-value="612$" value="4000$" variation="decrease" title="Revenue missed"></SingleValueBox>
      <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
      <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
      <SingleValueBox big-value="20" value="5" variation="increase" title="Clients served"></SingleValueBox>
    </div>
    <div class="buttons">
      <a href="/edit"
        ><Button title="Edit Data"><SettingsIcon /></Button
      ></a>
      <a href="/garage"
        ><Button title="View Garage"><GarageIcon /></Button
      ></a>
    </div>
  </div>
</template>

<style scoped lang="scss">
.buttons {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 2rem;
}

.displays {
  display: flex;
  flex-direction: row;
  gap: 2rem;
  flex-wrap: wrap;
}

.dashboard {
  display: flex;
  flex-direction: column;
  margin: 2rem 2rem;
  height: 100%;
  gap: 2rem;
}
</style>
