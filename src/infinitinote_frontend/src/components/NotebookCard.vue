<template>
    <div class="notebook-card">
        <div class="notebook-note-count">
            <p class="notebook-note-count-text">{{ notebook.notes.length }} {{ notes_text }}</p>
        </div>
        <div class="notebook-title">
            {{ notebook.title }}
        </div>   
        <div class="notebook-open-button">
        </div>
        <div class="notebook-share-button">
        </div>
    </div>
</template>

<script setup>
import {computed } from 'vue';

const props = defineProps({
  notebook: Object,
  title: String,
  cover: String
});

var notes_text = computed(() => { if(props.notebook.notes.length == 1){return 'note';} else return 'notes'; });

</script>

<style>

.notebook-card
{
    height: 361px;
    width: 276px;
    margin-right: 10px;
    background-image: url('../../assets/ui/notebook-cover-blue.png');
    cursor: pointer;
}

.notebook-title
{
    margin-left: 20px;
    margin-right: auto;
    position: relative;
    top: 221px;
    font-family: montserrat;
    font-size: 24px;
    color: #FFFFFF;
}

.notebook-note-count 
{
    font-family: montserrat;
    font-size: 12px;
    color: #FFFFFF;
    background-color: rgba(255, 255, 255, .25);
    position: relative;
    left: 21px;
    top: 24px;
    min-width: 80px;
    width: 80px;
    height: 25px;
    border-radius: 7px;
}

.notebook-note-count-text
{
    position: relative;
    left: 21%;
    top: 25%;
}

.notebook-open-button
{
    background: url('../../assets/ui/notebook-open-button.png');
    position: relative;
    width: 101px;
    height: 42px;
    top: 228px;
    left: 25px;
}

.notebook-share-button
{
    background: url('../../assets/ui/notebook-share-button.png');
    position: relative;
    width: 42px;
    height: 42px;
    top: 186px;
    left: 214px;
}

</style>