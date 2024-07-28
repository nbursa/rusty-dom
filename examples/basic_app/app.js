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
    app.set_attribute("id", "app");

    let header = new RustyElement("h1");
    header.set_attribute("class", "header");
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
            let newHeader = new RustyElement("h1");
            newHeader.set_attribute("class", "header");
            newHeader.set_text(parsedUpdatedState.message);
            console.log("Header element after set_text:", newHeader);
            app = new RustyElement("div");
            app.set_attribute("id", "app");
            app.append_child(newHeader);
            render_to_document(app);
        } catch (e) {
            console.error("Error setting text:", e);
        }
    } else {
        console.error("Parsed updated state or message is invalid:", parsedUpdatedState);
    }
}

run().catch(console.error);

