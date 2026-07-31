pub mod app;
pub mod nuoruo;

pub use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_logger::init();
    log::info!("Nuoruo Web Framework initialized");

    let app = app::App::new();
    log::info!("App created, now mounting...");

    let framework_app = App::new("app");
    log::info!("Framework app created with root: app");

    framework_app.mount(&app);
    log::info!("Mount complete");
}

pub trait Component {
    fn render(&self) -> String;
    fn mount(&self, element: &web_sys::Element);
}

pub struct App {
    root: String,
}

impl App {
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }

    pub fn mount<C: Component>(&self, component: &C) {
        log::info!("Framework mount: Looking for element #{}", self.root);
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document
            .query_selector(&format!("#{}", self.root))
            .unwrap()
            .expect("Root element not found");
        log::info!("Framework mount: Found element, calling component.mount()");

        component.mount(&element);
        log::info!("Framework mount: component.mount() returned");
    }
}

pub fn html_to_element(html: &str) -> web_sys::Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let template: web_sys::HtmlTemplateElement = document
        .create_element("template")
        .unwrap()
        .dyn_into()
        .unwrap();
    template.set_inner_html(html);
    template
        .content()
        .first_child()
        .unwrap()
        .dyn_into()
        .unwrap()
}

pub mod web_logger {
    use log::{LevelFilter, Log, Metadata, Record};
    use web_sys::console;

    pub fn init() {
        log::set_logger(&ConsoleLogger).unwrap();
        log::set_max_level(LevelFilter::Info);
    }

    struct ConsoleLogger;

    impl Log for ConsoleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= LevelFilter::Info
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let msg = format!("[{}] {}", record.level(), record.args());
                console::log_1(&msg.into());
            }
        }

        fn flush(&self) {}
    }
}
