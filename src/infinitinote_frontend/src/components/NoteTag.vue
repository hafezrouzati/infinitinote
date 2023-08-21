<template>
    <div class="note-tag-container">
        <div class="note-tag-header">
            <p>Notebook Tags</p>
            <span @click="addChip">
                <img src="/ui/add.svg" alt="" srcset="">
            </span>
        </div>
        <div class="note-tag-chip-container">
            <div class="note-tag-chip" v-for="(chip, index) in chipModel" :key="index">
                <span>
                    {{ chip }}
                </span>
                <span @click="deleteChip(index)">
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
            <div class="note-tag-chip" v-if="addingChip">
                <input v-model="newChip" type="text" v-focus />
                <span @click="saveChip">
                    <img src="/ui/tick.svg" alt="" srcset="">
                </span>
                <span @click="cancelChip">
                    <img src="/ui/close.svg" alt="" srcset="">
                </span>
            </div>
        </div>
    </div>
</template>


<script setup>

import { inject, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();

var backend = inject('backend');

var noteID = ref(route.params.noteID);
var notebookID = ref(route.params.notebookID);
var the_note = inject('the_note');

var notebooks = inject('notebooks');
var the_notebook = ref(null);
var the_note = ref(null);

const chipModel = ref([]);
const addingChip = ref(false);
const newChip = ref('');

onMounted(async () => {

    the_notebook.value = notebooks.value.find((n) => n.id == notebookID.value);
    the_note.value = the_notebook.value.notes.find((n) => n.id == noteID.value);

    function loadTagsModel() {
        if (route.path.includes('notebook'))
        {
            for (let i = 0; i < the_notebook.value.tags.length; i++)
            {
                chipModel.value.push(the_notebook.value.tags[i]);
            }
        }
        else
        {
            for (let i = 0; i < the_note.value.tags.length; i++)
            {
                chipModel.value.push(the_note.value.tags[i]);
            }
        }
    };

    loadTagsModel();

});
        

        function addChip() {
            addingChip.value = true;
        };

        async function saveChip() {
            chipModel.value.push(newChip.value);
            if (route.path.includes('notebook'))
            {
                await backend.value.add_tag_to_notebook(notebookID.value, newChip.value);
            } 
            else 
            {
                await backend.value.add_tag_to_note(notebookID.value, noteID.value, newChip.value);
            }
            newChip.value = '';
            addingChip.value = false;
        };

        function cancelChip() {
            newChip.value = '';
            addingChip.value = false;
        };

        async function deleteChip(index){
            var removed_tag = chipModel.value.splice(index, 1);
            if (route.path.includes('notebook'))
            {
                await backend.value.remove_tag_from_notebook(notebookID.value, removed_tag[0]);
                var tag_index = the_notebook.value.tags.indexOf(removed_tag[0]);
                the_notebook.value.tags.splice(tag_index, 1);
            }
            else 
            {
                await backend.value.remove_tag_from_note(notebookID.value, noteID.value, removed_tag[0]);
                var tag_index = the_note.value.tags.indexOf(removed_tag[0]);
                the_note.value.tags.splice(tag_index, 1);
            }
        };
</script>


<style>
.note-tag-container {
    width: 276px;
    min-height: 254px;
    border-radius: 25px;
    background: #FFF;
    padding: 29px 32px;
}

.note-tag-header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.note-tag-header p {
    color: #2B3249;
    font-size: 16px;
    font-weight: 600;
}

.note-tag-header span {
    display: flex;
    width: 22px;
    height: 22px;
    padding: 14px;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    line-height: 0;
    background: rgba(34, 97, 211, 0.10);
}

.note-tag-chip-container {
    display: flex;
    flex-direction: row;
    /* or column, depending on your layout */
    flex-wrap: wrap;
    margin-top: 15px;
    align-items: flex-start;
    /* aligns items to the start of the container */

}

.note-tag-chip {
    display: inline-flex;
    padding: 9px 8px 9px 12px;
    justify-content: center;
    align-items: center;
    gap: 7px;
    margin: 10px 5px;
    border-radius: 10px;
    background: rgba(34, 97, 211, 0.10);
}

.note-tag-chip span:nth-child(1) {
    font-size: 16px;
    font-weight: 600;
}
</style>