use three_d::*;

pub struct GBody {
    pub name: String,
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub color: Srgba,
    pub radius: f32,
    pub mass: f32,
    pub gm_sphere: Gm<Mesh, PhysicalMaterial>,
}

impl GBody {
    pub fn new(
        ctx: &Context,
        name: String,
        color: Srgba,
        radius: f32,
        mass: f32,
    ) -> Result<Self, three_d_asset::Error> {
        let mut sphere_mesh = CpuMesh::sphere(16);
        sphere_mesh.transform(&Mat4::from_scale(radius))?;
        let gm_sphere = Gm::new(
            Mesh::new(&ctx, &sphere_mesh),
            PhysicalMaterial::new_transparent(
                &ctx,
                &CpuMaterial {
                    albedo: color,
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

pub struct PhysicsMesh {
    pub const_g: f32,
    pub components: Vec<GBody>,
}

impl Default for PhysicsMesh {
    fn default() -> Self {
        Self {
            const_g: crate::consts::GRAVITATIONAL_CONSTANT,
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
    pub fn compute(&mut self, dt: f32) {
        let len = self.components.len();

        for i in 0..len {
            let (from_slice, rest) = self.components.split_at_mut(i + 1);
            let from = &mut from_slice[i]; // Safe mutable borrow

            for to in rest {
                //println!(
                //    "{} ({:?}) -> {} ({:?})",
                //    from.name, from.pos, to.name, to.pos
                //);
                let distance_sq = from.pos.distance2(to.pos);
                let accel = (self.const_g * to.mass) / distance_sq;
                from.accelerate_to(accel * dt, to.pos);

                //println!(
                //    "{} ({:?}) -> {} ({:?})",
                //    to.name, to.pos, from.name, from.pos
                //);
                let distance_sq = to.pos.distance2(from.pos);
                let accel = (self.const_g * from.mass) / distance_sq;
                to.accelerate_to(accel * dt, from.pos);
            }
        }

        self.components
            .iter_mut()
            .for_each(|gbody| gbody.process(dt));
    }

    pub fn render(&mut self) -> Vec<&dyn Object> {
        self.components
            .iter_mut()
            .map(|gbody| {
                gbody.render();
                &gbody.gm_sphere as &dyn Object
            })
            .collect()
    }
}
