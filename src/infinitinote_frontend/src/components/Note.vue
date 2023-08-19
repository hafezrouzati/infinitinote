<script setup>
import { inject, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { onBeforeRouteLeave } from 'vue-router';
import { QuillEditor } from '@vueup/vue-quill'
import NoteTag from './NoteTag.vue';
import ChatBot from './ChatBot.vue';
import Collaboration from '@tiptap/extension-collaboration'
import TextAlign from '@tiptap/extension-text-align'
import Image from '@tiptap/extension-image'
import Dropcursor from '@tiptap/extension-dropcursor'
import CollaborationCursor from '@tiptap/extension-collaboration-cursor'
import { HocuspocusProvider } from '@hocuspocus/provider'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Paragraph from '@tiptap/extension-paragraph'
import Text from '@tiptap/extension-text'
import FontSize from 'tiptap-extension-font-size'
import FontFamily from '@tiptap/extension-font-family'
import TextStyle from '@tiptap/extension-text-style';
const provider = new HocuspocusProvider({
    url: 'ws://23.111.144.10:3001',
    name: 'document',
})
const fontFamilies = ['Arial', 'Helvetica', 'Times New Roman', 'Courier New'];
const editor = useEditor({
    extensions: [
        StarterKit.configure({
            // The Collaboration extension comes with its own history handling
            history: false,
        }),
        Collaboration.configure({
            document: provider.document,
        }),
        Image,
        FontFamily.configure({
            fontFamilies: fontFamilies,
        }),
        Paragraph,
        Text,
        TextStyle,
        Dropcursor,
        FontSize,
        TextAlign.configure({
            types: ['heading', 'paragraph'],
        }),
        CollaborationCursor.configure({
            provider: provider,
            user: {
                name: 'User -' + new Date().toLocaleTimeString(),
                color: '#f783ac',
            },
        })
    ],
})



const props = defineProps({
    notebook: Object,
    note: Object
});

const router = useRouter();
const route = useRoute();

var userAuthenticated = inject('userAuthenticated');
var backend = inject('backend');
var notebooks = inject('notebooks');
var isLoading = inject('isLoading');

var noteID = ref(route.params.noteID);
var notebookID = ref(route.params.notebookID);

var the_note = ref(null)
var files = ref([])
var the_notebook = ref(null);
var the_note_content = ref(null);

async function load_notebooks() {
    isLoading.value = true;
    let result = await backend.value.get_notebooks_for_principal();
    notebooks.value = result;
    isLoading.value = false;
}

async function background_update_note() {
    //todo: background update the note
}

async function update_note() {
    console.log(editor)
    // Get the plain text content from the editor
    const note_content = editor.value.getText();

    // Get the delta string from the editor
    const note_content_delta = editor.value.getJSON();

    const note_content_delta_string = JSON.stringify(note_content_delta);
    let result = await backend.value.update_note(
        the_notebook.value.id,
        the_note.value.id,
        the_note.value.title,
        note_content,
        note_content_delta_string,
        the_note.value.tags
    );
    console.log(result);
}


function addImage(editorParamter) {
    const url = window.prompt('URL')
    if (url) {
        editorParamter.chain().focus().setImage({ src: url }).run()
    }
}

function handleDrop(event) {
    event.preventDefault();
    const fileList = event.dataTransfer.files;

    for (let i = 0; i < fileList.length; i++) {
        const file = fileList[i];
        files.value.push(file);
    }
}
onMounted(async () => {

    if (userAuthenticated.value == false) {
        router.push('/');
    }
    else {
        console.log('load notebooks');
        isLoading.value = true;
        await load_notebooks();
        the_notebook.value = notebooks.value.find((n) => n.id == notebookID.value);
        console.log(the_notebook.value);
        the_note.value = the_notebook.value.notes.find((n) => n.id == noteID.value);

        if (the_note.value.content_delta != "") {
            try {
                var note_content_delta = JSON.parse(the_note.value.content_delta);
                console.log(editor)
                editor.value.commands.setContent(note_content_delta || []);
            } catch (err) {
                console.log(err);
                the_note_content.value = "";
            }
        } else {
            the_note_content.value = "";
        }

        isLoading.value = false;
    }
});

onBeforeRouteLeave(async (to, from, next) => {
    isLoading.value = true;
    if (userAuthenticated.value != false) {
        if (the_note.value != null) {
            await update_note();
            await load_notebooks();
        }
    }

    isLoading.value = false;

    next()
})
function formatFileSize(size) {
    if (size < 1024) {
        return size + ' B';
    } else if (size < 1048576) {
        return (size / 1024).toFixed(2) + ' KB';
    } else {
        return (size / 1048576).toFixed(2) + ' MB';
    }
}
function changeFontSize(event, editorParamter) {
    const fontSize = event.target.value;
    editorParamter.chain().focus().setFontSize(fontSize).run();
}

function changeFontFamily(event, editorParamter) {
    const fontSize = event.target.value;
    editorParamter.chain().focus().setFontFamily(fontSize).run();
}

</script>

<template>
    <div class='note-container'>
        <div class='note-header' v-if="the_notebook">
            <div class='note-notebook-title-container' v-if="the_notebook">
                <p class='note-notebook-title'>{{ the_notebook.title }}</p>
            </div>
            <div class='separator'>/</div>
            <div class='note-note-title-container'>
                <input type='text' class='note-title-input' v-model='the_note.title' v-if="the_note">
            </div>
        </div>
        <div class='note-content-container'>
            <div class="flex-1">
                <div class="note-editor-container">
                    <div class='note-editor'>
                        <!-- Formatting Section -->
                        <div class="note-editor-toolbar">
                            <div class="note-editor-toolbar-formatting">
                                <div class="custom-select">
                                    <select @change="changeFontFamily($event, editor)">
                                        <option v-for="fontFamily in fontFamilies" :value="fontFamily" :key="fontFamily">
                                            {{ fontFamily }}
                                        </option>
                                    </select>
                                </div>
                                <div class="custom-select">
                                    <select @change="changeFontSize($event, editor)">
                                        <option value="12px">12px</option>
                                        <option value="14px">14px</option>
                                        <option value="16px">16px</option>
                                        <option value="18px">18px</option>
                                    </select>
                                </div>
                                <span></span>
                                <img src="/ui/format_bold.svg" alt="" class="m-0"
                                    @click="editor.chain().focus().toggleBold().run()">
                                <img src="/ui/format_underlined.svg" alt=""
                                    @click="editor.chain().focus().toggleStrike().run()">
                                <img src="/ui/format_italic.svg" alt=""
                                    @click="editor.chain().focus().toggleItalic().run()">
                                <span></span>
                                <img src="/ui/format_align_left.svg" alt=""
                                    @click="editor.chain().focus().setTextAlign('left').run()">
                                <img src="/ui/format_align_center.svg" alt=""
                                    @click="editor.chain().focus().setTextAlign('center').run()">
                                <img src="/ui/format_align_right.svg" alt=""
                                    @click="editor.chain().focus().setTextAlign('right').run()">
                                <span></span>
                                <img src="/ui/add_photo_alternate.svg" alt="" @click="addImage(editor)">
                            </div>
                            <div class="note-editor-toolbar-share-icon">
                                <img src="/ui/share.svg" alt="">
                            </div>
                        </div>
                        <!-- <QuillEditor theme='snow' ref="myQuillEditor" v-model:content="the_note_content" @ready="$event => quillReady()"/> -->
                        <editor-content :editor="editor" />
                        <!-- Action buttons -->
                        <div class="note-action-buttons">
                            <button class="note-action-buttons-ok" on-click="update_note()">Save note</button>
                            <button class="note-action-buttons-cancel">Cancel</button>
                        </div>
                    </div>
                    <!-- Drag and drop section with file at buttom -->
                    <div class="note-drag-drop">
                        <div class="note-drag-drop-container" @dragover.prevent @drop="handleDrop">
                            <div class="note-drag-drop-content">
                                <div>
                                    <img src="/ui/document-text.svg" alt="">
                                </div>
                                <div class="note-drag-drop-text">
                                    <p>Drag & Drop your files here </p>
                                    <p>Available format: pdf, xls, txt</p>
                                </div>
                            </div>

                        </div>
                        <div class="note-drag-drop-files">
                            <div class="note-drag-drop-file" v-for="file in files" :key="file.name">
                                <img class="file-type" src="/ui/document-text.svg" alt="" srcset="">
                                <div>
                                    <p>{{ file.name }}</p>
                                    <p>{{ formatFileSize(file.size) }}</p>
                                </div>
                                <img class="delete" src="/ui/scan_delete.svg" alt="" srcset="" @click="removeFile(file)">
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="note-editor-tag">
                <NoteTag />
                <div class="h-10"></div>
                <ChatBot />
            </div>
        </div>

    </div>
</template>

<style>
* {
    font-family: Montserrat;

}

.note-header {
    display: flex;
    flex-direction: row;
}

.separator {
    font-family: montserrat;
    font-size: 26px;
    position: relative;
    margin-left: 22px;
    top: 11px;
}

.note-notebook-title-container {
    display: inline-block;
    width: auto;
    height: 50px;
    max-width: 500px;
    margin-left: 22px;
    border-radius: 11px;
    background-color: #FFFFFF;
    font-family: montserrat;
    font-size: 26px;
    color: #2B3249;
}

.note-notebook-title {
    position: relative;
    top: -33%;
    left: 0%;
}

.note-note-title-container {
    width: auto;
    max-width: 500px;
    height: 50px;
    margin-left: 22px;
    border-radius: 11px;
    background-color: #FFFFFF;
    font-family: montserrat;
    font-size: 26px;
    color: #2B3249;
    padding-left: 1%;
    padding-right: 1%;

}

.note-note-title {}

.note-title-input {
    appearance: none;
    border: none;
    outline: none;
    background-color: transparent;
    color: inherit;
    font-size: inherit;
    font-family: inherit;
    padding: 0;
    margin: 0;
    position: relative;
    top: 20%;
}

.note-content-container {
    position: relative;
    top: 26px;
    display: flex;
}

.note-editor-container {
    margin-left: 26px;
    margin-right: 26px;

    flex: 1;
}

.note-editor-tag {
    margin-right: 26px;

}

.note-editor {
    min-height: 500px;
    padding: 21px;
    background-color: #FFFFFF;
    border-radius: 25px;
}

/* Editor style override */
.ql-toolbar-override {
    /* border: 1px solid #d1d5db; */
    box-sizing: border-box;
    font-family: 'Helvetica Neue', 'Helvetica', 'Arial', sans-serif;
    padding: 8px;
}

.note-editor-toolbar {
    display: flex;
    justify-content: space-between;
    height: 55px;
    border-radius: 25px;
    background: #EDEFF3;
    padding-right: 17px;
}

.note-editor-toolbar-formatting img {
    margin-left: 18px;
}

.m-0 {
    margin-left: 0px !important;
}

.note-editor-toolbar-formatting span {
    width: 2px;
    height: 36px;
    opacity: 0.20000000298023224;
    background: #9FA3AF;
    margin: 0px 25px;
}

.note-editor-toolbar-formatting {
    display: flex;
    justify-content: start;
    align-items: center;
}

.note-editor-toolbar-share-icon {
    display: flex;
    justify-content: center;
    align-items: center;
}

.custom-select {
    margin-left: 20px;
    position: relative;
    display: inline-block;
}

.custom-select select {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-color: transparent;
    border: none;
    padding: 5px 30px 5px 10px;
    font-size: 16px;
    cursor: pointer;
}

.custom-select select::-ms-expand {
    display: none;
}

.custom-select::after {
    content: "\25BC";
    position: absolute;
    top: 58%;
    right: 8px;
    transform: translateY(-50%);
    pointer-events: none;
    font-size: 12px;
    color: #2261D3;
}

.note-editor-toolbar-share-icon img {
    height: 35px;
}

.note-action-buttons button {
    border: 0;
}

.note-action-buttons-ok {
    color: #FFF;
    text-align: center;
    font-size: 16px;
    font-weight: 600;
    width: 222px;
    height: 54px;
    border-radius: 14px;
    background: linear-gradient(226deg, #205EE7 0%, #21AFFF 100%), linear-gradient(228deg, #FFCB2C 0%, #E7960A 100%), #DBE0EA;
}

.note-action-buttons-cancel {
    text-align: center;
    font-size: 16px;
    background: white;
    margin-left: 50px;
    font-weight: 600;
}

.ProseMirror {
    min-height: 400px
}

.ProseMirror:focus-visible {
    outline: 0;
    box-shadow: none;
}

.note-drag-drop {
    padding-top: 1px;
    padding-bottom: 1px;
    height: 100%;
    background-color: white;
    border-radius: 25px;
    margin-top: 35px;
}

.note-drag-drop-container {
    border-radius: 25px;
    height: 210px;
    display: flex;
    opacity: 0.5;
    background: linear-gradient(207deg, #E1E4EB 0%, rgba(225, 228, 235, 0.00) 100%);
    margin: 15px 30px;
    border-style: dotted;
}

.note-drag-drop-content {
    display: flex;
    width: 100%;
    padding: 18px 80px;
    align-items: center;
}

.note-drag-drop-content img {
    margin-right: 51px;
}

.note-drag-drop-content p:nth-child(1) {
    color: #2B3249;
    font-size: 24px;
    font-style: normal;
    font-weight: 600;
    margin-bottom: 0px;
}

.note-drag-drop-content p:nth-child(2) {
    font-size: 14px;
    font-weight: 500;
    margin-top: 5px !important;
}

.note-drag-drop-files {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    align-items: flex-start;
    margin: 15px 30px;
}

.note-drag-drop-file {
    display: flex;
    align-items: center;
    border-radius: 25px;
    opacity: 0.5600000023841858;
    background: #E7EAEF;
    width: 328px;
    height: 99px;
    margin: 5px;
}

.note-drag-drop-file .file-type {
    height: 45px;
}

.note-drag-drop-file p:nth-child(1) {
    color: #2B3249;
    font-size: 16px;
    font-weight: 600;
}

.note-drag-drop-file p:nth-child(2) {
    margin-top: 5px;
    color: #C2C2C2;
    font-size: 14px;
    font-weight: 600;
}

.flex-1 {
    flex: 1;
}
.h-10{
    height: 20px;
}

.ql-container-override {}
</style>