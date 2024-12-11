import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    await player.buy_prop(1n,4n)
    console.log("buy_prop ");
}

main();
