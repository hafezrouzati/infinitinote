<script setup>
import { ref, onMounted, computed, onBeforeUnmount, inject } from 'vue';
var userAuthenticated = inject('userAuthenticated');
const searchText = ref('');
const showSuggestions = ref(false);
const suggestions = computed(() => {
    return 'sasa'
});
function onInput() {
    showSuggestions.value = true;
}
function onBlur() {
    setTimeout(() => {
        showSuggestions.value = false;
    }, 200);
}
function selectSuggestion(suggestion) {
    searchText.value = suggestion;
    showSuggestions.value = false;
}

const handleClickOutside = (event) => {
    if (!document.querySelector('.search-bar').contains(event.target)) {
        showSuggestions.value = false;
    }
};

onMounted(() => {
    document.addEventListener('click', handleClickOutside);
});

onBeforeUnmount(() => {
    document.removeEventListener('click', handleClickOutside);
});

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
                </div>
                <div class="suggestions" v-if="showSuggestions">
                    <div class="suggestion">
                        <p class="header-title">material inside</p>
                        <div class="content">
                            <div class="d-flex">
                                <img src="/ui/search-pdf.svg" alt="">
                                <p class="name"> Book_t.pdf</p>
                            </div>
                            <p class="file">function declarations </p>
                        </div>
                        <div class="content">
                            <div class="d-flex">
                                <img src="/ui/search-pdf.svg" alt="">
                                <p class="name"> Book_t.pdf</p>
                            </div>
                            <p class="file">function declarations </p>
                        </div>
                        <div class="content">
                            <div class="d-flex">
                                <img src="/ui/search-pdf.svg" alt="">
                                <p class="name"> Book_t.pdf</p>
                            </div>
                            <p class="file">function declarations </p>
                        </div>
                        <p class="header-title">notes</p>
                        <div class="content">
                            <div class="d-flex">
                                <img src="/ui/background-notebook-1.svg" alt="">
                                <p class="name">function declarations in swift</p>
                            </div>
                        </div>
                        <div class="content">
                            <div class="d-flex">
                                <img src="/ui/background-notebook-1.svg" alt="">
                                <p class="name">function declarations in swift</p>
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
                <p>
                    <img src="/ui/document.svg" alt="">
                    My notes
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

.search-bar .suggestions {
    margin-top: 10px;
    border-radius: 25px;
    background: #FFF;
    height: 385px;
    box-shadow: 0px 4px 74px 0px rgba(0, 0, 0, 0.08);
    padding: 24px;
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
</style>