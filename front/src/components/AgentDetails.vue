<template>
    <h1>About {{agent.id}}</h1>
    <h2>Username: {{agent.username}}</h2>
</template>

<script setup>

const props = defineProps({
    agent: Object
})
</script>
