<template>
    <div class="notebook-add-page-container">
        <div class="notebook-add-header">Add Notebook</div>
        <div class="notebook-add-container">
            <div class="notebook-add-title-label">Notebook Title</div>
            <input type="text" class="notebook-add-title-input" v-model="notebook_title">
            <div class="notebook-add-cover-label">Notebook Cover</div>
            <div class="notebook-add-cover-container">
                <div v-for="cover in covers" :key="cover.color" class="notebook-add-cover" :class="`notebook-cover-${cover.color}`" @click="selectedCover=cover;">
                <img :src="`/ui/notebook-cover-${cover.color}.png`" class="notebook-cover-image image_button">
                <label :for="`cover-radio-${cover.color}`" class="cover-radio-label" v-if="cover.color!='custom'">
                    <input
                    :id="`cover-radio-${cover.color}`"
                    :name="`notebook-cover`"
                    type="radio"
                    class="cover-radio-input"
                    :value="cover"
                    v-model="selectedCover"
                    >
                </label>
            </div>
            </div>
            <div class="notebook-add-notebook-button">
                <img src='/ui/btn_add_notebook.jpg' class="image_button add_notebook_btn" @click="$event => add_notebook()">
            </div>
        </div>
        <div class="notebook-add-tags-container">

        </div>
    </div>

    <!-- <div class="border-margin">
        <div class="large-header">Add Notebook</div>
        <div>
            <div class="small-header">Notebook Title</div>
            <input type="text" v-model="notebook_title">
        </div>
        <div>
            <div>cover purple</div><div>cover yellow</div>
        </div>

        <div>
            <img src="/ui/in_logo.png" class="image_button" @click="$event => add_notebook()">
        </div>
    </div> -->
</template>

<style>
.cover-radio-label{
    position: absolute;
    top: 5px;
    left: 8px;
}
.add_notebook_btn{
    margin: 0;
    margin-top: 15px;
}
.notebook-add-page-container
{
    margin-left: 26px;

}

.notebook-add-header
{
    font-family: montserrat;
    font-size: 40px;
}

.notebook-add-title-label
{
    font-family: montserrat;
    font-size: 16px;
    left: 51px;
    top: 10px;
    position: relative;
}

.notebook-add-title-input
{
    position: relative;
    appearance: none;
    width: 1049px;
    height: 54px;
    left: 49px;
    top: 15px;
    border: 1px solid #D9D9D9;
    border-radius: 13px;
    font-family: montserrat;
    font-size: 20px;
}

.notebook-add-container
{
    width: 1134px;
    height: 500px;
    border-radius: 25px;
    background-color: #FFFFFF;
}

.notebook-add-cover
{
    position: relative;
    width: 170px;
    height: 223px;
    margin-left: 5px;
    background-size: contain;
}

.notebook-add-cover-label
{
    font-family: montserrat;
    font-size: 16px;
    position: relative;
    top: 20px;
    left: 51px;
}

.notebook-add-cover-container
{
    display: flex;
    position: relative;
    left: 49px;
    top: 30px;
}

.notebook-cover-image
{
    width: 170px;
    height: 223px;
}

.notebook-cover-green
{
    
    background-repeat: no-repeat;
}

.notebook-cover-yellow
{
    
    background-repeat: no-repeat;
}

.notebook-cover-purple
{
    
    background-repeat: no-repeat;
}

.notebook-cover-red
{
    
    background-repeat: no-repeat;
}

.notebook-cover-blue
{
    background-repeat: no-repeat;
}

.notebook-add-notebook-button
{
    position: relative;
    left: 59px;
    top: 50px;
}

</style>

<script setup>

import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();
var covers=ref([
        { color: 'purple' },
        { color: 'green' },
        { color: 'red' },
        { color: 'blue' },
        { color: 'yellow' },
        { color: 'custom' }
      ]);
var selectedCover=ref(null)
var userPrincipal = inject('userPrincipal');
var userAuthenticated = inject('userAuthenticated');
var backend = inject('backend');
var isLoading = inject('isLoading');

var notebook_title = ref(null);

async function add_notebook()
{
    isLoading.value = true;

    let result = await backend.value.add_notebook_for_principal(userPrincipal.value, notebook_title.value);
    console.log(result);

    isLoading.value = false;

    // todo: router.push name notebook params notebook_id
    router.push('/home');
}

onMounted( async () => {
        if (userAuthenticated.value == false)
        {
            router.push('/');
        }
});

</script>