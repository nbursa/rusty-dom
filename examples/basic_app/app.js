import init, { initialize, get_element_by_id, RustyElement } from './pkg/rusty_dom.js';

async function run() {
    console.log("Initializing WASM...");
    await init();
    await initialize();
    console.log("WASM Initialized");

    const initialState = {
        message: "Hello, World!",
        count: 0
    };

    const state = new State(initialState.message);
    console.log("Initial state:", state.get());

    const app = await get_element_by_id("app");

    const header = new RustyElement("h1");
    header.set_attribute("class", "header");
    header.set_text(state.get());
    app.append_child(header);

    const button = new RustyElement("button");
    button.set_text("Click me");
    button.set_attribute("class", "button");
    button.set_style("margin-top", "10px");

    const onClick = () => {
        state.set("Button clicked!");
        header.set_text(state.get());
    };

    const onClickClosure = Closure::wrap(Box::new(onClick) as Box<dyn FnMut()>);
    button.add_event_listener("click", onClickClosure);
    app.append_child(button);
}

run().catch(console.error);

