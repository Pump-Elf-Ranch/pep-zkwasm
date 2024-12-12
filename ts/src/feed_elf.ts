import {stringify} from "querystring";
import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    await player.feed_elf(1n,1n,4n)
    console.log("feed_elf ");
}

main();
