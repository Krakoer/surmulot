<template>
  <div class="modal fade" id="newjobModal" tabindex="-1" aria-labelledby="newjobModalLabel" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5" id="newjobModalLabel">Create new job</h1>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="triggerNewJob">
            <div class="form-group">
              <label for="jobCommand" class="form-label">Command</label>
              <input v-model="command" type="text" class="form-control" id="jobCommand">
            </div>
            <div class="form-group mb-3">
              <label for="exampleFormControlSelect1">Agent ID</label>

              <select v-model="selected_id" class="form-control" id="exampleFormControlSelect1">
                <option v-for="id in agents_ids" :key="id">{{ id }}</option>
              </select>
            </div>
            <button type="submit" class="btn btn-primary" data-bs-dismiss="modal">Run Job</button>
          </form>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useAgentsStore } from "@/stores/agents"
import { useJobsStore } from "@/stores/jobs"
import {ref} from 'vue'

const jobStore = useJobsStore();
const agentsStore = useAgentsStore();
const command = ref('');
const agents_ids = agentsStore.getIds;
const selected_id = ref('');

async function triggerNewJob() {
  await jobStore.createJob(command.value, selected_id.value)
  await jobStore.fetchJobs()
}

</script>