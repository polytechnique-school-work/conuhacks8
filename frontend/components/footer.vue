<template>
  <footer>
    <div class="about-us">
      About us
      <div class="wrapper">
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
    </div>

    <div class="developped">
      Developped for ConuHacks VIII
      <div class="wrapper">
        <div class="team">
          <div class="links">
            <GithubLogo />
            <div>GitHub</div>
          </div>
        </div>
        <div class="team">
          <div class="links">
            <DevpostLogo />
            <div>Devpost</div>
          </div>
        </div>
      </div>
    </div>
  </footer>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
footer {
  color: white;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  background-color: map-get($map: $themes, $key: text);
  padding: 2rem;
  .wrapper {
    flex-direction: row;
    display: flex;
    div {
      padding: 0.5rem;
      display: flex;
      flex-direction: column;
      .team {
        display: flex;
        flex-direction: column;
        padding-left: 0.5rem;
        .links {
          display: flex;
          flex-direction: row;
        }
      }
    }
  }
}
</style>
