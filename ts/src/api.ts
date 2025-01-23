import { query, LeHexBN, ZKWasmAppRpc } from "zkwasm-ts-server";
import BN from "bn.js";

/* The modifier mush less than eight */
function encode_modifier(modifiers: Array<bigint>) {
  let c = 0n;
  for (const m of modifiers) {
    c = (c << 8n) + m;
  }
  return c;
}

function bytesToHex(bytes: Array<number>): string  {
    return Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
}

function addrToParams(bn: BN): Array<bigint> {
  // address is encoded in BigEndian
  const mask = new BN('ffffffffffffffff', 16);
  let a = bn.and(mask).toArray('le', 8);
  let b = bn.shrn(64).and(mask).toArray('le', 8);
  let c = bn.shrn(128).and(mask).toArray('le', 8);
  let aHex = a.map(byte => byte.toString(16).padStart(2, '0')).join('');
  let bHex = b.map(byte => byte.toString(16).padStart(2, '0')).join('');
  let cHex = c.map(byte => byte.toString(16).padStart(2, '0')).join('');
  return [BigInt(`0x${cHex}`), BigInt(`0x${bHex}`), BigInt(`0x${aHex}`)];
}

const CMD_INSTALL_PLAYER = 1n;
const CMD_BUY_ELF = 2n;
const CMD_SELL_ELF = 6n;
const CMD_CLEAN_RANCH = 4n;
const CMD_COLLECT_GOLD = 11n;
const CMD_WITHDRAW = 7n;
const CMD_PROP = 12n;
const CMD_BUY_SLOT = 13n;

const CMD_FEED_ELF = 3n; // 喂食精灵
const  CMD_TREAT_ELF = 5n; // 治疗宠物
function createCommand(nonce: bigint, command: bigint, objindex: bigint) {
  return (nonce << 16n) + (objindex << 8n) + command;
}

const rpc = new ZKWasmAppRpc("https://zk-server.pumpelf.ai");
// const rpc = new ZKWasmAppRpc("http://127.0.0.1:3000");

export class Player {
  processingKey: string;
  constructor(key: string) {
    this.processingKey = key
  }

  async getConfig(): Promise<any> {
    let config = await rpc.query_config();
    return config;
  }

  async getState(): Promise<any> {
    // Get the state response
    let state = await rpc.queryState(this.processingKey);

    // Parse the response to ensure it is a plain JSON object
    const parsedState = JSON.parse(JSON.stringify(state));

    // Extract the data from the parsed response
    const data = JSON.parse(parsedState.data);

    return data;
  }

  async getNonce(): Promise<bigint> {
    const data = await this.getState();
    let nonce = BigInt(data.player.nonce);
    return nonce;
  }

  async installPlayer() {
    try {
      let finished = await rpc.sendTransaction(
        new BigUint64Array([createCommand(0n, CMD_INSTALL_PLAYER, 0n), 0n, 0n, 0n]),
        this.processingKey
      );
      console.log("installPlayer processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("installPlayer error at processing key:", this.processingKey);
    }
  }

  async collectGold(ranch_id: bigint, elf_id: bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_COLLECT_GOLD, 0n), ranch_id, elf_id, 0n]),
          this.processingKey
      );
      console.log("collectGold processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("collectGold error at processing key:", this.processingKey);
    }
  }

  async cleanRanch(ranch_id: bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_CLEAN_RANCH, 0n), ranch_id, 0n, 0n]),
          this.processingKey
      );
      console.log("cleanRanch processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("cleanRanch error at processing key:", this.processingKey);
    }
  }

  async buy_elf( ranch_id: bigint,elf_type:bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
        new BigUint64Array([createCommand(nonce, CMD_BUY_ELF, 0n), ranch_id, elf_type, 0n]),
        this.processingKey
      );
      console.log("buy_elf processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("buy_elf error at ranch_id:", ranch_id, "elf_type :", elf_type);
    }
  }

  async sell_elf( ranch_id: bigint,elf_id:bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_SELL_ELF, 0n), ranch_id, elf_id, 0n]),
          this.processingKey
      );
      console.log("sell_elf processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("sell_elf error at ranch_id:", ranch_id, "elf_id :", elf_id);
    }
  }

  async buy_prop( ranch_id: bigint,prop_type:bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_PROP, 0n), ranch_id, prop_type, 0n]),
          this.processingKey
      );
      console.log("buy_prop processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("buy_prop error at ranch_id:", ranch_id, "prop_type :", prop_type);
    }
  }

  async buy_slot( ranch_id: bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_BUY_SLOT, 0n), ranch_id, 0n, 0n]),
          this.processingKey
      );
      console.log("buy_slot processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("buy_slot error at ranch_id:", ranch_id);
    }
  }

  async feed_elf( ranch_id: bigint,elf_id:bigint,prop_type:bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_FEED_ELF, 0n), ranch_id, elf_id, prop_type]),
          this.processingKey
      );
      console.log("feed_elf processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("feed_elf error at ranch_id:", ranch_id, "prop_type :", prop_type);
    }
  }

  async treat_elf( ranch_id: bigint,elf_id:bigint,prop_type:bigint) {
    let nonce = await this.getNonce();
    console.log("nonce :",nonce)
    try {
      let finished = await rpc.sendTransaction(
          new BigUint64Array([createCommand(nonce, CMD_TREAT_ELF, 0n), ranch_id, elf_id, prop_type]),
          this.processingKey
      );
      console.log("treat_elf processed at:", finished);
    } catch(e) {
      if(e instanceof Error) {
        console.log(e.message);
      }
      console.log("treat_elf error at ranch_id:", ranch_id, "prop_type :", prop_type);
    }
  }








  /*
    (32 bit amount | 32 bit highbit of address)
    (64 bit mid bit of address (be))
    (64 bit tail bit of address (be))
       */
  async withdrawRewards(address: string, amount: bigint) {
    let nonce = await this.getNonce();
    let addressBN = new BN(address, 16);
    let a = addressBN.toArray("be", 20); // 20 bytes = 160 bits and split into 4, 8, 8
    console.log("address is", address);
    console.log("address be is", a);

    let firstLimb = BigInt('0x' + bytesToHex(a.slice(0,4).reverse()));
    let sndLimb = BigInt('0x' + bytesToHex(a.slice(4,12).reverse()));
    let thirdLimb = BigInt('0x' + bytesToHex(a.slice(12, 20).reverse()));


    console.log("first is", firstLimb);
    console.log("snd is", sndLimb);
    console.log("third is", thirdLimb);

    try {
      let processStamp = await rpc.sendTransaction(
        new BigUint64Array([
          createCommand(nonce, CMD_WITHDRAW, 0n),
          (firstLimb << 32n) + amount,
          sndLimb,
          thirdLimb
        ]), this.processingKey);
      console.log("withdraw rewards processed at:", processStamp);
    } catch(e) {
      if (e instanceof Error) {
        console.log(e.message);
      }
      console.log("collect reward error at address:", address);
    }
  }
}
