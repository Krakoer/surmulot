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
  
<script>
import Title from "@/components/Title.vue"
import AgentsTable from "@/components/AgentsTable.vue"
import AgentDetails from "@/components/AgentDetails.vue"
import {useAgentsStore} from "@/stores/agents"


export default {
    name: 'AgentsView',
    components:{
        Title,
        AgentsTable,
        AgentDetails,
    },
    data(){
        return {
            agents : [],
            selected_agent: null
        }
    },
    mounted(){
        const agentStore = useAgentsStore()
        this.agents = agentStore.getAll
    },
    methods: {
        selectAgent(new_id){
            this.selected_agent = this.agents.find((a) => a.id == new_id)
        }
    }
}
</script>
  