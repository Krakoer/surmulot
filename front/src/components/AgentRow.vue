<template>
    <tr :class="{selected: selected}">
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
        agent: Object,
        selected: Boolean
    },
    mixins: [formatDateMixin],
}
</script>

<style scoped>
td{
    cursor: pointer;
}
tr:hover
{
    background-color: #e9e9e9;
}
tr.selected
{
    background-color: #e9e9e9;
}
</style>