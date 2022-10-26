<template>
    <div>
        <Title title="Agents"></Title>
        <AgentsTable :agents="agents"></AgentsTable>
    </div>
</template>
  
<script>
import Title from "@/components/Title.vue"
import AgentsTable from "@/components/AgentsTable.vue"
import {useAgentsStore} from "@/stores/agents"


export default {
    name: 'AgentsView',
    components:{
        Title,
        AgentsTable
    },
    data(){
        return {
            agents : []
        }
    },
    mounted(){
        const agentStore = useAgentsStore()
        this.agents = agentStore.getAll
    }
}
</script>
  