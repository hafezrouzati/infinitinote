<template>
    <div class="border-margin">
        <div class="large-header">Add Notebook</div>
        <div>
            <div class="small-header">Notebook Title</div>
            <input type="text" v-model="notebook_title">
        </div>
        <div>
            <div>cover purple</div><div>cover yellow</div>
        </div>

        <div>
            <img src="/ui/in_logo.png" class="image_button" @click="$event => add_notebook()">
        </div>
    </div>
</template>

<script setup>

import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

var userPrincipal = inject('userPrincipal');
var backend = inject('backend');
var isLoading = inject('isLoading');

var notebook_title = ref(null);

async function add_notebook()
{
    isLoading.value = true;

    let result = await backend.value.add_notebook_for_principal(userPrincipal.value, notebook_title.value);
    console.log(result);

    isLoading.value = false;

    // todo: router.push name notebook params notebook_id
    router.push('/home');
}

</script>

<style>

</style>