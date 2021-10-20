init();

async function init() {
    const { setup } = await import("./index");
    await setup();
}