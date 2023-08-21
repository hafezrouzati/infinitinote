<script setup>
import { readUIntLE } from '@dfinity/candid';
import { ref, onMounted, computed, onBeforeUnmount, inject } from 'vue';
var userAuthenticated = inject('userAuthenticated');
var backend = inject('backend');
import { useRouter } from 'vue-router';
const router = useRouter();
const searchText = ref('');
const loader = ref(false);
const results = ref({
    files: [
        {
            name: 'Book_t.pdf',
        }, {
            name: 'Book_2.pdf',
        },
        {
            name: 'Book_3.pdf',

        }
    ], notebooks: [
        {}
    ],
    notes: [
        {
            name: "note1"
        },
        {
            name: "note2"
        }
    ]
});
const showSuggestions = ref(false);
async function onInput() {
    loader.value = true;
    //fetch data and set results here
    var notes_search_results = await backend.value.search_notes_by_tag(searchText.value);
    var notebooks_search_results = await backend.value.search_notebooks_by_tag(searchText.value);
    results.value.files = [];
    results.value.notes = [];
    results.value.notebooks = [];
    results.value.notes = notes_search_results;
    results.value.notebooks = notebooks_search_results;
    loader.value = false;
    showSuggestions.value = true;
}
function onBlur() {
    setTimeout(() => {
        showSuggestions.value = false;
    }, 200);
}


const handleClickOutside = (event) => {
    if (!document.querySelector('.search-bar').contains(event.target)) {
        showSuggestions.value = false;
    }
};
function navigate(data, type) {
    console.log(data)
    switch (type) {
        case 'notes':
            router.push(`/note/${data.notebook_id}/${data.id}/`);
            break;
        case 'notebooks':
            router.push(`/notebook/${data.notebook_id}`);
            break;

        default:
            break;
    }
}
onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
    document.removeEventListener('click', handleClickOutside);
});
function navigateToNotebook() {
    router.push('/home');
}
</script>

<template>
    <div class="navbar">
        <div class="logo">
            <img src="/ui/inf-logo.png">
        </div>
        <div class="left-content">
            <div class="search-bar" ref="searchBar" v-if="userAuthenticated">
                <div class="search-container">
                    <img src="/ui/manage_search.svg" alt="">
                    <input type="search" v-model="searchText" @input="onInput" @blur="onBlur">
                    <img src="/ui/loader.gif" alt="" srcset="" v-if="loader" class="h-loader-29px">
                </div>
                <div class="suggestions" v-if="showSuggestions">
                    <div class="suggestion">
                        <p class="header-title">files</p>
                        <div class="content" v-for="file of results.files" @click="navigate(file, 'file')">
                            <div class="d-flex">
                                <img src="/ui/search-pdf.svg" alt="" class="search-result">
                                <p class="name"> {{ material?.name }}</p>
                            </div>
                            <p class="file"></p>
                        </div>
                        <p class="header-title">notebooks</p>
                        <div class="content" v-for="notebook of results.notebooks" @click="navigate(notebook, 'notebooks')">
                            <div class="d-flex">
                                <img src="/ui/background-notebook-1.svg" alt="" class="search-result">
                                <p class="name search-result">{{ notebook.title }}</p>
                            </div>
                        </div>
                        <p class="header-title">notes</p>
                        <div class="content" v-for="note of results.notes" @click="navigate(note, 'notes')">
                            <div class="d-flex">
                                <img src="/ui/background-notebook-1.svg" alt="" class="search-result">
                                <p class="name search-result">{{ note.title }}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="container-not-logged-in left-content" v-if="!userAuthenticated">
                <p class="signin">Sign In</p>
                <p class="left-content signup">
                    <img src="/ui/login.svg" alt="" srcset="">
                    Sign Up
                </p>
            </div>
            <div class="container-logged-in left-content" v-if="userAuthenticated">
                <p @click="navigateToNotebook()">
                    <img src="/ui/document.svg" alt="">
                    My Notebooks
                </p>
                <p class="setting">
                    <img src="/ui/setting.svg" alt="">
                </p>
                <span>
                    <img src="/ui/login.svg" alt="">
                    Logout
                </span>
            </div>
        </div>
    </div>
</template>


<style>
.navbar {
    margin: 10px 20px 50px 0px;
    height: 84px;
    top: 16px;
    display: flex;
    justify-content: space-between;
}

.logo {
    position: relative;
    margin-left: 20px;
    width: 169px;
    top: 9px;
}

.search-container {
    display: flex;
    border-radius: 13px;
    background: #FFF;
    padding: 13px 10px;
}

.search-container input {
    border: 0px !important;
    outline: none;
    width: 500px;
}

.search-bar {
    position: relative;
}

.search-bar .suggestions {
    z-index: 1000;
    margin-top: 10px;
    border-radius: 25px;
    background: #FFF;
    height: 385px;
    padding: 24px;
    position: absolute;
    width: 100%;
    border-radius: 10px;
    box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
}

.search-result
{
    cursor: pointer;
}

.suggestion .header-title {
    opacity: 0.5;
    color: #2B3249;
    font-size: 14px;
}


.suggestion .content {
    display: flex;
    margin-top: 10px;
    justify-content: space-between;
}

.suggestion .content .name {
    color: #2B3249;
    font-family: Montserrat;
    font-size: 16px;
}

.left-content {
    display: flex;
    align-items: center;
}

.d-flex {
    display: flex;
}

.d-flex p {
    margin-left: 10px;
}

.suggestion .content .file {
    color: #2B3249;
    font-size: 16px;
    opacity: 0.4000000059604645;
}

.signin {
    color: #2B3249;
    font-size: 16px;
    margin-left: 49px;
}

.signup {
    color: #2261D3;
    font-size: 16px;
    display: flex;
    align-items: center;
    border-radius: 13px;
    background: rgba(34, 97, 211, 0.10);
    padding: 15px 27px;
    margin-left: 20px;
}

.signup img {
    margin-right: 13px;
}

.container-logged-in p {
    color: #2B3249;
    font-size: 16px;
    margin-left: 10px;
    display: flex;
    align-items: center;
    height: 50px;
    padding: 1px 27px;
    border-radius: 13px;
    border: 1.5px solid #D3D9E3;
}

.container-logged-in p img {
    margin-right: 13px;
}

.container-logged-in span img {
    margin-right: 13px;
}

.container-logged-in span {
    margin-left: 25px;
    display: flex;
    align-items: center;
    color: #2B3249;
    font-size: 16px;
}

.container-logged-in .setting {
    padding: 1px 11px !important;
}

.container-logged-in .setting img {
    margin-right: 0px !important
}

.h-loader-29px {
    height: 29px;
}
</style>