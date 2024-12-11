import {Player} from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {

    let config = await player.getConfig()
    console.log("config:", config);
}

main();
