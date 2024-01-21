<script setup lang="ts">
  const router = useRouter();
  const route = useRoute();

  function getDayOfYear(date: Date): number {
    const startOfYear = new Date(date.getFullYear(), 0, 0);
    const timeDifference = date.getTime() - startOfYear.getTime();
    const days = Math.floor(timeDifference / (24 * 60 * 60 * 1000));

    return days;
  }

  function dayOfYearToDate(): string {
    const startOfYear = new Date(2022, 0, 0);
    const date = route.params.date as unknown as number;
    const timeStamp = startOfYear.getTime() + date * 24 * 3600 * 1000;
    const dateOf = new Date(timeStamp);
    return dateOf.toISOString().split('T')[0];
  }
  const pageDate = dayOfYearToDate();

  function onClick(event: any) {
    const date: number = getDayOfYear(new Date(event.target.value));
    router.replace('/dashboard/' + date);
  }

</script>

<template>
  <Box class="dateBox" title="Date">
    <div class="info">
      <input ref="date" type="date" id="start" name="trip-start" :value="pageDate" @input="onClick" />
    </div>
  </Box>
</template>

<style lang="scss" scoped>
.dateBox {
  width: 350px;
  height: 60px;

  .info {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;

    & input {
      // reset style
      border: none;
      outline: none;
      background: none;
      // custom style
      font-size: 1rem;
      font-family: inherit;
      // remove calendar
    }
  }

  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
