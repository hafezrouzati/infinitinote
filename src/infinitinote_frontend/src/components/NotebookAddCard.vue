<template>
    <div class="notebook-add-card" @click="$event => navigate_add_notebook()">
    </div>
</template>

<script setup>

import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

function navigate_add_notebook()
{
    router.push('/add_notebook');
}

</script>

<style>

.notebook-add-card
{
    background-image: url('../../assets/ui/notebook-create-new.png');
    height: 361px;
    width: 276px;
    margin-right: 10px;
    cursor: pointer;
}

</style>