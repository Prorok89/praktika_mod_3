mod lib;
use blog_client::{AuthResponse, Post};
// use blog_wasm::register;
use web_sys::{console, window};
use yew::{html::ChildrenProps, platform::spawn_local, prelude::*};

use crate::lib::{
    AuthContext, MessageKind, Notification, NotificationContext, clear_data, list_posts, login_wasm, register_wasm, save_data
};

const TOKEN_KEY: &str = "blog_token";
const USER_ID_KEY: &str = "blog_user_id";

#[function_component(AuthProvider)]
pub fn auth_provider(props: &ChildrenProps) -> Html {
    let is_logged_in = use_state(|| false);
	let username = use_state(|| "".to_string());
 	let user_id = use_state(|| None::<i64>);

    {
        let is_logged_in = is_logged_in.clone();
		let user_id = user_id.clone();
        use_effect_with((), move |_| {
            if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                if let Ok(Some(_token)) = storage.get_item(TOKEN_KEY) {
                    is_logged_in.set(true);
                }
				                
                if let Ok(Some(id_str)) = storage.get_item(USER_ID_KEY) {
                    if let Ok(id) = id_str.parse::<i64>() {
                        user_id.set(Some(id));
                    }
                }
            }
            || ()
        });
    }

    let logout = {
        let is_logged_in = is_logged_in.clone();
        Callback::from(move |_| {
            let is_logged_in = is_logged_in.clone();
			clear_data();
            is_logged_in.set(false);
        })
    };

    let login = {
        let is_logged_in = is_logged_in.clone();
		let username = username.clone();
        Callback::from(move |a: AuthResponse| {
            let is_logged_in = is_logged_in.clone();
			let e = a.user.email.clone();
			let u = a.user.username.clone();
			let id = a.user.id; 
			let f = format!("{} - {}", e, u);
			save_data(&a.token, id.unwrap());
			username.set(f);
            is_logged_in.set(true);
        })
    };
	
    let context = AuthContext {
		user_id: *user_id,
		username : username.to_string(),
        is_logged_in: *is_logged_in,
        logout,
        login,
    };

    html! {

        <ContextProvider<AuthContext> context={context}>
			if *is_logged_in {
				{
					(*username).to_string()
				}
			}
            { props.children.clone() }
        </ContextProvider<AuthContext>>
    }
}

#[function_component(AuthButton)]
pub fn auth_button() -> Html {
    
    let auth =
        use_context::<AuthContext>().expect("AuthContext не найден! Оберните App в AuthProvider");

    let is_register_visible = use_state(|| false);
    let is_login_visible = use_state(|| false);

    let on_register = {
        let is_register_visible = is_register_visible.clone();
        let is_login_visible = is_login_visible.clone();

        Callback::from(move |_| {
            is_register_visible.set(true);
            is_login_visible.set(false);
        })
    };

    let on_login = {
        let is_register_visible = is_register_visible.clone();
        let is_login_visible = is_login_visible.clone();

        Callback::from(move |_| {
            is_register_visible.set(false);
            is_login_visible.set(true);
 			
            console::log_1(&"Нажата кнопка: Войти (форма скрыта)".into());
        })
    };

    let on_logout = {
        let is_register_visible = is_register_visible.clone();
        let is_login_visible = is_login_visible.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            is_register_visible.set(false);
            is_login_visible.set(false);
            auth.logout.emit(MouseEvent::new("click").unwrap());
        })
    };

    html! {
        <>
            if auth.is_logged_in {
                <button
                    onclick={on_logout}
                    style="padding: 10px 20px; cursor: pointer; background-color: #f44336; color: white; border: none; border-radius: 4px;"
                >
                    { "Выйти" }
                </button>
            }
            else
            {
                <button
                    onclick={on_register}
                    style="padding: 10px 20px; cursor: pointer; background-color: #4CAF50; color: white; border: none; border-radius: 4px;"
                >
                    { "Регистрация" }
                </button>

                <button
                    onclick={on_login}
                    style="padding: 10px 20px; cursor: pointer; background-color: #008CBA; color: white; border: none; border-radius: 4px;"
                >
                    { "Войти" }
                </button>
            }

            if *is_register_visible {
                <RegisterForm on_cancel={
                    let is_register_visible = is_register_visible.clone();
                    Callback::from(move |_| is_register_visible.set(false))
                } />
            }

            if *is_login_visible {
                <LoginForm on_cancel={
                    let is_login_visible = is_login_visible.clone();
                    Callback::from(move |_| is_login_visible.set(false))
                } />
            }
        </>
    }
}

#[function_component(RegisterForm)]
fn register_form(props: &RegisterFormProps) -> Html {
    let notification_ctx = use_context::<NotificationContext>()
        .expect("NotificationContext не найден! Оберните App в NotificationProvider");

    // Состояния для полей формы
    let username = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let on_cancel: Callback<MouseEvent> = props.on_cancel.clone();
        let ctx: NotificationContext = notification_ctx.clone();

        Callback::from(move |_| {
            if username.is_empty() || email.is_empty() || password.is_empty() {
                ctx.notify.emit(Notification {
                    text: format!("Все поля должны быть заполнены"),
                    kind: MessageKind::Error,
                });
            } else {
                let username = username.clone();
                let email = email.clone();
                let password = password.clone();
				let on_cancel = on_cancel.clone();
				let ctx = ctx.clone();
                spawn_local(async move {
                    let res = register_wasm(&username, &email, &password).await;

                    match res {
						 Ok(js_val) => {
                            if let Ok(_auth_res) =
                                serde_wasm_bindgen::from_value::<AuthResponse>(js_val)
                            {
                                ctx.notify.emit(Notification {
                                    text: "Вы успешно зерегестрировались!".to_string(),
                                    kind: MessageKind::Success,
                                });

                                on_cancel.emit(MouseEvent::new("click").unwrap());   
                            }
                        }
                        Err(e) => {
                            ctx.notify.emit(Notification {
                                text: format!("Ошибка: {:?}", e),
                                kind: MessageKind::Error,
                            });
                        }
                    }
                })
            }
        })
    };

    html! {
        <div style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
                    background: white; padding: 30px; border: 1px solid #ccc; 
                    border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); z-index: 1000;
                    display: flex; flex-direction: column; gap: 15px; min-width: 300px;">

            <h2>{"Регистрация"}</h2>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Username:"}</label>
                <input type="text" oninput={on_username_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Email:"}</label>
                <input type="email" oninput={on_email_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Password:"}</label>
                <input type="password" oninput={on_password_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 10px;">
                <button onclick={props.on_cancel.clone()} style="padding: 8px 15px; cursor: pointer;">
                    {"Отмена"}
                </button>
                <button onclick={on_submit} style="padding: 8px 15px; cursor: pointer; background-color: #4CAF50; color: white; border: none; border-radius: 4px;">
                    {"Создать аккаунт"}
                </button>
            </div>
        </div>
    }
}

#[function_component(LoginForm)]
fn login_form(props: &RegisterFormProps) -> Html {
    let notification_ctx = use_context::<NotificationContext>()
        .expect("NotificationContext не найден! Оберните App в NotificationProvider");
    let auth =
        use_context::<AuthContext>().expect("AuthContext не найден! Оберните App в AuthProvider");
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username_clone = username.clone();
        let password_clone = password.clone();
        let on_cancel_clone = props.on_cancel.clone();
        let ctx_clone: NotificationContext = notification_ctx.clone();
        let auth = auth.clone();

        Callback::from(move |_| {
            if username_clone.is_empty() || password_clone.is_empty() {
                ctx_clone.notify.emit(Notification {
                    text: format!("Все поля должны быть заполнены"),
                    kind: MessageKind::Error,
                });
            } else {
                let username_clone1 = username_clone.clone();
                let password_clone1 = password_clone.clone();
                let on_cancel_clone1 = on_cancel_clone.clone();
                let notification_ctx = notification_ctx.clone();
                let auth = auth.clone();
                spawn_local(async move {
                    let data_login =
                        login_wasm(username_clone1.to_string(), password_clone1.to_string()).await;

                    match data_login {
                        Ok(js_val) => {
                            if let Ok(auth_res) =
                                serde_wasm_bindgen::from_value::<AuthResponse>(js_val)
                            {
                                notification_ctx.notify.emit(Notification {
                                    text: "Авторизация успешна!".to_string(),
                                    kind: MessageKind::Success,
                                });
							
                                on_cancel_clone1.emit(MouseEvent::new("click").unwrap());
                                auth.login.emit(auth_res);
                            }
                        }
                        Err(e) => {
                            notification_ctx.notify.emit(Notification {
                                text: format!("Ошибка: {:?}", e),
                                kind: MessageKind::Error,
                            });
                        }
                    }
                })
            }
        })
    };

    html! {
        <div style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
                    background: white; padding: 30px; border: 1px solid #ccc; 
                    border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); z-index: 1000;
                    display: flex; flex-direction: column; gap: 15px; min-width: 300px;">

            <h2>{"Авторизация"}</h2>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Username:"}</label>
                <input type="text" oninput={on_username_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Password:"}</label>
                <input type="password" oninput={on_password_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 10px;">
                <button onclick={props.on_cancel.clone()} style="padding: 8px 15px; cursor: pointer;">
                    {"Отмена"}
                </button>
                <button onclick={on_submit} style="padding: 8px 15px; cursor: pointer; background-color: #4CAF50; color: white; border: none; border-radius: 4px;">
                    {"Войти"}
                </button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RegisterFormProps {
    on_cancel: Callback<MouseEvent>,
}

#[function_component(NotificationProvider)]
pub fn notification_provider(props: &ChildrenProps) -> Html {
    // Состояние самого уведомления
    let notification = use_state(|| None::<Notification>);

    // Функция, которую мы будем давать другим компонентам
    let notify = {
        let notification = notification.clone();
        Callback::from(move |n: Notification| {
            notification.set(Some(n));
        })
    };

    let context = NotificationContext { notify };

    html! {
        <ContextProvider<NotificationContext> context={context}>
            <div class="notification-container">
                { if let Some(n) = &*notification {
                    let class = match n.kind {
                        MessageKind::Success => "notif-success",
                        MessageKind::Error => "notif-error",
                    };
                 html! {
                    <div class={class}>
                        <span class="notif-icon">
                            { if n.kind == MessageKind::Success { "✅" } else { "❌" } }
                        </span>
                        <span class="notif-text">{ &n.text }</span>
                        <button class="notif-close" onclick={let notification = notification.clone(); move |_| notification.set(None)}>
                            { "×" }
                        </button>
                    </div>
                }
                } else {
                    html! {}
                }}
            </div>

            { props.children.clone() }
        </ContextProvider<NotificationContext>>
    }
}

#[function_component(PostList)]
pub fn post_list() -> Html {
    let posts = use_state(|| Vec::<Post>::new());
    let is_loading = use_state(|| true);
    let error_msg = use_state(|| None::<String>);
    let editing_post_id = use_state(|| None::<i64>);
    let is_create_mode_open = use_state(|| false);
    let auth = use_context::<AuthContext>().expect("AuthContext не найден! Оберните App в NotificationProvider");

    let fetch_posts = {
        let posts = posts.clone();
        let is_loading = is_loading.clone();
        let error_msg = error_msg.clone();
        Callback::from(move |_| {
            let posts = posts.clone();
            let is_loading = is_loading.clone();
            let error_msg = error_msg.clone();
            spawn_local(async move {
                is_loading.set(true);
                match list_posts(10, 0).await {
                    Ok(p) => {
                        posts.set(p);
                        is_loading.set(false);
                    }
                    Err(e) => {
                        error_msg.set(Some(e.as_string().unwrap_or_else(|| "Unknown error".into())));
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    {
        let fetch_posts = fetch_posts.clone();
        use_effect_with((), move |_| {
            fetch_posts.emit(());
            || ()
        });
    }

    let start_editing = {
        let editing_post_id = editing_post_id.clone();
        Callback::from(move |id: i64| {
            editing_post_id.set(Some(id));
        })
    };

    let cancel_editing = {
        let editing_post_id = editing_post_id.clone();
        Callback::from(move |_| {
            editing_post_id.set(None);
        })
    };

    let toggle_create_mode = {
        let is_create_mode_open = is_create_mode_open.clone();
        Callback::from(move |_| {
            is_create_mode_open.set(!*is_create_mode_open);
        })
    };

    // Логика удаления
    let delete_post_callback = {
        let fetch_posts = fetch_posts.clone();
        let notification_ctx = use_context::<NotificationContext>().expect("NotificationContext не найден!");
        Callback::from(move |id: i64| {
            let fetch_posts = fetch_posts.clone();
            let notification_ctx = notification_ctx.clone();
            spawn_local(async move {
                let res = crate::lib::delete_post_wasm(id).await;
                match res {
                    Ok(_) => {
                        notification_ctx.notify.emit(Notification {
                            text: "Пост успешно удален".to_string(),
                            kind: MessageKind::Success,
                        });
                        fetch_posts.emit(()); // Обновляем список
                    }
                    Err(e) => {
                        notification_ctx.notify.emit(Notification {
                            text: format!("Ошибка удаления: {:?}", e),
                            kind: MessageKind::Error,
                        });
                    }
                }
            });
        })
    };

    html! {
        <div class="posts-container">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <h2 style="margin: 0;">{"Список постов"}</h2>
                
                { if auth.is_logged_in {
                    html! {
                        <button 
                            onclick={toggle_create_mode.clone()}
                            style="padding: 10px 15px; cursor: pointer; background-color: #2196F3; color: white; border: none; border-radius: 4px;"
                        >
                            { "+ Новый пост" }
                        </button>
                    }
                } else {
                    html! {}
                }}
            </div>

            { if *is_loading {
                html! { <p style="text-align: center;">{"Загрузка..."}</p> }
            } else if let Some(err) = &*error_msg {
                html! { <p class="error" style="text-align: center; color: red;">{ err }</p> }
            } else {
                html! {}
            } }

            <div class="posts-grid" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 20px; padding: 20px;">
                { for posts.iter().map(|post| {
                    let post_id = post.id.unwrap_or(0);
                    let is_author = auth.user_id == Some(post.author_id);
                    let on_edit = start_editing.clone();
                    let on_delete = delete_post_callback.clone();

                    html! {
                        <div class="post-card" key={post_id} style="border: 1px solid #ddd; padding: 15px; border-radius: 8px; background: #f9f9f9; position: relative; display: flex; flex-direction: column; justify-content: space-between;">
                            <div>
                                <h3 style="margin-top: 0;">{ &post.title }</h3>
                                <p style="white-space: pre-wrap;">{ &post.content }</p>
                            </div>
                            <div class="post-footer" style="display: flex; justify-content: space-between; align-items: center; font-size: 0.8em; color: #666; margin-top: 15px;">
                                <span>{"Автор ID: "} {post.author_id}</span>
                                { if is_author {
                                    html! {
                                        <div style="display: flex; gap: 5px;">
                                            <button
                                                onclick={move |_| on_edit.emit(post_id)}
                                                style="padding: 5px 10px; cursor: pointer; background-color: #FF9800; color: white; border: none; border-radius: 4px;"
                                            >
                                                {"Edit"}
                                            </button>
                                            <button
                                                onclick={move |_| on_delete.emit(post_id)}
                                                style="padding: 5px 10px; cursor: pointer; background-color: #f44336; color: white; border: none; border-radius: 4px;"
                                            >
                                                {"Del"}
                                            </button>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>
            
            { if posts.is_empty() && !*is_loading {
                html! { <p style="text-align: center;">{"Постов пока нет."}</p> }
            } else { html! {} } }

            // Рендеринг формы редактирования
            { if let Some(id) = *editing_post_id {
                let post_to_edit = posts.iter().find(|p| p.id == Some(id)).cloned();
                if let Some(post) = post_to_edit {
                    html! {
                        <EditPostForm
                            post_id={post.id.unwrap_or(0)}
                            initial_title={post.title.clone()}
                            initial_content={post.content.clone()}
                            on_cancel={cancel_editing.clone()}
                            on_success={
                                let f = fetch_posts.clone();
                                Callback::from(move |_| f.emit(()))
                            }
                        />
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }}

           
            { if *is_create_mode_open {
                html! {
                    <CreatePostForm 
                        on_cancel={
                            let is_create_mode_open = is_create_mode_open.clone();
                            Callback::from(move |_| is_create_mode_open.set(false))
                        }
                        on_success={
                            let f = fetch_posts.clone();
                            let is_create_mode_open = is_create_mode_open.clone();
                            Callback::from(move |_| {
                                f.emit(());
                                is_create_mode_open.set(false);
                            })
                        }
                    />
                }
            } else {
                html! {}
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct EditPostFormProps {
    post_id: i64,
    initial_title: String,
    initial_content: String,
    on_cancel: Callback<MouseEvent>,
    on_success: Callback<()>,
}

#[function_component(EditPostForm)]
fn edit_post_form(props: &EditPostFormProps) -> Html {
    let notification_ctx = use_context::<NotificationContext>()
        .expect("NotificationContext не найден! Оберните App в NotificationProvider");
    let _auth = use_context::<AuthContext>().expect("AuthContext не найден!");

    let title = use_state(|| props.initial_title.clone());
    let content = use_state(|| props.initial_content.clone());

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let content = content.clone();
        let post_id = props.post_id;
        let on_cancel = props.on_cancel.clone();
        let on_success = props.on_success.clone();
        let ctx = notification_ctx.clone();

        Callback::from(move |_| {
            let title = title.clone();
            let content = content.clone();
            let post_id = post_id;
            let on_cancel = on_cancel.clone();
            let on_success = on_success.clone();
            let ctx = ctx.clone();

            spawn_local(async move {
                if title.is_empty() || content.is_empty() {
                    ctx.notify.emit(Notification {
                        text: "Поля не могут быть пустыми".to_string(),
                        kind: MessageKind::Error,
                    });
                    return;
                }

                let res = crate::lib::update_post_wasm(post_id, (*title).clone(), (*content).clone()).await;

                match res {
                    Ok(_) => {
                        ctx.notify.emit(Notification {
                            text: "Пост успешно обновлен!".to_string(),
                            kind: MessageKind::Success,
                        });
                        on_success.emit(());
                        on_cancel.emit(MouseEvent::new("click").unwrap());
                    }
                    Err(e) => {
                        ctx.notify.emit(Notification {
                            text: format!("Ошибка: {:?}", e),
                            kind: MessageKind::Error,
                        });
                    }
                }
            });
        })
    };

    html! {
        <div style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
                    background: white; padding: 30px; border: 1px solid #ccc; 
                    border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); z-index: 1000;
                    display: flex; flex-direction: column; gap: 15px; min-width: 350px;">

            <h2>{"Редактирование поста"}</h2>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Заголовок:"}</label>
                <input type="text" value={title.to_string()} oninput={on_title_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Текст поста:"}</label>
              <textarea 
					oninput={on_content_input} 
					value={content.to_string()} 
					style="padding: 8px; min-height: 100px; border: 1px solid #ccc; border-radius: 4px;" 
				/>
            </div>

            <div style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 10px;">
                <button onclick={props.on_cancel.clone()} style="padding: 8px 15px; cursor: pointer;">
                    {"Отмена"}
                </button>
                <button onclick={on_submit} style="padding: 8px 15px; cursor: pointer; background-color: #4CAF50; color: white; border: none; border-radius: 4px;">
                    {"Сохранить"}
                </button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CreatePostFormProps {
    on_cancel: Callback<MouseEvent>,
    on_success: Callback<()>,
}

#[function_component(CreatePostForm)]
fn create_post_form(props: &CreatePostFormProps) -> Html {
    let notification_ctx = use_context::<NotificationContext>()
        .expect("NotificationContext не найден! Оберните App в NotificationProvider");

    let title = use_state(|| "".to_string());
    let content = use_state(|| "".to_string());

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let content = content.clone();
        let on_cancel = props.on_cancel.clone();
        let on_success = props.on_success.clone();
        let ctx = notification_ctx.clone();

        Callback::from(move |_| {
            let title = title.clone();
            let content = content.clone();
            let on_cancel = on_cancel.clone();
            let on_success = on_success.clone();
            let ctx = ctx.clone();

            spawn_local(async move {
                if title.is_empty() || content.is_empty() {
                    ctx.notify.emit(Notification {
                        text: "Заголовок и текст не могут быть пустыми".to_string(),
                        kind: MessageKind::Error,
                    });
                    return;
                }

                let res = crate::lib::create_post_wasm((*title).clone(), (*content).clone()).await;

                match res {
                    Ok(_) => {
                        ctx.notify.emit(Notification {
                            text: "Пост успешно создан!".to_string(),
                            kind: MessageKind::Success,
                        });
                        on_success.emit(());
                        on_cancel.emit(MouseEvent::new("click").unwrap());
                    }
                    Err(e) => {
                        ctx.notify.emit(Notification {
                            text: format!("Ошибка создания: {:?}", e),
                            kind: MessageKind::Error,
                        });
                    }
                }
            });
        })
    };

    html! {
        <div style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
                    background: white; padding: 30px; border: 1px solid #ccc; 
                    border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); z-index: 1000;
                    display: flex; flex-direction: column; gap: 15px; min-width: 350px;">

            <h2>{"Создать новый пост"}</h2>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Заголовок:"}</label>
                <input type="text" value={title.to_string()} oninput={on_title_input} style="padding: 8px;" />
            </div>

            <div style="display: flex; flex-direction: column; text-align: left;">
                <label>{"Текст поста:"}</label>
                <textarea oninput={on_content_input} value={content.to_string()} style="padding: 8px; min-height: 100px; border: 1px solid #ccc; border-radius: 4px;" />
            </div>

            <div style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 10px;">
                <button onclick={props.on_cancel.clone()} style="padding: 8px 15px; cursor: pointer;">
                    {"Отмена"}
                </button>
                <button onclick={on_submit} style="padding: 8px 15px; cursor: pointer; background-color: #4CAF50; color: white; border: none; border-radius: 4px;">
                    {"Опубликовать"}
                </button>
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <NotificationProvider>
        <AuthProvider>
            <main style="text-align: center; font-family: sans-serif; padding-top: 50px;">
                  <AuthButton />
                  <PostList />
            </main>
        </AuthProvider>
        </NotificationProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
