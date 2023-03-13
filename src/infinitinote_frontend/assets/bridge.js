import { infinitinote_backend } from "../../declarations/infinitinote_backend";
//import infinitinote_backend from "ic:canisters/infinitinote_backend";
// import { Actor, HttpAgent } from '@dfinity/agent';
// import { Principal } from '@dfinity/principal';
// import { AuthClient } from '@dfinity/auth-client';

console.log("INITBRIDGE");
window.backend = infinitinote_backend;

window.dataStore = {};

window.dataStore.notebooks = [];

window.backend_result = "";

export async function backend_hello() {
    console.log("GREETING EARTHLING");
}

window.call_backend_one = async function(funcName) {
    var result;
    result = await window["backend"][funcName]();
}

window.call_backend_two

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

window.get_notebooks_for_principal = async function(principal_id) {
    if (window.dataStore.notebooks == null)
    {
        window.dataStore.notebooks = [];
    }
};

window.add_notebook_for_principal = async function(principal_id, notebook_title)
{
    let notebook_id = "test" + Math.floor(Math.random() * 1000);

    var notebook = {
        uuid : "test000",
        title : notebook_title
    }

    window.dataStore.notebooks.append(notebook);
}


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

