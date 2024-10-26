#![feature(trait_upcasting, specialization)]

pub mod config;
pub mod consts;
pub mod physics;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

use physics::PhysicsMesh;

use three_d::*;

pub async fn run(config: config::Config) -> anyhow::Result<()> {
    let window = Window::new(WindowSettings {
        title: config.global.window_name,
        max_size: config.global.window_size,
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = config.camera.as_camera(window.viewport());
    let mut orbit_control = OrbitControl::new(*camera.target(), 1.0, 10000.0);
    let mut fly_control = FlyControl::new(0.01);

    let mut pmesh = PhysicsMesh::default();

    config
        .bodies
        .into_iter()
        .map(|body| body.as_gbody(&context).expect("error making body"))
        .for_each(|gbody| pmesh.add(gbody));

    let mut lights = config.lights.as_scene_lighting(&context); // .as_dyn_lights(&context);

    //let skybox = Skybox::new_from_equirectangular(&context, &CpuTexture::default());

    let clear_color_state = srgba_as_clearstate(config.global.background_color, 255);
    window.render_loop(move |mut frame_input| {
        if frame_input.elapsed_time > config.global.max_frame_dt {
            println!("dropping frame ({}ms)", frame_input.elapsed_time);
            return FrameOutput::default();
        }
        camera.set_viewport(frame_input.viewport);
        orbit_control.handle_events(&mut camera, &mut frame_input.events);
        fly_control.handle_events(&mut camera, &mut frame_input.events);

        pmesh.compute((frame_input.elapsed_time * config.cheats.time_mult) as f32);

        // so bcs we compute new body positions (`.render()`) after this
        // the shadow compute will be a frame outdated, unless we call
        // `.render()` twice...
        pmesh.render();
        let light_render = lights.render(
            4096,
            pmesh.get_mesh().as_slice(),
        );
        frame_input.screen().clear(clear_color_state).render(
            &camera,
            pmesh.render().as_slice().into_iter(),
            light_render.as_slice(),
        );

        FrameOutput::default()
    });

    Ok(())
}

fn srgba_as_clearstate(srgba: Srgba, depth: u8) -> ClearState {
    ClearState::color_and_depth(
        srgba.r as f32 / 255.0,
        srgba.g as f32 / 255.0,
        srgba.b as f32 / 255.0,
        srgba.a as f32 / 255.0,
        depth as f32 / 255.0,
    )
}
