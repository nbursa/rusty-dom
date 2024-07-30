# RustyDOM

My learning project RustyDOM is a library for creating and manipulating DOM elements using Rust and WebAssembly. This library aims to provide a simple and efficient way to manage state and interact with the DOM in web applications.

Note: This library is a work in progress. Some features may not be fully implemented or tested yet.

## Features

- Create and manipulate DOM elements
- Manage state efficiently
- Easy integration with Rust and WebAssembly
- Minimal and efficient API

## Prerequisites

Ensure you have the following installed:

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- `wasm-pack`: [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Node.js and npm: [Install Node.js](https://nodejs.org/)

## Getting Started

### Setup

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/nbursa/rusty-dom.git
cd rusty-dom
```

### Building the Project

To build the project, run:

```sh
wasm-pack build --target web
```

### Running an Example

Navigate to the `examples/basic_app` directory and start a local server:

```sh
cd examples/basic_app
http-server .
```

Open your browser and navigate to `http://127.0.0.1:8080`. You should see the example application running.

## Usage

The main logic of the library is in the `src` directory. Here is an overview of the core functionality:

### `src/lib.rs`

```rust
extern crate wasm_bindgen;
extern crate wee_alloc;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod element;
pub mod state;
mod router;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize logging
    console_error_panic_hook::set_once();

    Ok(())
}
```

### Creating and Manipulating Elements

The `element` module provides a simple API for creating and manipulating DOM elements. Here is an example:

```rust
use rusty_dom::element::Element;
use rusty_dom::state::State;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let state = State::new("Hello, World!".to_string());

    let mut app = Element::new("div");
    let mut header = Element::new("h1");
    header.set_text(&state.get());

    app.append_child(header.clone());

    let rendered = app.render();
    let document = window().unwrap().document().unwrap();
    let app_container = document.get_element_by_id("app").unwrap();
    app_container.set_inner_html(&rendered);

    state.set("Hello, RustyDOM!".to_string());
    header.set_text(&state.get());
    app_container.set_inner_html(&app.render());

    Ok(())
}
```

### Managing State

The `state` module provides a simple API for managing state. Here is an example:

```rust
use rusty_dom::state::State;

let state = State::new("Hello, World!".to_string());
println!("{}", state.get()); // Prints "Hello, World!"
state.set("New State".to_string());
println!("{}", state.get()); // Prints "New State"
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you encounter any problems or have suggestions for improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

