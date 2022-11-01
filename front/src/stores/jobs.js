import { defineStore } from 'pinia'


export const useJobsStore = defineStore('jobs', {
    state: () => ({ jobs: [] }),
    getters: {
        getAll: (state) => state.jobs,
        getIds: (state) => {
            return state.jobs.map((agent) => agent.id)
        },
        getByAgent: (state) => {
            return (agentId) => state.jobs.filter((job) => job.agent_id == agentId)
        }
    },
    actions: {
        async fetchJobs() {
            const resp = await fetch("api/jobs")
            const data = await resp.json()
            this.jobs = data
        },

        async createJob(command, agentId) {
            const resp = await fetch("api/jobs", {
                method: "POST", headers: {
                    'Content-Type': 'application/json',
                }, body: JSON.stringify({ command: command, agent_id: agentId })
            })
        }
    }
})