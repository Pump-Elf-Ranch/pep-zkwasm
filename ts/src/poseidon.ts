// poseidon.js
import { poseidon } from "delphinus-curves/src/poseidon";
import { Field } from 'delphinus-curves/src/field';
import { BN } from "bn.js";

const input = [new Field(new BN("123")), new Field(new BN("456"))];
const hashResult = poseidon(input);
console.log(hashResult.toString());