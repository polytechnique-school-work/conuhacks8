<template>
  <footer>
    <div class="about-us">
      About us
      <div class="wrapper">
        <div class="frontend">
          Frontend
          <div class="team grey">
            <div><a href="https://github.com/Sportek" target="_blank">Gabriel Landry</a></div>
            <div><a href="https://github.com/LuckasRakoto" target="_blank">Luckas Rakotondrafara</a></div>
          </div>
        </div>
        <div class="backend">
          Backend
          <div class="team grey">
            <div><a href="https://github.com/marcantoinem" target="_blank">Marc-Antoine Manningham</a></div>
            <div><a href="https://github.com/celidur" target="_blank">Gaëtan Florio</a></div>
          </div>
        </div>
      </div>
    </div>

    <div class="developped">
      Developped for ConuHacks VIII
      <div class="wrapper">
        <div class="team">
          <div class="links">
            <a href="https://github.com/polytechnique-school-work/conuhacks8" target="_blank">
              <div class="grey">
                <GithubLogo />
              </div>
              <div class="grey">GitHub</div>
            </a>
          </div>
        </div>
        <div class="team">
          <div class="links">
            <a href="https://devpost.com/software/todo-of2gv9" target="_blank">
              <div class="grey">
                <DevpostLogo />
              </div>
              <div class="grey">Devpost</div>
            </a>
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
  & a {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
}

.grey {
  color: map-get($map: $themes, $key: grey-text);
}
</style>
