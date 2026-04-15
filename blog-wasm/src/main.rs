#[cfg(target_arch = "wasm32")]
use blog_wasm::HomePage;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(HomePage);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("blog-wasm is intended to run in the browser via the wasm32-unknown-unknown target.");
}