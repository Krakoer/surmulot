<template>
    <tr>
        <td>{{agent["id"]}}</td>
        <td>{{formatDate(agent["created_at"])}}</td>
        <td>{{formatDate(agent["last_seen_at"])}}</td>
        <td>{{agent["username"]}}</td>
        <td>{{agent["hostname"]}}</td>
    </tr>
</template>

<script>
import formatDateMixin from '@/mixins/formatDateMixin';

export default{
    props:{
        agent: Object
    },
    mixins: [formatDateMixin],
}
</script>

<style scoped>
td{
cursor: pointer;
}
</style>