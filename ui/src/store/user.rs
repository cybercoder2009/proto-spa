use std::rc::Rc;
use common::rest::login::PostResp;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub const KEY_USER: &str = "user";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub token: String,
    #[serde(flatten)]
    pub data: PostResp,
    #[serde(default)]
    pub exp: usize,
}

impl User {
    pub fn is_logged_in(&self) -> bool {
        !self.token.is_empty()
    }

    pub fn username(&self) -> &str {
        &self.data.username
    }

    pub fn display_name(&self) -> &str {
        if !self.data.username.is_empty() {
            &self.data.username
        } else {
            "User"
        }
    }

    /// Save the JWT token string into localStorage under key 'user'
    pub fn save(&self) {
        if self.is_logged_in() {
            crate::storage::write(KEY_USER, Some(&self.token));
        } else {
            crate::storage::write(KEY_USER, None);
        }
    }
}

pub enum UserAction {
    SetJwt(String),
    Logout,
}

impl Reducible for User {
    type Action = UserAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            UserAction::SetJwt(jwt) => {
                let mut user = crate::utilities::from_jwt::<User>(&jwt).unwrap_or_default();
                user.token = jwt;
                user.save();
                user
            }
            UserAction::Logout => {
                let empty = User::default();
                empty.save();
                empty
            }
        };
        Rc::new(next_state)
    }
}

pub type UserContext = UseReducerHandle<User>;
