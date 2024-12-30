import {stringify} from "querystring";
import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    await player.sell_elf(1n,1n)
    console.log("sell_elf ");
}

main();
