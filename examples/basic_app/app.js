import init, { Element, Router, State } from './pkg/basic_app.js';

async function run() {
    await init();

    const state = State.new("Hello, World!");

    const app = Element.new("div");
    const header = Element.new("h1");
    header.set_text(state.get());

    app.append_child(header);

    document.getElementById("app").innerHTML = app.render();

    state.set("Hello, RustyDOM!");
    header.set_text(state.get());

    document.getElementById("app").innerHTML = app.render();
}

run();

