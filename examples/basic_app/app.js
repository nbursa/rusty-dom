import init, { RustyElement, Router, State, render_to_document } from './pkg/basic_app.js';

async function run() {
    await init();

    const state = State.new("Hello, World!");

    const app = RustyElement.new("div");
    const header = RustyElement.new("h1");
    header.set_text(state.get());

    app.append_child(header);

    render_to_document(app);
}

run();

