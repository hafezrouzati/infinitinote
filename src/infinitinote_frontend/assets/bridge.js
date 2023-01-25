import { infinitinote_backend } from "../../declarations/infinitinote_backend";

export async function greet() {
    await infinitinote_backend.greet('test');
}