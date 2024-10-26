use crate::config;

use std::{cell::RefCell, rc::Rc};

use {
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::spawn_local,
    web_sys::{window, File, FileReader, HtmlInputElement},
};

#[wasm_bindgen]
extern "C" {
    fn show_open_file_picker() -> JsValue;
}

// Mostly chatGPT code tbh, docs and logic sucks
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let window = window().ok_or("No global window available")?;
    let document = window.document().ok_or("No document on window")?;

    let input = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.set_type("file");

    let input_rc = Rc::new(RefCell::new(input));

    let open_computer_button = document.get_element_by_id("open-computer").unwrap();
    let open_computer_closure = Closure::wrap(Box::new(move || {
        let input_clone = input_rc.clone();
        let change_closure = Closure::wrap(Box::new(move || {
            if let Some(files) = input_clone.borrow().files() {
                if let Some(file) = files.get(0) {
                    wasm_run_from_conf(file);
                }
            }
        }) as Box<dyn Fn()>);

        input_rc.borrow().click();
        input_rc
            .borrow()
            .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref())
            .unwrap();
        change_closure.forget();
    }) as Box<dyn Fn()>);

    open_computer_button.add_event_listener_with_callback(
        "click",
        open_computer_closure.as_ref().unchecked_ref(),
    )?;
    open_computer_closure.forget();

    Ok(())
}

pub fn wasm_run_from_conf(file: File) {
    web_sys::console::log_1(&file.name().into());
    web_sys::console::log_1(&format!("{:?}", file).into());

    let reader = FileReader::new().unwrap();

    // Set up a closure to handle the file read result
    let reader_closure = Closure::wrap(Box::new(move |evt: web_sys::Event| {
        if let Some(target) = evt.target() {
            let reader = target.dyn_into::<FileReader>().unwrap();
            if let Some(result) = reader.result().unwrap().as_string() {
                // Log the file contents
                let config: config::Config = toml::from_str(&result).expect("invalid config");
                spawn_local(async {
                    crate::run(config).await.expect("Error running simulation");
                });
            }
        }
    }) as Box<dyn Fn(web_sys::Event)>);

    reader.set_onload(Some(reader_closure.as_ref().unchecked_ref()));
    reader_closure.forget();

    reader.read_as_text(&file).unwrap();
}
