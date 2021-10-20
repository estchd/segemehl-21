let generate_stats_func;

let initialized = false;

async function initialize() {
    const {
        default: init,
        generate_file_stats
    } = await import("./../../pkg/segemehl_21_frontend.js");

    await init();
    generate_stats_func = generate_file_stats;
    initialized = true;
}

addEventListener("message", async (e) => {
    if (!initialized) { await initialize();}

    for (const file of e.data.files) {

        let result = generate_stats_func(file.buffer);
        result = JSON.parse(result);

        if (result.result === "success") {
            self.postMessage(
                {
                    type: "success",
                    files: [{
                        name: file.name,
                        stats: result.data.stats,
                        data: result.data.data,
                        references: result.data.references
                    }]
                }
            )
        }
        else if (result.result === "error") {
            self.postMessage({
                type: "error",
                files: [{
                    name: file.name,
                    err: result.data
                }]
            })
        }
    }
});