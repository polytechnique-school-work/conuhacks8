<template>
  <footer>
    <div class="about-us">
      About us
      <div class="frontend">
        Frontend
        <div class="team">
          <div>Gabriel Landry</div>
          <div>Luckas Rakotondrafara</div>
        </div>
      </div>
      <div class="backend">
        Backend
        <div class="team">
          <div>Marc-Antoine Manningham</div>
          <div>Gaëtan Florio</div>
        </div>
      </div>
    </div>

    <div class="developped">
      <div>Developped for ConuHacks VIII</div>
      <div class="team">
        <div class="github">
          <div><GithubLogo /></div>
          <div>GitHub</div>
        </div>
        <div class="devpost">
          <div><DevpostLogo /></div>
          <div>Devpost</div>
        </div>
      </div>
    </div>
  </footer>
</template>

<style scoped lang="scss">
footer {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  background-color: #f5f5f5;
  font-family: "Roboto", sans-serif;
  font-size: 1.2rem;
  color: #333;
}
</style>
