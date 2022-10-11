<template>
<div class="table-responsive">
    <table class="table table-striped table-sm">
      <thead>
        <tr>
          <th scope="col">ID</th>
          <th scope="col">Created At</th>
          <th scope="col">Last seen At</th>
        </tr>
      </thead>
      <tbody v-for="agent in agents" :key="agent.id">
            <AgentRow :agent="agent"></AgentRow>
      </tbody>
    </table>
  </div>
</template>

<script>
import AgentRow from './AgentRow.vue';
export default{
    props: {
        agents: Array
    },
    components: { AgentRow }
}
</script>