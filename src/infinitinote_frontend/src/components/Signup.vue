<template>
    SIGNUP
    <img src="/ui/icp_login.png" class="image_button" @click="authenticate_icp()" >
</template>

<script setup>
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { infinitinote_backend, canisterId, createActor } from '../../../declarations/infinitinote_backend';
import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';

var userAuthenticated = inject('userAuthenticated');
var userPrincipal = inject('userPrincipal');
var isLoading = inject('isLoading');
var backend = inject('backend');

const router = useRouter()
const route = useRoute();

var authClient = null;

async function onAuthenticated_icp(authClient) 
{
    userAuthenticated.value = true;

    let userIdentity = await authClient.getIdentity();
    userPrincipal.value = userIdentity.getPrincipal().toString();

    var actorOptions = {
        agentOptions: {
            identity: userIdentity,
        }
    };

    var authenticated_backend = createActor(canisterId, actorOptions);

    backend.value = authenticated_backend;

    router.push('/home');
}

async function authenticate_icp() 
{
    authClient = await AuthClient.create();

    await authClient.login({
            identityProvider: process.env.II_URL,
            onSuccess: async () => {
              onAuthenticated_icp(authClient);
            },
        });
}

onMounted( async () => {
        isLoading.value = false;
        if (userAuthenticated.value == true)
        {
            router.push('/home');
        }
});

</script>

<style>

</style>