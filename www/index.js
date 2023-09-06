import * as wasm from "how-many-days-huh";
import { count_days } from "how-many-days-huh";

wasm.greet(`Your name ${count_days()}`);

console.log(count_days());
