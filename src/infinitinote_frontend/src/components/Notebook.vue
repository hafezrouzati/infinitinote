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
const openSideBar = ref(false);
const notes = ref([
    {
        title: 'Notebook 1',
        notes: [
            {
                name: 'Note 1',
            },
            {
                name: 'Note 2',
            }
        ]
    },
    {
        title: 'Notebook 2',
        notes: [
            {
                name: 'Note 3',
            },
            {
                name: 'Note 4',
            }
        ]
    },
    {
        title: 'Notebook 3',
        notes: [
            {
                name: 'Note 6',
            },
            {
                name: 'Note 7',
            }
        ]
    }
]);
var notebooks = inject('notebooks');

var the_notebook = ref(null);

var isLoading = inject('isLoading');

async function add_note() {
    isLoading.value = true;

    let result = await backend.value.add_note_to_notebook(the_notebook.value.id, "", "", "", []);

    isLoading.value = false;

    if (result == 'Error') {
        //todo: handle error case
    }
    else {
        let note_id = result;
        console.log(note_id);
        router.push({ name: 'note', params: { notebookID: the_notebook.value.id, noteID: note_id } });
    }
}

async function open_note(noteID) {
    router.push({ name: 'note', params: { notebookID: the_notebook.value.id, noteID: noteID } });
}
function toggleSideBar() {
    openSideBar.value = !openSideBar.value;
}
onMounted(async () => {
    console.log(notebooks.value);
    console.log(userAuthenticated.value);
    if (userAuthenticated.value == false) {
        // router.push('/');
    }
    else {
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
                <img src='/ui/add-photo-alternate.png' width="22px" height="22px" class="add-image-button"
                    @click="add_cover()">
                <p>Notebook Cover</p>
            </div>
            <div class="note-add-button" @click="add_note()"><img src='/ui/add-circle.png' width="22px" height="22px"
                    class="add-image-button" @click="add_note()">
                <p>Add</p>
            </div>
            <div class="notebook-tags-button"><img src='/ui/add-tag-icon.png' width="22px" height="22px"
                    class="add-image-button" @click="add_tags()">
                <p>Tags</p>
            </div>

        </div>
        <div class="notebook-note-container">
            <div class="notebook-sidebar-left">
                <div class="arrow-section">
                    <div class="header-container">

                        <span class="arrow" @click="toggleSideBar()">
                            <img src="/ui/arrow_right.svg" alt="" srcset="" v-if="!openSideBar">
                            <img src="/ui/close-chapter.svg" alt="" srcset="" v-if="openSideBar">
                        </span>
                        <span v-if="openSideBar" class="close">Close</span>

                    </div>
                    <div class="card-title" v-for="note in notes">
                        <div class="card-header" @click="note.expand = !note.expand">
                            <img src="/ui/background-notebook-1.svg" alt="" srcset="">
                            <p class="name" v-if="openSideBar">{{ note.title }}</p>
                            <img src="/ui/arrow_circle_down.svg" alt="" v-if="openSideBar && note.expand"
                                class="arrow-side arrow-side-down">
                            <img src="/ui/arrow_circle_right-chapter.svg" alt="" v-if="openSideBar && !note.expand"
                                class="arrow-side arrow-side-down">
                        </div>
                        <div class="card-body" v-if="openSideBar && note.expand">
                            <div class="chapter-card" v-for="chapter in note.notes">
                                <img src="/ui/arrow_circle_right-chapter.svg" alt="" class="arrow-side arrow-side-right">
                                <span class="title">{{ chapter.name }}</span>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
            <div class="notes-container" v-if="the_notebook">
                <NoteCard v-for="note in the_notebook.notes" :note="note" v-if="the_notebook.notes"
                    @click="open_note(note.id)" />
            </div>
        </div>

    </div>
</template>

<style>
.notebook-container {
    margin-left: 27px;
    top: 54px;
    min-width: 1457px;
}

.notebook-header-flex-container {
    display: flex;
}

.notebook-header {
    font-family: montserrat;
    font-size: 40px;
    width: 900px;
}

.notebook-note-container {
    position: relative;
    display: flex;
    top: 60px;
}

.notebook-sidebar-left {
    position: relative;
    min-width: 40px;
    min-height: 694px;
    border-radius: 25px;
    background-color: #FFFFFF;
    padding: 25px;

}

.notes-container {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-auto-rows: 338px;
    grid-gap: 12px;
    margin-left: 33px;
    position: relative;
}

.notebook-cover-button {
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

.note-add-button {
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

.notebook-tags-button {
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

.add-image-button {
    margin: 0;
    width: 22px;
    height: 22px;
}

.arrow-section {
    display: flex;
    flex-direction: column;
    align-items: start;
}

.header-container {
    display: flex;
    justify-content: center;
    align-items: center;
}

.arrow-section .arrow {
    margin-top: 25px;
    border-radius: 50%;
    background: rgba(34, 97, 211, 0.10);
    display: flex;
    width: 20px;
    height: 20px;
    padding: 15px;
    justify-content: center;
    align-items: center;
}

.arrow-section .arrow img {
    height: 21px;
}

.arrow-section .card-title {
    display: flex;
    flex-direction: column;
    align-items: start;
    width: 100%;
}

.arrow-section .card-title img {
    margin-top: 20px;
    width: 100%;
    height: 46px;
}

.arrow-section .card-title .arrow-side {
    height: 30px;
    width: 30px;
    margin-left: 0px;
}

.arrow-section .close {
    color: #205EE7;
    font-size: 16px;
    font-weight: 600;
    margin-left: 15px;
    margin-top: 23px;
}

.arrow-section .card-header {
    display: flex;
}

.arrow-section .card-header .name {
    color: #2B3249;
    font-size: 14px;
}

.header-container {
    margin-bottom: 20px;
}

.arrow-section .card-body {
    margin-top: 10px;
    width: 229px;
    border-radius: 15px;
    background: #EDEFF3;
    display: flex;
    margin-left: 15px;
    flex-direction: column;
    margin-right: 52px;
}

.chapter-card img {
    height: 30px;
}

.chapter-card {
    display: flex;
    padding: 10px;
    align-items: center;
}

.chapter-card .title {
    color: #2B3249;
    font-size: 14px;

}

.arrow-side-right {
    margin-top: 0px !important;
    margin-right: 10px;
}

.arrow-side-down {
    margin-left: 20px !important;
}
</style>