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

<script setup>

import Header from "@/components/Header.vue"
import NavBar from "@/components/NavBar.vue"
import {useAgentsStore} from "@/stores/agents"
import {useJobsStore} from "@/stores/jobs"
import { onMounted } from "vue";

const agentsStore = useAgentsStore();
const jobsStore = useJobsStore();

onMounted(async ()  => {
    await agentsStore.fetchAgents();
    await jobsStore.fetchJobs();
  }
)
</script>

<style>
body {
  font-size: .875rem;
}
</style>
