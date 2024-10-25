pub mod config;
pub mod consts;
pub mod physics;

use physics::PhysicsMesh;

use three_d::*;

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::spawn_local,
    web_sys::{window, File, FileReader, HtmlInputElement},
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn show_open_file_picker() -> JsValue;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let window = window().ok_or("No global window available")?;
    let document = window.document().ok_or("No document on window")?;
    let body = document.body().ok_or("No body in document")?;

    // Create the file input element
    let input = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    input.set_type("file");

    // Wrap input in Rc<RefCell> for shared mutability
    let input_rc = Rc::new(RefCell::new(input));

    // Closure to handle clicks on the body
    let closure = Closure::wrap(Box::new(move || {
        // Trigger the click on the input element
        input_rc.borrow().click();

        // Clone the Rc<RefCell<HtmlInputElement>> for the change event closure
        let input_clone = input_rc.clone();
        let change_closure = Closure::wrap(Box::new(move || {
            // Borrow the input to access files
            if let Some(files) = input_clone.borrow().files() {
                if let Some(file) = files.get(0) {
                    wasm_run_from_conf(file);
                }
            }
        }) as Box<dyn Fn()>);

        // Add the change event listener to the input
        input_rc
            .borrow()
            .add_event_listener_with_callback("change", change_closure.as_ref().unchecked_ref())
            .unwrap();
        change_closure.forget(); // Keep the closure alive
    }) as Box<dyn Fn()>);

    // Attach the event listener to the body
    body.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Keep the closure alive

    Ok(())
}

#[cfg(target_arch = "wasm32")]
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
                    run(config).await.expect("Error running simulation");
                });
            }
        }
    }) as Box<dyn Fn(web_sys::Event)>);

    // Set up the reader to call our closure when the read is complete
    reader.set_onload(Some(reader_closure.as_ref().unchecked_ref()));
    reader_closure.forget(); // Keep the closure alive

    // Read the file as text
    reader.read_as_text(&file).unwrap();
}

pub async fn run(config: config::Config) -> anyhow::Result<()> {
    let window = Window::new(WindowSettings {
        title: config.global.window_name,
        max_size: config.global.window_size,
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = config.camera.as_camera(window.viewport());
    let mut control = OrbitControl::new(*camera.target(), 1.0, 10000.0);

    let mut pmesh = PhysicsMesh::default();

    config
        .bodies
        .into_iter()
        .map(|body| body.as_gbody(&context).expect("error making body"))
        .for_each(|gbody| pmesh.add(gbody));

    let lights: Vec<_> = config
        .directional_lights
        .into_iter()
        .map(|light| light.as_dlight(&context))
        .collect();

    //let skybox = Skybox::new_from_equirectangular(&context, &CpuTexture::default());

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        pmesh.compute((frame_input.elapsed_time * config.cheats.time_mult) as f32);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                pmesh.render().into_iter(),
                lights
                    .iter()
                    .map(|dlight| dlight as &dyn Light)
                    .collect::<Vec<_>>()
                    .as_slice(),
            );

        FrameOutput::default()
    });

    Ok(())
}
