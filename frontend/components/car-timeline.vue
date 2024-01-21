<template>
  <div class="car-timeline">
    <div class="elements">
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
      <div class="element">
        <TimelineElement :percent_start="0" :percent_end="100"></TimelineElement>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.elements {
  display: flex;
  flex-direction: row;
  justify-content: space-between;

  & .element {
    width: 100%;
    height: 100%;
  }
}
</style>
