#[cfg(target_arch = "wasm32")]
use blog_wasm::App;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("blog-wasm is intended to run in the browser via the wasm32-unknown-unknown target.");
}

