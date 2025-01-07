import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    await player.buy_slot(1n)
    console.log("buy_slot ");
}

main();
