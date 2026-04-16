pub const BLOG_TOKEN_KEY: &str = "blog_token";
pub const BLOG_USER_ID_KEY: &str = "blog_user_id";

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Auth {
    pub user_id: Option<String>,
    pub token: Option<String>,
}

impl Auth {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = browser_storage() {
                return Self {
                    user_id: storage.get_item(BLOG_USER_ID_KEY).ok().flatten(),
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
            }
        }
    }

    pub fn clear(&mut self) {
        self.user_id = None;
        self.token = None;
        self.save();
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub fn is_author_of(&self, author_id: &str) -> bool {
        self.is_authenticated() && self.user_id.as_deref() == Some(author_id)
    }
}

