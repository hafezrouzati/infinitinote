<template>
  <editor-content :editor="editor" />
    <img src="/ui/icp_login.png" class="image_button" @click="authenticate_icp()" >
</template>

<script setup>
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { infinitinote_backend, canisterId, createActor } from '../../../declarations/infinitinote_backend';
import { inject, onMounted, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { Editor } from '@tiptap/core'
import Collaboration from '@tiptap/extension-collaboration'
import CollaborationCursor from '@tiptap/extension-collaboration-cursor'
import { HocuspocusProvider } from '@hocuspocus/provider'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
// Set up the Hocuspocus WebSocket provider
const provider = new HocuspocusProvider({
  url: 'ws://0.0.0.0:80',
  name: 'document',
})

const editor = useEditor({
  extensions: [
  StarterKit.configure({
      // The Collaboration extension comes with its own history handling
      history: false,
    }),
    Collaboration.configure({
      document: provider.document,
    }),
    CollaborationCursor.configure({
      provider: provider,
      user: {
        name: 'hira-'+new Date().toLocaleTimeString(),
        color: '#f783ac',
      },
    }),
  ],
})
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
.collaboration-cursor__label{
    font-size: 18px !important;
    width: 133px;
    height: 24px;
    border-radius: 30px;
    padding: 10px;
}
</style>