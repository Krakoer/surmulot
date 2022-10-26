import { defineStore } from 'pinia'


export const useAgentsStore = defineStore('agents', {
  state: () => ({agents: []}),
  getters: {
    getAll: (state) => state.agents,
    getIds: (state) => {
        return state.agents.map((agent) => agent.id)
    }
  },
  actions: {
    async fetchAgents() {
        const resp = await fetch("api/agents")
        const data = await resp.json()
        this.agents = data
    }
  }
})