<template>
    <tr>
        <td>{{job["id"]}}</td>
        <td>{{formatDate(job["created_at"])}}</td>
        <td>{{formatDate(job["executed_at"])}}</td>
        <td>{{job["command"]}}</td>
        <td>{{job["args"]}}</td>
        <td>{{job["output"]}}</td>
        <td>{{job["agent_id"]}}</td>
    </tr>
</template>

<script setup>
import {formatDate} from '@/modules/Format'

const props = defineProps({
    job: Object
})

</script>