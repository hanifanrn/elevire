use leptos::{
    ev::KeyboardEvent,
    html::InnerHtmlAttribute,
    prelude::{
        component, event_target_value, view, AriaAttributes, ClassAttribute, ElementChild, Get,
        IntoView, OnAttribute, ReadSignal, RwSignal, Set, Update,
    },
    task::spawn_local,
};
use pulldown_cmark::{html, Options, Parser};
use reqwasm::http::Request;

use crate::components::chat::model::{ChatRequest, ChatResponse};

pub async fn send_chat(prompt: String) -> Option<String> {
    let request_data = ChatRequest {
        prompt: prompt.clone(),
    };

    let res = Request::post("http://localhost:8081/chat")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_data).unwrap())
        .send()
        .await;

    if let Ok(response) = res {
        if let Ok(chat_response) = response.json::<ChatResponse>().await {
            return Some(chat_response.response);
        }
    }

    None
}

#[component]
pub fn ChatArea() -> impl IntoView {
    let chat_history = RwSignal::new(vec![]);

    view! {
        <div class="chat-area">
            <div class="chat">
                <div class="chat-history">
                    <ChatHistory history=chat_history.read_only() />
                </div>
                <ChatInput history=chat_history />
            </div>
        </div>
    }
}

#[component]
fn ChatHistory(history: ReadSignal<Vec<(String, String)>>) -> impl IntoView {
    view! {
        <ul>
            {move || history.get().iter().map(|(user, bot)| view! {
                <li class="message-row user">
                    <div class="chat-bubble user">{user.clone()}</div>
                </li>
                <li class="message-row bot">
                    <div class="chat-bubble bot">
                        {
                            let bot_clone = bot.clone(); // Clone the bot string to extend its lifetime
                            let parser = Parser::new_ext(&bot_clone, Options::all()); // Use the cloned string
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);
                            view! { <div inner_html=html_output /> }
                        }
                    </div>
                </li>
            }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn ChatInput(history: RwSignal<Vec<(String, String)>>) -> impl IntoView {
    let message = RwSignal::new("".to_string());

    let send = {
        move || {
            let msg = message.get();
            if msg.is_empty() {
                return;
            }

            message.set("".to_string());

            let user_msg = msg.clone();
            spawn_local(async move {
                if let Some(bot_msg) = send_chat(msg).await {
                    history.update(|h| {
                        h.push((user_msg, bot_msg));
                    });
                }
            });
        }
    };

    let handle_enter = {
        move |ev: KeyboardEvent| {
            if ev.key() == "Enter" {
                send();
            }
        }
    };

    view! {
        <div class="chat-input">
            <input
                class="chat-input-field"
                type="text"
                placeholder="Type your message..."
                aria-label="Message input"
                value=move || message.get()
                on:input=move |e| message.set(event_target_value(&e))
                on:keydown=handle_enter
            />
            <button
                class="chat-send-button"
                aria-label="Send message"
                on:click=move |_| send()
            >
                "Send"
            </button>
        </div>
    }
}
