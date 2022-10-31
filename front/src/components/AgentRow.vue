<template>
    <tr>
        <td>{{agent["id"]}}</td>
        <td>{{agent["created_at"]}}</td>
        <td>{{agent["last_seen_at"]}}</td>
        <td>{{agent["username"]}}</td>
        <td>{{agent["hostname"]}}</td>
    </tr>
</template>

<script>
export default{
    props:{
        agent: Object
    }
}
</script>

<style>
td{
cursor: pointer;
}
</style>