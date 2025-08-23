import init, { PEuler } from "./build-peuler-wasm/wasm.js";

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
            postMessage(peuler.benchmark(e.data.id, e.data.iterations));
            break;
        default:
            throw new Error("[Worker] Unknown work type: " + e.data.workType);
    }
};

postMessage("[Worker] Ready!");
