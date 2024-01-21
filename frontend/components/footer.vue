<template>
  <footer>
    <div class="about-us">
      About us
      <div class="wrapper">
        <div class="frontend">
          Frontend
          <div class="team grey">
            <div>Gabriel Landry</div>
            <div>Luckas Rakotondrafara</div>
          </div>
        </div>
        <div class="backend">
          Backend
          <div class="team grey">
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
            <div class="grey">
              <a href="https://github.com/polytechnique-school-work/conuhacks8"><GithubLogo /></a>
            </div>
            <div class="grey">GitHub</div>
          </div>
        </div>
        <div class="team">
          <div class="links">
            <div class="grey">
              <a href="https://devpost.com/software/606404/joins/DinLic1vr1UEJa2hJnn5mA"><DevpostLogo /></a>
            </div>
            <div class="grey">Devpost</div>
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
  background-color: map-get($map: $themes, $key: background-dark);
  padding: 2rem;
  .wrapper {
    flex-direction: row;
    display: flex;
    div {
      padding: 0.5rem;
      .team {
        display: flex;
        flex-direction: column;
        padding-left: 0.5rem;
      }
    }
  }
}
.links {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.grey {
  color: map-get($map: $themes, $key: grey-text);
}
</style>
