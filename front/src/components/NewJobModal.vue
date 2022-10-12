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
                <div class="mb-3">
                    <label for="jobCommand" class="form-label">Command</label>
                    <input v-model="command" type="text" class="form-control" id="jobCommand">
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

<script>
export default{
    data(){
        return {
            command: ''
        }
    },
    methods:{
        triggerNewJob(){
            console.log(this.command)
        }
    }
}
</script>