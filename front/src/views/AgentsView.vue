<template>
    <div>
        <Title title="Agents"></Title>
        <div class="pt-3 pb-2 mb-3 border-bottom">
        <AgentsTable :agents="agents" @changeAgentSelected="selectAgent"></AgentsTable>
        </div>
        <div v-if="selected_agent != null">
        <AgentDetails  :agent="selected_agent"></AgentDetails>
    </div>
    </div>
</template>
  
<script setup>
import Title from "@/components/Title.vue"
import AgentsTable from "@/components/AgentsTable.vue"
import AgentDetails from "@/components/AgentDetails.vue"
import {useAgentsStore} from "@/stores/agents"
import {ref} from "vue"


const agentStore = useAgentsStore()
const agents = agentStore.getAll
const selected_agent = ref(null)

function selectAgent(new_id){
    if(selected_agent.value != null && new_id == selected_agent.value.id){
        selected_agent.value = null
        return
    }
    selected_agent.value = agents.find((a) => a.id == new_id)
}

</script>
  