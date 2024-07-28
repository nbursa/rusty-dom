import init, { RustyElement, State, render_to_document } from './pkg/rusty_dom.js';

async function run() {
    console.log("Initializing WASM...");
    await init();
    console.log("WASM Initialized");

    const initialState = {
        message: "Hello, World!",
        count: 0
    };

    const state = new State(JSON.stringify(initialState));
    console.log("Initial state:", state.get());
    const parsedState = JSON.parse(state.get());
    console.log("Parsed initial state:", parsedState);

    let app = new RustyElement("div");
    let header = new RustyElement("h1");
    console.log("Header element created with pointer:", header.__wbg_ptr);

    if (parsedState && parsedState.message) {
        console.log("Setting text to:", parsedState.message);
        header.set_text(parsedState.message);
        console.log("Header element after set_text:", header);
    } else {
        console.error("Parsed state or message is invalid:", parsedState);
    }

    app.append_child(header);
    render_to_document(app);

    const updatedState = {
        message: "Hello, RustyDOM!",
        count: 1
    };

    state.set(JSON.stringify(updatedState));
    console.log("Updated state:", state.get());
    const parsedUpdatedState = JSON.parse(state.get());
    console.log("Parsed updated state:", parsedUpdatedState.message);

    if (parsedUpdatedState && parsedUpdatedState.message) {
        console.log("Setting text to:", parsedUpdatedState.message);
        try {
            header = new RustyElement("h1"); // Re-create the header element
            header.set_text(parsedUpdatedState.message);
            console.log("Header element after set_text:", header);
            app = new RustyElement("div"); // Re-create the app element
            app.append_child(header);
        } catch (e) {
            console.error("Error setting text:", e);
        }
    } else {
        console.error("Parsed updated state or message is invalid:", parsedUpdatedState);
    }

    render_to_document(app);
}

run().catch(console.error);

