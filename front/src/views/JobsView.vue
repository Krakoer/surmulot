<template>
    <NewJobModal></NewJobModal>
    <div>
        <div
            class="d-flex  flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
            <h1 class="h2">Jobs</h1>
            <div class="btn-toolbar mb-2 mb-md-0 mx-5">
          <button type="button" class="btn btn-sm btn-outline-primary" data-bs-toggle="modal" data-bs-target="#newjobModal">
            New Job
          </button>
        </div>
        </div>
        <JobsTable :jobs="jobs"></JobsTable>
    </div>
</template>
  
<script>
import JobsTable from "@/components/JobsTable.vue"
import NewJobModal from "@/components/NewJobModal.vue"
import {useJobsStore} from "@/stores/jobs"

export default {
    name: 'JobsView',
    components:{
        JobsTable,
        NewJobModal
    },
    data(){
        return {
            jobs : []
        }
    },
    mounted(){
        const jobStore = useJobsStore()
        this.jobs = jobStore.getAll
    }
}
</script>
  