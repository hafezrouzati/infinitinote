<script setup>
import { inject, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { onBeforeRouteLeave } from 'vue-router';
import { QuillEditor } from '@vueup/vue-quill'

const props = defineProps({
  notebook: Object,
  note: Object
});

const router = useRouter();
const route = useRoute();

var userAuthenticated = inject('userAuthenticated');
var backend = inject('backend');
var notebooks = inject('notebooks');
var isLoading = inject('isLoading');

var noteID = ref(route.params.noteID);
var notebookID = ref(route.params.notebookID);

var the_note = ref(null)
var the_notebook = ref(null);
var the_note_content = ref(null);
var myQuillEditor = ref(null);

async function load_notebooks() 
{
    isLoading.value = true;
    let result = await backend.value.get_notebooks_for_principal();
    notebooks.value = result;
    isLoading.value = false;
}

async function background_update_note()
{
    //todo: background update the note
}

async function update_note()
{
    let note_content = myQuillEditor.value.getQuill().getText();
    let note_content_delta_string = JSON.stringify(myQuillEditor.value.getQuill().getContents());

    let result = await backend.value.update_note(
        the_notebook.value.id,
        the_note.value.id, 
        the_note.value.title, 
        note_content, 
        note_content_delta_string,
        the_note.value.tags
    );
    console.log(result);
}

async function quillReady()
{
    console.log('Quill editor is ready!');

}

onMounted( async () => {

    if (userAuthenticated.value == false)
    {
            router.push('/');
    }
    else 
    {
        console.log('load notebooks');
        isLoading.value = true;
        await load_notebooks();
        the_notebook.value = notebooks.value.find((n) => n.id == notebookID.value);
        console.log(the_notebook.value);
        the_note.value = the_notebook.value.notes.find((n) => n.id == noteID.value);

        if (the_note.value.content_delta != "")
        {
            try {
                var note_content_delta = JSON.parse(the_note.value.content_delta);
                myQuillEditor.value.getQuill().setContents(note_content_delta);
            } catch (err) {
                console.log(err);
                the_note_content.value = "";
            }
        } else {
            the_note_content.value = "";
        }

        isLoading.value = false;
    }
});

onBeforeRouteLeave(async (to, from, next) => {
    isLoading.value = true;
    if(userAuthenticated.value != false)
    {  
        if (the_note.value != null)
        {
            await update_note();
            await load_notebooks();
        }
   }

   isLoading.value = false;

  next()
})

</script>

<template>
    <div class='note-container'>
        <div class='note-header' v-if="the_notebook">
            <div class='note-notebook-title-container' v-if="the_notebook">
                <p class='note-notebook-title'>{{ the_notebook.title }}</p>
            </div>
            <div class='separator'>/</div>
            <div class='note-note-title-container'>
                <input type='text' class='note-title-input' v-model='the_note.title' v-if="the_note">
            </div>
        </div>
        <div class='note-content-container'>
            <div class="note-editor-container">
                <div class='note-editor'>
                    <QuillEditor theme='snow' ref="myQuillEditor" v-model:content="the_note_content" @ready="$event => quillReady()"/>
                </div>
            </div>
        </div>
    </div>

</template>

<style>

.note-header
{
    display: flex;
    flex-direction: row;
}

.separator
{
    font-family: montserrat;
    font-size: 26px;
    position: relative;
    margin-left: 22px;
    top: 11px;
}

.note-notebook-title-container
{
    display: inline-block;
    width: auto;
    height: 50px;
    max-width: 500px;
    margin-left: 22px;
    border-radius: 11px;
    background-color: #FFFFFF;
    font-family: montserrat;
    font-size: 26px;
    color: #2B3249;
}

.note-notebook-title
{
    position: relative;
    top: -33%;
    left: 0%;
}

.note-note-title-container
{
    width: auto;
    max-width: 500px;
    height: 50px;
    margin-left: 22px;
    border-radius: 11px;
    background-color: #FFFFFF;
    font-family: montserrat;
    font-size: 26px;
    color: #2B3249;
    padding-left: 1%;
    padding-right: 1%;

}

.note-note-title
{

}

.note-title-input
{
    appearance: none;
    border: none;
    outline: none;
    background-color: transparent;
    color: inherit;
    font-size: inherit;
    font-family: inherit;
    padding: 0;
    margin: 0;
    position: relative;
    top: 20%;
}

.note-content-container
{
    position: relative;
    top: 26px;
}

.note-editor-container
{
    margin-left: 26px;
    margin-right: 26px;
    background-color: #FFFFFF;
    border-radius: 25px;
}

.note-editor
{
    height: 500px;
}

</style>