<template>
<div class="table-responsive">
    <table class="table table-striped table-sm">
      <thead>
        <tr>
          <th scope="col">ID</th>
          <th scope="col">Created At</th>
          <th scope="col">Last seen At</th>
          <th scope="col">Username</th>
          <th scope="col">Hostname</th>
        </tr>
      </thead>
      <tbody v-for="agent in agents" :key="agent.id">
            <AgentRow :agent="agent" v-on:click="selectAgent(agent.id)"></AgentRow>
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
    components: { AgentRow },
    data:  () => {
      return {
        selected_agent: ""
      }
    },
    methods: {
      selectAgent(id) {
        this.$emit('changeAgentSelected', id)
      }
    },
    emits: ['changeAgentSelected'],
}
</script>