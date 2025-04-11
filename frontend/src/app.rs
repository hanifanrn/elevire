use crate::components::{
    chat::chat_component::ChatArea, footer::footer_component::FooterArea,
    header::header_component::HeaderArea, thread::thread_component::ThreadArea,
};
use leptos::prelude::{view, ClassAttribute, ElementChild, IntoView};

pub fn app() -> impl IntoView {
    view! {
        <main>
            <div class="site-frame">
                <HeaderArea />
                <div class="chat-thread">
                    <ThreadArea />
                    <ChatArea />
                </div>
            </div>
            <FooterArea />
        </main>
    }
}
