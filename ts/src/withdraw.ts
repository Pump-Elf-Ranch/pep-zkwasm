import {stringify} from "querystring";
import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    let eth_address = "0x1234";

    await player.withdrawRewards(eth_address,1n)
    console.log("buy_elf ");
}

main();