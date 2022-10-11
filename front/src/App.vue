<template>
  <Header></Header>
  <div class="container-fluid">
    <div class="row">
      <NavBar></NavBar>
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <router-view/>
      </main>
    </div>
  </div>
</template>

<script>

import Header from "@/components/Header.vue"
import NavBar from "@/components/NavBar.vue"

export default{
  components:{
    Header,
    NavBar
  }
}
</script>

<style>
body {
  font-size: .875rem;
}
</style>
