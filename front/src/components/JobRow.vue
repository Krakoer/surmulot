<template>
    <tr>
        <td>{{job["id"]}}</td>
        <td>{{job["created_at"]}}</td>
        <td>{{job["executed_at"]}}</td>
        <td>{{job["command"]}}</td>
        <td>{{job["args"]}}</td>
        <td>{{job["output"]}}</td>
        <td>{{job["agent_id"]}}</td>
    </tr>
</template>

<script>
export default{
    props:{
        job: Object
    }
}
</script>