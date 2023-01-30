use std::cell::RefCell;
use std::rc::Rc;

use reqwest::Method;
use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::utilities::client::{BorrowClient, HttpRequest, RoboltClient};

pub struct UserBuilder {
    pub(crate) client: Rc<RefCell<RoboltClient>>,
}

impl UserBuilder {
    pub(crate) fn new(client: Rc<RefCell<RoboltClient>>) -> Self {
        Self { client }
    }

    pub fn fetch(&self, id: u64) -> Result<User, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/{}", ENDPOINTS.users, id),
            body: None,
        };

        self.client.request::<User>(req)
    }

    pub fn me(&self) -> Result<AuthenticatedUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/users/authenticated", ENDPOINTS.users),
            body: None,
        };

        self.client.request::<AuthenticatedUser>(req)
    }

    pub fn partial(&self, id: u64) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/users/{}", ENDPOINTS.base, id),
            body: None,
        };

        self.client.request::<PartialUser>(req)
    }

    pub fn find(&self, username: &str) -> Result<PartialUser, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!(
                "{}/users/get-by-username?username={}",
                ENDPOINTS.base, username
            ),
            body: None,
        };

        self.client.request::<PartialUser>(req)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "name")]
    pub username: String,
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: u64,
    pub display_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartialUser {
    pub username: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    #[serde(rename = "name")]
    pub username: String,
    pub display_name: String,
    pub id: u64,
}
