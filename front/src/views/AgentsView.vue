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
            agents : [
                {
                    id: 1,
                    created_at: "vouvou",
                    last_seen: "bonjoue"
                }
            ]
        }
    }
}
</script>
  