//import { infinitinote_backend } from "../../declarations/infinitinote_backend";
import infinitinote_backend from "ic:canisters/infinitinote_backend";

export async function hello() {
    console.log("HELLO EARTHLING");
}

export async function greet() {
    console.log("GREETINGS EARTHLING");
    //await infinitinote_backend.greet('test');
}
console.log("BRIDGING");
window.backend = infinitinote_backend;