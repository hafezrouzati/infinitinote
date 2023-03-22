<script setup>

import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import NotebookCard from './NotebookCard.vue';
import NotebookAddCard from './NotebookAddCard.vue';

const router = useRouter();
const route = useRoute();

var the_notebooks = [];

var userAuthenticated = inject('userAuthenticated');
var notebooks = inject('notebooks');
var backend = inject('backend');

var isLoading = inject('isLoading');

function navigate_add_notebook()
{
    router.push('/add_notebook');
}

async function load_notebooks()
{
    isLoading.value = true;
    
    let result = await backend.value.get_notebooks_for_principal();
    notebooks.value = result;
    the_notebooks = notebooks.value;
    console.log(the_notebooks);
    setContentWidth();
    
    isLoading.value = false;
}

function setContentWidth()
{
    var contentContainer = document.getElementById('notebooks-scrolling-content');
    var computedWidth = ((notebooks.value.length + 1) * 276) + 100 + 'px';
    contentContainer.style.width = computedWidth;
}

onMounted( async () => {
        if (userAuthenticated.value == false)
        {
            router.push('/');
        }

        if (backend.value != null)
        {
            if (notebooks.value)
            {
                the_notebooks = notebooks.value;
            }
            load_notebooks();
        }

});

</script>

<template>
    <div class="notebooks-container">
        
        <div class="notebooks-header">Notebooks Home</div>
        
        <div class="notebooks-scrolling-container">
            <div id='notebooks-scrolling-content' class="notebooks-scrolling-content">
                <NotebookAddCard />
                <NotebookCard v-for="notebook in the_notebooks" :notebook="notebook" v-if="notebooks" @click="$event => router.push({ name: 'notebook', params: { notebookID: notebook.id }})" />
            </div>
        </div>
    </div>

</template>

<style>

.notebooks-container
{
    margin-left: 27px;
}

.notebooks-scrolling-container
{
    width: 100%;
    overflow-x: auto;
}

.notebooks-scrolling-content
{
    display: flex;
    width: 200%;
}

</style>