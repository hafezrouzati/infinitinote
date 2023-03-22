<script setup>

import { infinitinote_backend, createActor } from '../../declarations/infinitinote_backend';
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

import TopBar from './components/TopBar.vue';
import { FlowerSpinner } from 'epic-spinners';

import { ref, onMounted, inject, provide } from 'vue';
import { useRouter, useRoute } from 'vue-router';

var userPrincipal = inject('userPrincipal');
var userAuthenticated = inject('userAuthenticated');
var isLoading = inject('isLoading');
const router = useRouter();
const route = useRoute();

var authClient;

</script>

<template>

    <div>
        <TopBar />
    </div>
    <div>
        <div id="overlay" v-if="isLoading">
                <div class="centered vertical-center">
                    <flower-spinner :animation-duration="1000" :size="60" color="#FFCB2C" class="centered" />
                </div>
        </div> 
        <router-view></router-view>
    </div>    
</template>

<style>

.centered
{
    margin-left: auto;
    margin-right: auto;
    margin-top: auto;
    margin-bottom: auto;
}

.vertical-center
{
    left: 50%;
    top: 50%;
    position: absolute;
    margin: 0;
    -ms-transform: translateX(50%) translateY(-50%);
    transform: translateX(50%) translateY(-50%);
}

</style>