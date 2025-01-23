import { Player } from "./api.js";

let account = "1234";
let player = new Player(account);

async function main() {
  let config = await player.getConfig();
  console.log("config", config);

  console.log("install player ...\n");
  await player.installPlayer();



  console.log("install object ...\n");
  // await player.installObject(0n, [0n, 0n, 0n, 0n, 0n, 0n, 0n, 0n]);



  let state = await player.getState();
  console.log("query state:", state);

  console.log("withdraw:\n");
  await player.withdrawRewards("c177d1d314C8FFe1Ea93Ca1e147ea3BE0ee3E470", 1n);
}

main();
