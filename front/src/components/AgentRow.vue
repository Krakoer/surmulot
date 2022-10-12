<template>
    <tr>
        <td>{{agent["id"]}}</td>
        <td>{{agent["created_at"]}}</td>
        <td>{{agent["last_seen_at"]}}</td>
    </tr>
</template>

<script>
export default{
    props:{
        agent: Object
    }
}
</script>