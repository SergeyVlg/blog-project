pub const BLOG_TOKEN_KEY: &str = "blog_token";

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

pub fn save_token(token: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = browser_storage() {
            let _ = storage.set_item(BLOG_TOKEN_KEY, token);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = token;
    }
}

pub fn load_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        return browser_storage()?.get_item(BLOG_TOKEN_KEY).ok().flatten();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn clear_token() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = browser_storage() {
            let _ = storage.remove_item(BLOG_TOKEN_KEY);
        }
    }
}

