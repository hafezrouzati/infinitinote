<template>
    <div class="notebook-card">
        <div class="notebook-note-count">
            {{ notebook.notes.length }} notes
        </div>
        <div class="notebook-title">
            {{ notebook.title }}
        </div>   
        <div class="notebook-open-button">
            Open
        </div>
    </div>
</template>

<script setup>

const props = defineProps({
  notebook: Object,
  title: String,
  cover: String
});

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
}

.notebook-open-button
{
    width: 101px;
    height: 42px;
}

</style>