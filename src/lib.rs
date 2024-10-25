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
