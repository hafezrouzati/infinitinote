<script setup>
import { inject, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import NoteCard from './NoteCard.vue';

const props = defineProps({
  notebook: Object
});

const router = useRouter();
const route = useRoute();

var backend = inject('backend');
var userAuthenticated = inject('userAuthenticated');

const notebookID = ref(route.params.notebookID);
var notebooks = inject('notebooks');

var the_notebook = ref(null);

var isLoading = inject('isLoading');

async function add_note()
{
    isLoading.value = true;

    let result = await backend.value.add_note_to_notebook(the_notebook.value.id, "","", "", []);

    isLoading.value = false;

    if (result == 'Error')
    {
        //todo: handle error case
    }
    else {
        let note_id = result;
        console.log(note_id);
        router.push({name: 'note', params: { notebookID: the_notebook.value.id, noteID: note_id }});
    }
}

async function open_note(noteID)
{
    router.push({ name: 'note', params: { notebookID: the_notebook.value.id, noteID: noteID}});
}

onMounted( async () => {
    console.log(notebooks.value);
    console.log(userAuthenticated.value);
    if (userAuthenticated.value == false)
    {
            router.push('/');
    }
    else 
    {
        the_notebook.value = notebooks.value.find((n) => n.id == notebookID.value);
        console.log(the_notebook);
    }
});

</script>

<template>
<div class="notebook-container">
    <div class="notebook-header-flex-container">
        <div v-if="the_notebook">
            <div class="notebook-header">{{ the_notebook.title || 'Loading...' }}</div>
        </div>    

        <div class="notebook-cover-button">
            <img src='/ui/add-photo-alternate.png' width="22px" height="22px" class="add-image-button" @click="add_cover()"><p>Notebook Cover</p>
        </div>
        <div class="note-add-button" @click="add_note()"><img src='/ui/add-circle.png' width="22px" height="22px" class="add-image-button" @click="add_note()"><p>Add</p></div>
        <div class="notebook-tags-button"><img src='/ui/add-tag-icon.png' width="22px" height="22px" class="add-image-button" @click="add_tags()"><p>Tags</p></div>

    </div>
    <div class="notebook-note-container">
        <div class="notebook-sidebar-left">

        </div>
        <div class="notes-container" v-if="the_notebook">
            <NoteCard  v-for="note in the_notebook.notes" :note="note" v-if="the_notebook.notes" @click="open_note(note.id)"/>
        </div>
    </div>

</div>

</template>

<style>

.notebook-container
{
    margin-left: 27px;
    top: 54px;
    min-width: 1457px;
}

.notebook-header-flex-container
{
    display: flex;
}

.notebook-header {
    font-family: montserrat;
    font-size: 40px;
    width: 900px;
}

.notebook-note-container 
{
    position: relative;
    display: flex;
    top: 60px;
}

.notebook-sidebar-left
{
    position: relative;
    width: 75px;
    min-width: 75px;
    height: 694px;
    border-radius: 25px;
    background-color: #FFFFFF;

}

.notes-container
{
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-auto-rows: 338px;
  grid-gap: 12px;
  margin-left: 33px;
  position: relative;
}

.notebook-cover-button
{
    display: flex;
    align-items: center;
    text-align: center;
    justify-content: center;
    width: 216px;
    min-width: 216px;
    height: 50px;
    background-color: #FFFFFF;
    border-radius: 13px;
    margin-left: 80px;
    cursor: pointer;
    font-family: montserrat;
    font-size: 16px;
}

.note-add-button
{
    background-color: #FFFFFF;
    display: flex;
    align-items: center;
    text-align: center;
    justify-content: center;
    width: 122px;
    min-width: 122px;
    height: 50px;
    border-radius: 13px;
    margin-left: 6px;
    cursor: pointer;
    font-family: montserrat;
    font-size: 16px;
}

.notebook-tags-button
{
    background-color: #FFFFFF;
    display: flex;
    align-items: center;
    text-align: center;
    justify-content: center;
    width: 126px;
    min-width: 126px;
    height: 50px;
    border-radius: 13px;
    margin-left: 6px;
    cursor: pointer;
    font-family: montserrat;
    font-size: 16px;
}

.add-image-button
{
    margin:0;
    width: 22px;
    height: 22px;
}

</style>