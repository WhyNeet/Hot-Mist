#![allow(unsafe_op_in_unsafe_fn)]

wit_bindgen::generate!();

struct HelloWorldComponent;

impl Guest for HelloWorldComponent {
    fn handler() -> String {
        println!("Hello, world!");

        "Hello there!".to_string()
    }
}

export!(HelloWorldComponent);
