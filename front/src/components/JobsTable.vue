<template>
<div class="table-responsive">
    <table class="table table-striped table-sm">
      <thead>
        <tr>
          <th scope="col">ID</th>
          <th scope="col">Created At</th>
          <th scope="col">Executed at</th>
          <th scope="col">Command</th>
          <th scope="col">Args</th>
          <th scope="col">Output</th>
          <th scope="col">Agent ID</th>
        </tr>
      </thead>
      <tbody v-for="job in jobs" :key="job.id">
            <jobRow :job="job"></jobRow>
      </tbody>
    </table>
  </div>
</template>

<script>
import JobRow from './JobRow.vue';
export default{
    props: {
        jobs: Array
    },
    components: { JobRow }
}
</script>