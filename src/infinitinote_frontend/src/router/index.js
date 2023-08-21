import { createRouter, createWebHashHistory } from 'vue-router';

import Signup from '../components/Signup.vue'
import NotebooksHome from '../components/NotebooksHome.vue'
import NotebookAdd from '../components/NotebookAdd.vue';
import Notebook from '../components/Notebook.vue';
import Note from '../components/Note.vue';

export const routes = [
    { path: '/', component: Signup },
    { path: '/home', component: NotebooksHome },
    { path: '/add_notebook', component: NotebookAdd },
    { path: '/notebook/:notebookID', name: 'notebook', component: Notebook },
    { path: '/note/:notebookID/:noteID/', name: 'note', component: Note },
    { path: '/sharednote/:principalID/:notebookID/:noteID', name: 'sharednote', component: Note }
]

export const router = createRouter( {
    history: createWebHashHistory(),
    routes
});