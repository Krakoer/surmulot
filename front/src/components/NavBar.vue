<template>
    <nav id="navBar" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse">
        <div class="position-sticky pt-3">
            <ul class="nav flex-column">
                <li class="nav-item">
                    <router-link class="nav-link" to="/">
                        <font-awesome-icon class="feather" icon="fa-solid fa-house" />
                        Dashboard
                    </router-link>
                </li>
                <li class="nav-item">
                    <router-link class="nav-link" to="/agents">
                        <font-awesome-icon class="feather" icon="fa-solid fa-person-military-rifle" />
                        Agents
                    </router-link>
                </li>
                <li class="nav-item">
                    <router-link class="nav-link" to="/jobs">
                        <font-awesome-icon class="feather" icon="fa-solid fa-screwdriver-wrench" />
                        Jobs
                    </router-link>
                </li>
            </ul>
        </div>
    </nav>
</template>

<script>
export default {
    name: "NavBar"
}
</script>

<style>
.sidebar{
  position: fixed;
  top: 0;
  /* rtl:raw:
  right: 0;
  */
  bottom: 0;
  /* rtl:remove */
  left: 0;
  z-index: 100; /* Behind the navbar */
  padding: 48px 0 0; /* Height of navbar */
  box-shadow: inset -1px 0 0 rgba(0, 0, 0, .1);
}

@media (max-width: 767.98px) {
  .sidebar {
    top: 5rem;
  }
}

.sidebar-sticky {
  position: relative;
  top: 0;
  height: calc(100vh - 48px);
  padding-top: .5rem;
  overflow-x: hidden;
  overflow-y: auto; /* Scrollable contents if viewport is shorter than content. */
}

.sidebar .nav-link {
  font-weight: 500;
  color: #333;
}

.sidebar .nav-link .feather {
  margin-right: 4px;
  color: #727272;
}

.sidebar .nav-link.router-link-active {
  color: #2470dc;
}

.sidebar .nav-link:hover .feather,
.sidebar .nav-link.router-link-active .feather {
  color: inherit;
}

.sidebar-heading {
  font-size: .75rem;
  text-transform: uppercase;
}


.feather {
  width: 16px;
  height: 16px;
  vertical-align: text-bottom;
}
</style>