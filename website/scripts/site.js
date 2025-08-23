class PEuler {
    constructor() {
        this.initialized = this.init().then(() => {
            this.updatePicker();
            this.updateInfo();
        });
    }

    async init() {
        // PEulerWorker is a Web Worker that through which we can access the PEuler WebAssembly module
        this.peulerWorker = new PEulerWorker();

        // initialize basic problems data
        this.problems = new Map();
        const problems = await this.peulerWorker.problems();
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

        // add event listeners for the problem picker arrows
        document.getElementById("problem-picker-left-arrow").addEventListener("click", () => {
            this.pickerLeftClick();
        });
        document.getElementById("problem-picker-right-arrow").addEventListener("click", () => {
            this.pickerRightClick();
        });
        
        // add event listeners for all problem cells in the picker
        const t_body = document.querySelector("#problem-picker>table>tbody");
        let i = 0;
        for (const t_row of t_body.children) {
            for (const t_d of t_row.children) {
                const cellIndex = i; // capture the current value of i
                t_d.addEventListener("click", () => {
                    this.pickerProblemClick(this.currPickerProblemId + cellIndex);
                });
                i += 1;
            }
        }

        /*
        document.getElementById("problem-info").addEventListener("submit", async (e) => {
            e.preventDefault();
            const problemId = problemsSelect.value;
            console.log(await this.peulerWorker.solve(problemId));
        });
        */
    }


    async pickerLeftClick() {
        await this.initialized;
        if (this.currPickerProblemId > this.minProblemId) {
            this.currPickerProblemId -= 100;
            this.updatePicker();
        }
    }

    async pickerRightClick() {
        await this.initialized;
        if (this.currPickerProblemId + 100 <= this.maxProblemId) {
            this.currPickerProblemId += 100;
            this.updatePicker();
        }
    }

    async pickerProblemClick(clickedId) {
        if (this.problems.has(clickedId) && clickedId !== this.currProblemId) {
            console.log(clickedId);
        }
    }

    updatePicker() {
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

    updateInfo() {

    }
}

class PEulerWorker {
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
    
    async benchmark(problemId, iterations) {
        return this.sendJob({ workType: "benchmark", id: problemId, iterations: iterations });
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
