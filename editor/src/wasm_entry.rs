use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::LazyLock;

use ratatui_core::terminal::Terminal;
use wasm_bindgen::prelude::*;

use crate::app::App;
use crate::key::{convert_terminal_key, Key};
use crate::storage::remote::RemoteStorage;
use crate::ui;
use crate::wasm_backend::WasmBackend;

static EVENT_QUEUE: LazyLock<std::sync::Mutex<VecDeque<Key>>> =
    LazyLock::new(|| std::sync::Mutex::new(VecDeque::new()));

thread_local! {
    static APP_STATE: RefCell<Option<AppState>> = const { RefCell::new(None) };
}

struct AppState {
    app: App,
    storage: RemoteStorage,
    terminal: Terminal<WasmBackend>,
}

#[wasm_bindgen]
pub fn init(base_url: &str, cols: u16, rows: u16) {
    console_error_panic_hook::set_once();

    let backend = WasmBackend::new(cols, rows);
    let terminal = Terminal::new(backend).expect("failed to create terminal");

    let app = App::new();
    let storage = RemoteStorage::new(base_url);

    APP_STATE.with(|state| {
        *state.borrow_mut() = Some(AppState {
            app,
            storage,
            terminal,
        });
    });
}

#[wasm_bindgen]
pub fn is_alive() -> bool {
    APP_STATE.with(|state| state.borrow().is_some())
}

#[wasm_bindgen]
pub fn push_key(key: &str) {
    if let Some(k) = convert_terminal_key(key) {
        EVENT_QUEUE.lock().unwrap().push_back(k);
    }
}

#[wasm_bindgen]
pub fn push_ctrl_s() {
    EVENT_QUEUE.lock().unwrap().push_back(Key::CtrlS);
}

#[wasm_bindgen]
pub fn push_backtab() {
    EVENT_QUEUE.lock().unwrap().push_back(Key::BackTab);
}

#[wasm_bindgen]
pub fn tick() -> Option<String> {
    APP_STATE.with(|state| {
        let mut guard = state.borrow_mut();
        let app_state = guard.as_mut()?;

        let events: Vec<Key> = EVENT_QUEUE.lock().unwrap().drain(..).collect();
        for key in events {
            app_state.app.handle_key(key, &mut app_state.storage);
        }

        let _ = app_state.terminal.draw(|frame| {
            ui::render(frame, &app_state.app);
        });

        Some(app_state.terminal.backend_mut().take_output())
    })
}

#[wasm_bindgen]
pub fn load_entries(data: &str) {
    APP_STATE.with(|state| {
        if let Some(ref mut app_state) = *state.borrow_mut() {
            app_state.app.load_entries(data);
        }
    });
}

#[wasm_bindgen]
pub fn resize(cols: u16, rows: u16) {
    APP_STATE.with(|state| {
        if let Some(ref mut app_state) = *state.borrow_mut() {
            app_state.terminal.backend_mut().resize(cols, rows);
        }
    });
}
