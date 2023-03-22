import { createApp, ref } from 'vue';
import App from './App.vue';
import './styles/global.css';
import { QuillEditor } from '@vueup/vue-quill'
import '@vueup/vue-quill/dist/vue-quill.snow.css';
//import '../assets/sass/style.scss';

import { router } from './router';

const app = createApp(App);

var userPrincipal = ref(null);
var userAuthenticated = ref(false);
var backend = ref(null);
var notebooks = ref(null);
var notes = ref(null);
var selectedNotebook = ref(null);
var selectedNote = ref(null);
var isLoading = ref(false);

app.provide('userPrincipal', userPrincipal);
app.provide('userAuthenticated', userAuthenticated);
app.provide('backend', backend);
app.provide('notebooks', notebooks);
app.provide('notes', notes);
app.provide('selectedNotebook', selectedNotebook);
app.provide('selectedNote', selectedNote);
app.provide('isLoading', isLoading);

app.use(router);
app.component('QuillEditor', QuillEditor);

app.mount("#app");