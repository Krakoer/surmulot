<template>
    <h1>About {{agent.id}}</h1>
    <JobsTable :jobs="agentJobs"></JobsTable>
</template>

<script setup>
import { useJobsStore } from '@/stores/jobs';
import JobsTable from "@/components/JobsTable.vue"
const props = defineProps({
    agent: Object
})

const jobStore = useJobsStore();

const agentJobs = jobStore.getByAgent(props.agent.id)

</script>
