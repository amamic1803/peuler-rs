import init, { PEuler } from "./build-wasm/wasm.js";

await init();
let peuler = new PEuler();

onmessage = (e) => {
    switch (e.data.workType) {
        case "problems":
            postMessage(peuler.problems());
            break;
        case "solve":
            postMessage(peuler.solve(e.data.id));
            break;
        case "benchmark":
            postMessage(peuler.benchmark(e.data.id));
            break;
        default:
            throw new Error("Unknown work type: " + e.data.workType);
    }
};

postMessage("ready");
