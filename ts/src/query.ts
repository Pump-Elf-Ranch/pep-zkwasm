import {stringify} from "querystring";
import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {
  await player.installPlayer();

  await player.buy_elf(1n,1n,2n)
  for (let i = 0; i < 100000; i++) {
    let data = await player.getState();

    console.log("player info:",data);
    console.log(JSON.stringify(data));
    await wait(5000);
  }

  function wait(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  // let config = await player.getConfig();
  // console.log("config", config);
}

main();
