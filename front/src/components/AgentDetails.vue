<template>
    <NewJobModal :agent_id="agent.id"></NewJobModal>
    <div class="d-flex  flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
        <h1 class="h2">Agent {{agent.id}}</h1>
        <div class="btn-toolbar mb-2 mb-md-0 mx-5">
            <button type="button" class="btn btn-sm btn-outline-primary" data-bs-toggle="modal"
                data-bs-target="#newjobModal">
                New Job
            </button>
        </div>
    </div>
        <JobsTable :jobs="agentJobs(agent.id)"></JobsTable>
</template>

<script setup>
import { useJobsStore } from '@/stores/jobs';
import JobsTable from "@/components/JobsTable.vue"
import NewJobModal from './NewJobModal.vue';

const props = defineProps({
    agent: Object
})

const jobStore = useJobsStore();

const agentJobs = jobStore.getByAgent
</script>
