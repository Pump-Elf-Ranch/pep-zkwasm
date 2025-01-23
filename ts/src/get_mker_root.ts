import {ZkWasmUtil} from "zkwasm-service-helper";
import { get_latest_proof } from "zkwasm-ts-server/src/prover.js";

import { U8ArrayUtil } from 'zkwasm-ts-server/src/lib.js';


async function main( ) {

    let taskid = "6792525a7893e0306e5b4cb8"
    let task = await get_latest_proof(taskid);
    console.log("latest taskId got from remote:", task?._id);
    console.log("latest task", task?.instances);
    if (task) {
        const instances = ZkWasmUtil.bytesToBN(task?.instances);
        console.log("instances", instances);
        let instArr = [new U8ArrayUtil(task?.instances).toNumber()];
        let merkle_root_new = BigInt(instArr[0][0]) << BigInt(192) |
            BigInt(instArr[0][1]) << BigInt(128) |
            BigInt(instArr[0][2]) << BigInt(64) |
            BigInt(instArr[0][3]);
        console.log(merkle_root_new.toString());
    }
}

main();

