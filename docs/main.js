import init, { run_code } from "./pkg/spemath.js";

async function main() {
    await init();

    const input = document.getElementById("input");
    const output = document.getElementById("output");
    const run = document.getElementById("run");

    run.addEventListener("click", () => {
        try {
            const result = run_code(input.value);
            output.textContent = result;
        } catch (err) {
            output.textContent = "WASM Error: " + err;
        }
    });
}

main();
