import { canisterId, createActor, infinitinote_backend } from "../../declarations/infinitinote_backend";
import { AuthClient } from "@dfinity/auth-client";
//import * as wasm from "../../../dist/infinitinote_frontend/infinitinote_frontend";
//import infinitinote_backend from "ic:canisters/infinitinote_backend";
// import { Actor, HttpAgent } from '@dfinity/agent';
// import { Principal } from '@dfinity/principal';
// import { AuthClient } from '@dfinity/auth-client';


console.log("INITBRIDGE");
window.backend = infinitinote_backend;

window.dataStore = {};

window.dataStore.principals = {}
window.dataStore.notebooks = [];

window.backend_result = "";

export async function backend_hello() {
    console.log("GREETING EARTHLING");
}

window.call_backend_one = async function(funcName) {
    var result;
    result = await window["backend"][funcName]();
}

window.call_backend_func = async function(...args) {
    var result;
    if (args.length == 1)
    {
        result = await window["backend"][args[0]]();
    }
    else if (args.length == 2)
    {
        result = await window["backend"][args[0]](args[1]);
    }
    else if (args.length == 3)
    {
        result = await window["backend"][args[0]](args[1], args[2]);
    }
    else if (args.length == 4)
    {
        result = await window["backend"][args[0]](args[1], args[2], args[3]);
    }

    return result;
}

window.authClient = {};
window.userAuthenticated = false;
window.userPrincipal = "";

window.getUserPrincipal = function() {
    return window.userPrincipal;
}

window.onMount = async function() {
    window.authClient = await AuthClient.create();  
}

window.onAuthenticated = async function() {
    userAuthenticated = true;

    let userIdentity = await authClient.getIdentity();
    window.userPrincipal = userIdentity.getPrincipal().toString();

    var actorOptions = {
        agentOptions: {
            identity: userIdentity,
        }
    };

    var newbackend = createActor(canisterId, actorOptions);

    console.log(newbackend);

    window.backend = newbackend;

    //userID.value = userIdentity.getPrincipal().toString();
    //userPrincipal.value = userIdentity.getPrincipal();

    console.log("USER ID");
    console.log(window.userPrincipal);
};

window.authenticate = async function() {
        await authClient.login({
            identityProvider: process.env.II_URL,
            onSuccess: async () => {
              window.onAuthenticated(authClient);
            },
        });
};

window.get_notebooks_for_principal = async function() {
    let result = await window.backend.get_notebooks_for_principal();
    console.log("Got Notebooks");
    window.dataStore.notebooks = result;
    return result;
};

window.get_notebooks = function() {
    return window.dataStore.notebooks;
}

window.add_notebook_for_principal = async function(notebook_title)
{
    let result = await window.backend.add_notebook_for_principal(window.userPrincipal, notebook_title);
    console.log("added notebook");
    console.log(result);
};

window.add_note_to_notebook = async function(principal_id, notebook_id, note_title, note_text) {
    var notebook = window.dataStore.principals[principal_id].notebooks[notebook_id];
    var note = {
        id : "000",
        title : note_title,
        text : note_text
    };
    notebook.notes.push(note);
};

window.get_notes_for_notebook = async function(principal_id, notebook_id)
{
    var notebook = window.dataStore.principals[principal_id].notebooks[notebook_id];
    var notes = notebook.notes;
    return notes;
};

window.backend_hello = function() {
    console.log("HELLO SPOCK");
    window.backend.greet("PUDDING").then((result) => { window.backend_result = result; });
}

window.get_backend_result = function() {
    return window.backend_result;
}

window.backend_promise = async function() {
    var result = await window.backend.greet("YELLOW");
    return result;
}

window.backend_principal = async function() {
    var result = await window.backend.test_principal();
    return result;
}

export async function greet() {
    console.log("GREETINGS EARTHLING");
    await infinitinote_backend.greet('test');
}

window.test_create_notebook = async function() {
    let result = await window.backend.create_notebook("Test Notebook");
    console.log(result);
}

window.test_add_note = async function() {
    let notebook_id = await window.backend.create_notebook("Test Notebook");

    let note_title = "Test Title";
    let note_content = "Test Content";
    let note_tags = ["test tag 1", "test tag 2", "test tag 3"];

    let note_result = await window.backend.add_note_to_notebook(notebook_id, note_title, note_content, note_tags);
    console.log(note_result);

    let notes_result = await window.backend.get_notes_for_notebook(notebook_id);
    console.log(notes_result);
}

window.addEventListener('load', window.onMount);

