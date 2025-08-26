import init, { Sample } from "./build-wasm/wasm.js";

await init();


class PEuler {
    constructor() {
        this.initialized = this.init().then(() => {
            this.updatePicker().then(() => {});
            this.updateInfo().then(() => {});
        });
    }

    async init() {
        // a problem worker is a web worker for handling project euler problem-solving
        this.problemWorker = new ProblemWorker();

        // initialize basic problems data
        this.problems = new Map();
        const problems = await this.problemWorker.problems();
        if (problems.length === 0) {
            alert("No problems available");
            throw new Error("No problems available");
        }
        this.minProblemId = problems[0].id;
        this.maxProblemId = problems[0].id;
        for (const problem of problems) {
            this.problems.set(problem.id, problem.title);
            if (problem.id < this.minProblemId) {
                this.minProblemId = problem.id;
            }
            if (problem.id > this.maxProblemId) {
                this.maxProblemId = problem.id;
            }
        }
        this.currProblemId = this.minProblemId;

        // the first problem id in the page shown in the picker (this and 99 next problems == 100 problems per page)
        this.currPickerProblemId = Math.floor((this.currProblemId - 1) / 100) * 100 + 1;

        // initialize current problem solution
        this.currProblemSolution = null;

        // initialize benchmark data
        this.benchmarkRunning = false;
        this.benchmarkSample = new Sample();

        // initialize problem choice id,
        // used to ignore queued jobs after a problem switch
        // this can only change if the problem id changed, so it is enough to check this
        // (it is not necessary to check the inequality of the problem ids)
        this.problemChoiceId = 0;

        // add event listeners for the problem picker arrows
        document.getElementById("problem-picker-left-arrow").addEventListener("click", async () => {
            return this.pickerLeftClick();
        });
        document.getElementById("problem-picker-right-arrow").addEventListener("click", async () => {
            return this.pickerRightClick();
        });
        
        // add event listeners for all problem cells in the picker
        const t_body = document.querySelector("#problem-picker>table>tbody");
        let i = 0;
        for (const t_row of t_body.children) {
            for (const t_d of t_row.children) {
                const cellIndex = i; // capture the current value of i
                t_d.addEventListener("click", async () => {
                    return this.pickerProblemClick(this.currPickerProblemId + cellIndex);
                });
                i += 1;
            }
        }

        // add event listeners for the buttons
        document.getElementById("solve-btn").addEventListener("click", async () => {
            return this.solveButtonClick();
        });
        document.getElementById("benchmark-btn").addEventListener("click", async () => {
            return this.benchmarkButtonClick();
        });
        document.getElementById("reset-btn").addEventListener("click", async () => {
            return this.resetButtonClick();
        });
    }

    async pickerLeftClick() {
        await this.initialized;
        if (this.currPickerProblemId > this.minProblemId) {
            this.currPickerProblemId -= 100;
            await this.updatePicker();
        }
    }

    async pickerRightClick() {
        await this.initialized;
        if (this.currPickerProblemId + 100 <= this.maxProblemId) {
            this.currPickerProblemId += 100;
            await this.updatePicker();
        }
    }

    async pickerProblemClick(clickedId) {
        await this.initialized;
        if (this.problems.has(clickedId) && clickedId !== this.currProblemId) {
            this.problemChoiceId++;
            this.currProblemId = clickedId;
            this.currProblemSolution = null;
            this.benchmarkRunning = false;
            this.benchmarkSample.clear();
            document.getElementById("benchmark-btn").innerText = "Benchmark";
            document.getElementById("benchmark-loader").style.display = "none";
            await this.updatePicker();
            await this.updateInfo();
        }
    }
    
    async solveButtonClick() {
        await this.initialized;
        if (this.currProblemSolution === null) {
            let problem_choice_id = this.problemChoiceId;
            this.currProblemSolution = "pending";
            await this.updateInfoSolution();
            this.problemWorker.solve(this.currProblemId).then((solution) => {
                if (problem_choice_id === this.problemChoiceId) {
                    this.currProblemSolution = solution;
                    this.updateInfoSolution();
                }
            });
        }
    }
    
    async benchmarkButtonClick() {
        await this.initialized;
        if (!this.benchmarkRunning) {
            let problem_choice_id = this.problemChoiceId;
            this.benchmarkRunning = true;
            document.getElementById("benchmark-btn").innerText = "Stop";
            document.getElementById("benchmark-loader").style.display = "inline-block";
            if (this.currProblemSolution === null) {
                this.currProblemSolution = "pending";
                await this.updateInfoSolution();
            }
            while (this.benchmarkRunning && problem_choice_id === this.problemChoiceId) {
                let bench = await this.problemWorker.benchmark(this.currProblemId);
                if (this.currProblemSolution === "pending" && problem_choice_id === this.problemChoiceId) {
                    this.currProblemSolution = bench.result;
                    await this.updateInfoSolution();
                }
                if (!this.benchmarkRunning || problem_choice_id !== this.problemChoiceId) {
                    break;
                }
                this.benchmarkSample.push(bench.duration);
                await this.updateInfoBenchmark();
            }
            if (this.currProblemSolution === "pending" && problem_choice_id === this.problemChoiceId) {
                let solution = await this.problemWorker.solve(this.currProblemId);
                if (this.currProblemSolution === "pending" && problem_choice_id === this.problemChoiceId) {
                    this.currProblemSolution = solution;
                    await this.updateInfoSolution();
                }
            }
        } else {
            this.benchmarkRunning = false;
            document.getElementById("benchmark-btn").innerText = "Benchmark";
            document.getElementById("benchmark-loader").style.display = "none";
        }
    }
    
    async resetButtonClick() {
        await this.initialized;
        this.benchmarkSample.clear();
        return this.updateInfoBenchmark();
    }

    async updatePicker() {
        await this.initialized;
        // update arrows in the picker table header
        const leftArrow = document.getElementById("problem-picker-left-arrow");
        const rightArrow = document.getElementById("problem-picker-right-arrow");
        if (this.minProblemId < this.currPickerProblemId) {
            leftArrow["src"] = "images/left-arrow-green.svg";
            leftArrow.className = "enabled-arrow";
            leftArrow["alt"] = "Enabled left arrow";
        } else {
            leftArrow["src"] = "images/left-arrow-grey.svg";
            leftArrow.className = "disabled-arrow";
            leftArrow["alt"] = "Disabled left arrow";
        }
        if (this.maxProblemId >= this.currPickerProblemId + 100) {
            rightArrow["src"] = "images/right-arrow-green.svg";
            rightArrow.className = "enabled-arrow";
            rightArrow["alt"] = "Enabled right arrow";
        } else {
            rightArrow["src"] = "images/right-arrow-grey.svg";
            rightArrow.className = "disabled-arrow";
            rightArrow["alt"] = "Disabled right arrow";
        }

        // update table body
        const t_body = document.querySelector("#problem-picker>table>tbody");
        let i = this.currPickerProblemId;
        for (const t_row of t_body.children) {
            for (const t_d of t_row.children) {
                // t_d contains a <p> element, whose innerText we need to set to i
                t_d.firstElementChild.innerText = i.toString();
                
                t_d.id = "";
                t_d.title = "";
                t_d.classList.remove("available-problem", "unavailable-problem");
                if (this.problems.has(i)) {
                    if (i === this.currProblemId) {
                        t_d.id = "current-problem-cell";
                    } else {
                        t_d.classList.add("available-problem");
                    }
                    t_d.title = this.problems.get(i);
                } else {
                    t_d.classList.add("unavailable-problem");
                }

                i += 1;
            }
        }
    }

    async updateInfo() {
        await this.initialized;
        document.getElementById("problem-info-id").innerText = this.currProblemId;
        document.getElementById("problem-info-title").innerText = this.problems.get(this.currProblemId);
        document.getElementById("problem-info-link").href = "https://projecteuler.net/problem=" + this.currProblemId;
        document.getElementById("problem-info-link").title = `Problem ${this.currProblemId} on projecteuler.net`;
        document.querySelector("#problem-info-link>img").alt = `Link to Problem ${this.currProblemId} on projecteuler.net`;
        await this.updateInfoSolution();
        return this.updateInfoBenchmark();
    }
    
    async updateInfoSolution() {
        await this.initialized;
        if (this.currProblemSolution === null) {
            document.getElementById("problem-info-solution").style.display = "none";
            document.getElementById("solve-btn").style.display = "inline-block";
            document.getElementById("solve-loader").style.display = "none";
        } else if (this.currProblemSolution === "pending") {
            document.getElementById("problem-info-solution").style.display = "none";
            document.getElementById("solve-btn").style.display = "none";
            document.getElementById("solve-loader").style.display = "inline-block";
        } else {
            const solution = document.getElementById("problem-info-solution");
            solution.innerText = this.currProblemSolution;
            solution.style.display = "inline-block";
            document.getElementById("solve-btn").style.display = "none";
            document.getElementById("solve-loader").style.display = "none";
        }
    }
    
    async updateInfoBenchmark() {
        await this.initialized;
        if (this.benchmarkSample.is_empty()) {
            document.getElementById("problem-info-benchmark-iterations-label").style.display = "none";
            document.getElementById("problem-info-benchmark-iterations").style.display = "none";
            document.getElementById("problem-info-benchmark-mean-label").style.display = "none";
            document.getElementById("problem-info-benchmark-mean").style.display = "none";
            document.getElementById("problem-info-benchmark-stddev-label").style.display = "none";
            document.getElementById("problem-info-benchmark-stddev").style.display = "none";

            document.getElementById("reset-btn").style.display = "none";
        } else {
            let unit = "ns";

            let mean = this.benchmarkSample.mean();
            let stddev = this.benchmarkSample.stddev();
            if (stddev === undefined) {
                stddev = 0;
            } else {
                document.getElementById("problem-info-benchmark-stddev-label").style.display = "inline-block";
                document.getElementById("problem-info-benchmark-stddev").style.display = "inline-block";
            }

            if (mean >= 1000) {
                mean /= 1000;
                stddev /= 1000;
                unit = "µs";
            }
            if (mean >= 1000) {
                mean /= 1000;
                stddev /= 1000;
                unit = "ms";
            }
            if (mean >= 1000) {
                mean /= 1000;
                stddev /= 1000;
                unit = "s";
            }

            document.getElementById("problem-info-benchmark-iterations").innerText = (this.benchmarkSample.len()).toFixed(0);
            document.getElementById("problem-info-benchmark-mean").innerText = mean.toFixed(6).toString() + " " + unit;
            document.getElementById("problem-info-benchmark-stddev").innerText = stddev.toFixed(6).toString() + " " + unit;

            document.getElementById("problem-info-benchmark-iterations-label").style.display = "inline-block";
            document.getElementById("problem-info-benchmark-iterations").style.display = "inline-block";
            document.getElementById("problem-info-benchmark-mean-label").style.display = "inline-block";
            document.getElementById("problem-info-benchmark-mean").style.display = "inline-block";

            document.getElementById("reset-btn").style.display = "inline-block";
        }
    }
}

class ProblemWorker {
    constructor() {
        this.initialized = this.init();
    }

    async init() {
        this.worker = new Worker("./scripts/worker.js", { type: "module" });
        this.lastJob = new Promise((resolve) => { resolve(); }); // initially resolved promise
        return new Promise((resolve, reject) => {
            this.worker.onmessage = (e) => {
                if (e.data === "[Worker] Ready!") {
                    resolve();
                } else {
                    console.error("[Worker] Error: " + e.data);
                    reject(new Error("Worker initialization failed"));
                }
            };
            this.worker.onerror = (e) => {
                console.error("[Worker] Error: " + e.message);
                reject(e);
            };
        });
    }
    
    async problems() {
        return this.sendJob({ workType: "problems" });
    }

    async solve(problemId) {
        return this.sendJob({ workType: "solve", id: problemId });
    }
    
    async benchmark(problemId) {
        return this.sendJob({ workType: "benchmark", id: problemId });
    }
    
    async sendJob(msg) {
        await this.initialized;
        this.lastJob = this.lastJob.then(() => {
            return new Promise((resolve, reject) => {
                this.worker.onmessage = function(e) {
                    resolve(e.data);
                };
                this.worker.onerror = function(e) {
                    reject(e.message);
                };
                this.worker.postMessage(msg);
            });
        });
        return this.lastJob;
    }
}


new PEuler();
