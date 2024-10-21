use three_d::*;

struct GBody {
    pub name: &'static str,
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub color: u32,
    pub radius: f32,
    pub mass: f32,
    pub gm_sphere: Gm<Mesh, PhysicalMaterial>,
}

impl GBody {
    pub fn new(
        ctx: &Context,
        name: &'static str,
        color: u32,
        radius: f32,
        mass: f32,
    ) -> Result<Self, three_d_asset::Error> {
        let [r, g, b, a] = unsafe { std::mem::transmute(color.to_be()) };

        let mut sphere_mesh = CpuMesh::sphere(16);
        sphere_mesh.transform(&Mat4::from_scale(radius))?;
        let gm_sphere = Gm::new(
            Mesh::new(&ctx, &sphere_mesh),
            PhysicalMaterial::new_transparent(
                &ctx,
                &CpuMaterial {
                    albedo: Srgba { r, g, b, a },
                    ..Default::default()
                },
            ),
        );
        Ok(Self {
            name,
            pos: Vector3::zero(),
            vel: Vector3::zero(),
            color,
            radius,
            mass,
            gm_sphere,
        })
    }

    pub fn set_motion(&mut self, pos: Vector3<f32>, vel: Vector3<f32>) {
        self.pos = pos;
        self.vel = vel;
    }

    pub fn displace(&mut self, dr: Vector3<f32>) {
        self.pos += dr;
    }

    pub fn accelerate(&mut self, dv: Vector3<f32>) {
        self.vel += dv;
    }

    pub fn accelerate_to(&mut self, dv: f32, to: Vector3<f32>) {
        let vec_u = (to - self.pos).normalize();
        self.accelerate(vec_u * dv);
    }

    pub fn process(&mut self, dt: f32) {
        self.displace(self.vel * dt);
    }

    pub fn render(&mut self) {
        self.gm_sphere
            .set_transformation(Mat4::from_translation(self.pos));
    }
}

struct PhysicsMesh {
    pub const_g: f32,
    pub components: Vec<GBody>,
}

impl Default for PhysicsMesh {
    fn default() -> Self {
        Self {
            const_g: 6.67e-11, // mm³/(kg·ms²)
            components: vec![],
        }
    }
}
impl IntoIterator for PhysicsMesh {
    type Item = Gm<Mesh, PhysicalMaterial>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.components
            .into_iter()
            .map(|gbody| gbody.gm_sphere)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl PhysicsMesh {
    pub fn add(&mut self, component: GBody) {
        self.components.push(component);
    }

    // TODO: improve this
    fn compute(&mut self, dt: f32) {
        let len = self.components.len();

        for i in 0..len {
            // Split the slice at index `i + 1` to get two non-overlapping slices
            let (from_slice, rest) = self.components.split_at_mut(i + 1);
            let from = &mut from_slice[i]; // Safe mutable borrow

            for to in rest {
                println!("{} ({:?}) -> {} ({:?})", from.name, from.pos, to.name, to.pos);
                let distance_sq = from.pos.distance2(to.pos);
                let accel = (self.const_g * to.mass) / distance_sq;
                from.accelerate_to(accel * dt, to.pos);


                println!("{} ({:?}) -> {} ({:?})", to.name, to.pos, from.name, from.pos);
                let distance_sq = to.pos.distance2(from.pos);
                let accel = (self.const_g * from.mass) / distance_sq;
                to.accelerate_to(accel * dt, from.pos);
            }
        }

        self.components.iter_mut().for_each(|gbody| gbody.process(dt));
    }

    pub fn render<'a>(&'a mut self) -> Vec<&dyn Object> {
        self.components
            .iter_mut()
            .map(|gbody| {
                gbody.render();
                &gbody.gm_sphere as &dyn Object
            })
            .collect()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

pub async fn run() -> anyhow::Result<()> {
    let window = Window::new(WindowSettings {
        title: "Orbit!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut sun = GBody::new(&context, "sun", 0xffff00ff, 6.957, 1.989e30 / 1e23)?;
    sun.set_motion(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0));
    let mut earth = GBody::new(&context, "earth", 0x006994ff, 0.6371, 5.972e24 / 1e23)?;
        earth.set_motion(vec3(149.0 / 5.0, 0.0, 0.0), vec3(0.0, 0.006, 0.0));

    let mut pmesh = PhysicsMesh::default();
    pmesh.add(sun);
    pmesh.add(earth);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        Vector3::zero(),
        vec3(0.0, 0.5, 0.0),
        vec3(0.0, 0.0, 1.0),
        degrees(45.0),
        0.1,
        100000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 10000.0);

    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));

    //let skybox = Skybox::new_from_equirectangular(&context, &CpuTexture::default());

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        pmesh.compute(frame_input.elapsed_time as f32);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, pmesh.render().into_iter(), &[&light0, &light1]);

        FrameOutput::default()
    });

    Ok(())
}
