pub const BLOG_TOKEN_KEY: &str = "blog_token";
pub const BLOG_USER_ID_KEY: &str = "blog_user_id";
pub const BLOG_USER_NAME_KEY: &str = "blog_user_name";

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Auth {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub token: Option<String>,
}

impl Auth {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = browser_storage() {
                return Self {
                    user_id: storage.get_item(BLOG_USER_ID_KEY).ok().flatten(),
                    user_name: storage.get_item(BLOG_USER_NAME_KEY).ok().flatten(),
                    token: storage.get_item(BLOG_TOKEN_KEY).ok().flatten(),
                };
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = browser_storage() {
                match &self.token {
                    Some(token) => {
                        let _ = storage.set_item(BLOG_TOKEN_KEY, token);
                    }
                    None => {
                        let _ = storage.remove_item(BLOG_TOKEN_KEY);
                    }
                }

                match &self.user_id {
                    Some(user_id) => {
                        let _ = storage.set_item(BLOG_USER_ID_KEY, user_id);
                    }
                    None => {
                        let _ = storage.remove_item(BLOG_USER_ID_KEY);
                    }
                }

                match &self.user_name {
                    Some(user_name) => {
                        let _ = storage.set_item(BLOG_USER_NAME_KEY, user_name);
                    }
                    None => {
                        let _ = storage.remove_item(BLOG_USER_NAME_KEY);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.user_id = None;
        self.user_name = None;
        self.token = None;
        self.save();
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

