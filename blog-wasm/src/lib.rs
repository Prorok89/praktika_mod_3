use blog_client::{AuthResponse, BlogClient, Post, Transport};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use yew::{Callback, MouseEvent};

const SERVER_URL: &str = "http://localhost:8089";


const TOKEN_KEY: &str = "blog_token";
const USER_ID_KEY: &str = "blog_user_id";

#[derive(Clone, PartialEq)]
pub struct NotificationContext {
    pub notify: Callback<Notification>,
}

#[derive(Clone, PartialEq)]
pub enum MessageKind {
    Success,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
	pub user_id: Option<i64>,
	pub username : String,
    pub is_logged_in: bool,
    pub logout: Callback<MouseEvent>,
    pub login: Callback<AuthResponse>,
}

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub text: String,
    pub kind: MessageKind,
}
#[wasm_bindgen]
pub async fn register_wasm(username: &str, email: &str, password: &str) -> Result<JsValue, JsValue> {

    let mut client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));

    let res = client
        .register(username.to_string(), email.to_string(), password.to_string())
        .await;

    match res {
        Ok(auth_response) => {
            Ok(to_value(&auth_response).map_err(|e| JsValue::from_str(&e.to_string()))?)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}


pub async fn list_posts(limit: i64, offset : i64) -> Result<Vec<Post>, JsValue> {
	let client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));

    let res = client
        .list_posts(limit, offset)
        .await;

    match res {
        Ok(posts) => {
            Ok(posts)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub async fn login_wasm(username: String, password: String) -> Result<JsValue, JsValue> {
    let mut client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));

    let res = client.login(username, password).await;

    match res {
        Ok(auth_response) => {
            Ok(to_value(&auth_response).map_err(|e| JsValue::from_str(&e.to_string()))?)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub async fn update_post_wasm(post_id: i64, title: String, content: String) -> Result<JsValue, JsValue> {
    let (token, _) = get_data().ok_or_else(|| JsValue::from_str("No auth data found"))?;
    let mut client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));
    client.set_token(token);

    let res = client.update_post(post_id, title, content).await;

    match res {
        Ok(updated_post) => {
            Ok(to_value(&updated_post).map_err(|e| JsValue::from_str(&e.to_string()))?)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub async fn create_post_wasm(title: String, content: String) -> Result<JsValue, JsValue> {
    let (token, _) = get_data().ok_or_else(|| JsValue::from_str("No auth data found"))?;
    let mut client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));
    client.set_token(token);

    let res = client.create_post(title, content).await;

    match res {
        Ok(new_post) => {
            Ok(to_value(&new_post).map_err(|e| JsValue::from_str(&e.to_string()))?)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
#[wasm_bindgen]
pub async fn delete_post_wasm(post_id: i64) -> Result<JsValue, JsValue> {
    let (token, _) = get_data().ok_or_else(|| JsValue::from_str("No auth data found"))?;
    let mut client = BlogClient::new(Transport::Http(SERVER_URL.to_string()));
    client.set_token(token);

    let res = client.delete_post(post_id).await;

    match res {
        Ok(_) => {
            Ok(JsValue::NULL)
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}


pub fn save_data(token: &str, user_id : i64) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(TOKEN_KEY, token);
            let _ = storage.set_item(USER_ID_KEY, &user_id.to_string());
        }
    }
}

pub fn get_data() -> Option<(String, String)> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let token = storage.get_item(TOKEN_KEY).ok()??;
    let user_id = storage.get_item(USER_ID_KEY).ok()??;
	Some((token, user_id))
}

pub fn clear_data() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(TOKEN_KEY);
            let _ = storage.remove_item(USER_ID_KEY);
        }
    }
}
