<template>
    <div>
        <Title title="Agents"></Title>
        <AgentsTable :agents="agents"></AgentsTable>
    </div>
</template>
  
<script>
import Title from "@/components/Title.vue"
import AgentsTable from "@/components/AgentsTable.vue"

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
    async created(){
        const resp = await fetch("api/agents")
        const data = await resp.json()
        this.agents = data
    }
}
</script>
  