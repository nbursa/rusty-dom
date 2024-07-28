import init, { initialize, create_header, State, get_element_by_id } from './pkg/rusty_dom.js';

async function run() {
    await init();
    initialize();

    const initialState = {
        message: "Hello, World!",
        count: 0
    };

    const state = new State(JSON.stringify(initialState));

    let app = get_element_by_id("app");

    let header = create_header(state);

    app.append_child(header);

    const updatedState = {
        message: "Hello, RustyDOM!",
        count: 1
    };

    state.set(JSON.stringify(updatedState));
    const parsedUpdatedState = JSON.parse(state.get());

    header.set_text(parsedUpdatedState.message);
}

run().catch(console.error);

