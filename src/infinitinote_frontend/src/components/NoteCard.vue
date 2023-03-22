<template>
    <div class="note-card-container">
        <div class="note-card-inner-container">
            <div class="note-title">
                {{ note.title }}
            </div>
            <div class="note-content" v-if="note.content">
                {{  note.content }}
            </div>
        </div>

    </div>

</template>

<script setup>
import { inject, onMounted, ref } from 'vue';
import { QuillEditor, Quill } from '@vueup/vue-quill'

const props = defineProps({
  note: Object,
});

onMounted( () => {
    console.log(props.note.content);
});

</script>

<style>

.note-card-container
{
    background-color: #FFFFFF;
    border-radius: 25px;
    width: 441px;
    height: 338px;
    cursor: pointer;
}

.note-title
{
    font-family: montserrat;
    font-size: 24px;
    position: relative;
    top: 30px;
    margin-left: 29px;
    width: 358px;
    overflow: hidden;
}

.note-content
{
    position: relative;
    margin-left: 29px;
    font-family: montserrat-medium;
    font-size: 14px;
    top: 110px;
    color: #8C909E;
    width: 358px;
    height: 120px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

</style>