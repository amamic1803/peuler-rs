import init, { PEuler } from "./build-peuler-wasm/wasm.js";



await init();
let peuler = new PEuler();



class ProblemsPicker {
    constructor(peuler) {
        this.problems = new Map();
        this.minProblemId = 1;
        this.maxProblemId = 1;
        for (const problem of peuler.problems()) {
            this.problems.set(problem.id, problem.title);
            if (problem.id < this.minProblemId) {
                this.minProblemId = problem.id;
            }
            if (problem.id > this.maxProblemId) {
                this.maxProblemId = problem.id;
            }
        }
        this.currProblemId = this.minProblemId;
        document.getElementById("problems-picker-left-arrow").addEventListener("click", () => {
            this.left();
        });
        document.getElementById("problems-picker-right-arrow").addEventListener("click", () => {
            this.right();
        });
        this.update();
    }

    right() {
        if (this.currProblemId + 100 <= this.maxProblemId) {
            this.currProblemId += 100;
            this.update();
        }
    }

    left() {
        if (this.currProblemId > this.minProblemId) {
            this.currProblemId -= 100;
            this.update();
        }
    }
    
    update() {
        // update arrows in the table header
        const leftArrow = document.getElementById("problems-picker-left-arrow");
        const rightArrow = document.getElementById("problems-picker-right-arrow");
        if (this.minProblemId < this.currProblemId) {
            leftArrow["src"] = "images/left-arrow-green.svg";
            leftArrow.className = "enabled-arrow";
            
        } else {
            leftArrow["src"] = "images/left-arrow-grey.svg";
            leftArrow.className = "disabled-arrow";
        }
        if (this.maxProblemId >= this.currProblemId + 100) {
            rightArrow["src"] = "images/right-arrow-green.svg";
            rightArrow.className = "enabled-arrow";
            
        } else {
            rightArrow["src"] = "images/right-arrow-grey.svg";
            rightArrow.className = "disabled-arrow";
        }
        
        // update table body
        const table_body = document.querySelector("#problems-picker>table>tbody");
        while (table_body.firstChild) {
            table_body.removeChild(table_body.firstChild);
        }
        let table_row = document.createElement("tr");
        for (let i = this.currProblemId; i <= this.currProblemId + 100; i++) {
            const table_cell_p = document.createElement("p");
            table_cell_p.innerText = i.toString();
            
            const table_cell = document.createElement("td");
            table_cell.appendChild(table_cell_p);
            
            if (this.problems.has(i)) {
                table_cell.className = "available-problem";
                table_cell.title = this.problems.get(i);
                table_cell.addEventListener("click", function() {
                    console.log(i);
                });
            } else {
                table_cell.className = "unavailable-problem";
            }
            
            table_row.appendChild(table_cell);
            
            if (i % 10 === 0) {
                table_body.appendChild(table_row);
                table_row = document.createElement("tr");
            }
        }
    }
}



let problemsPicker = new ProblemsPicker(peuler);



const problemsSelect = document.getElementById("problems-select");
for (const problem of peuler.problems()) {
    let option = document.createElement("option");
    option.value = problem.id;
    option.innerText = problem.id;
    problemsSelect.appendChild(option);
}

document.getElementById("runner").addEventListener("submit", function(event) {
    event.preventDefault();
    console.log("Login form submitted");
    const problemId = problemsSelect.value;
    console.log(peuler.solve(problemId));
});


