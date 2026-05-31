use crate::Component;
use wasm_bindgen::prelude::*;
use web_sys::{console, Element};

#[wasm_bindgen]
pub struct App {
    title: String,
    message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Welcome to Nuoruo Web".to_string(),
            message: "A lightweight frontend framework".to_string(),
        }
    }
}

impl Component for App {
    fn render(&self) -> String {
        console::log_1(&"App.render() called".into());
        format!(
            r#"
            <main class="container">
                <h1>{}</h1>
                <p>{}</p>
                <button id="demo-btn" class="btn btn-primary">Click Me</button>
                <div id="counter">Counter: <span id="count">0</span></div>
            </main>
            "#,
            self.title, self.message
        )
    }

    fn mount(&self, element: &Element) {
        console::log_1(&"App.mount() START".into());

        console::log_1(&"App.mount() - calling set_inner_html".into());
        let html = self.render();
        console::log_1(&format!("App.mount() - HTML length: {}", html.len()).into());

        element.set_inner_html(&html);
        console::log_1(&"App.mount() - set_inner_html done".into());

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        console::log_1(&"App.mount() - querying button".into());
        let btn_result = document.query_selector("#demo-btn");
        match &btn_result {
            Ok(Some(btn)) => {
                console::log_1(&"App.mount() - button found!".into());

                let document_for_closure = document.clone();
                let closure = Closure::wrap(Box::new(move || {
                    console::log_1(&"App.click() - CLICK EVENT FIRED!".into());
                    if let Ok(Some(count_span)) = document_for_closure.query_selector("#count") {
                        let current = count_span.inner_html().parse::<i32>().unwrap_or(0);
                        console::log_1(&format!("App.click() - current: {}", current).into());
                        count_span.set_inner_html(&(current + 1).to_string());
                    } else {
                        console::log_1(&"App.click() - count span not found".into());
                    }
                }) as Box<dyn Fn()>);

                match btn
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                {
                    Ok(_) => console::log_1(&"App.mount() - listener added successfully".into()),
                    Err(e) => {
                        console::log_1(&format!("App.mount() - listener error: {:?}", e).into())
                    }
                }
                closure.forget();
            }
            Ok(None) => console::log_1(&"App.mount() ERROR - button not found".into()),
            Err(e) => console::log_1(&format!("App.mount() ERROR - query failed: {:?}", e).into()),
        }
        console::log_1(&"App.mount() COMPLETE".into());
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
