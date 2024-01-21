<script></script>
<template>
  <div class="garage">
    <!-- <DateSelector></DateSelector> -->

    <div class="hours">
      <div class="hour">7:00</div>
      <div class="hour">8:00</div>
      <div class="hour">9:00</div>
      <div class="hour">10:00</div>
      <div class="hour">11:00</div>
      <div class="hour">12:00</div>
      <div class="hour">13:00</div>
      <div class="hour">14:00</div>
      <div class="hour">15:00</div>
      <div class="hour">16:00</div>
      <div class="hour">17:00</div>
      <div class="hour">18:00</div>
      <div class="hour">19:00</div>
    </div>

    <div class="timelines">
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
      <div class="cartimeline"><CarTimeline /></div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.garage {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  padding: 2rem;
  box-sizing: border-box;

  & .hours {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin: 0.5rem;
  }
  & .timelines {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
    gap: 0.5rem;

    & .cartimeline {
      width: 100%;
      height: 100%;
      background-color: white;
      border-radius: 5px;
    }
  }
}
</style>
