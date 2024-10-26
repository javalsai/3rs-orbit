use three_d::*;

use crate::physics::lighting::ShadowableLight;

impl super::ConfigCamera {
    pub fn as_camera(self, viewport: Viewport) -> Camera {
        Camera::new_perspective(
            viewport,
            self.position,
            self.target,
            self.up,
            self.fov,
            self.render_distance.0,
            self.render_distance.1,
        )
    }
}

impl super::ConfigBody {
    pub fn as_gbody(self, ctx: &Context) -> Result<crate::physics::GBody, three_d_asset::Error> {
        let mut body =
            crate::physics::GBody::new(&ctx, self.name, self.color, self.radius, self.mass)?;
        body.set_motion(self.position, self.velocity);
        Ok(body)
    }
}

pub trait IntoDynLight {
    fn into_dyn_light(self, ctx: &Context) -> Box<dyn ShadowableLight>;
}

impl super::ConfigLights {
    pub fn as_scene_lighting(self, ctx: &Context) -> crate::physics::lighting::SceneLights {
        let shadowable_lights = std::iter::empty::<Box<dyn ShadowableLight>>()
            .chain(self.directional.into_iter().map(|l| l.into_dyn_light(ctx)))
            .chain(self.ambient.into_iter().map(|l| l.into_dyn_light(ctx)));

        crate::physics::lighting::SceneLights { lights: shadowable_lights.collect() }
    }
}

impl super::ConfigDirectionalLight {
    pub fn as_dlight(self, ctx: &Context) -> DirectionalLight {
        DirectionalLight::new(&ctx, self.intensity, self.color, &self.direction)
    }
}
impl IntoDynLight for super::ConfigDirectionalLight {
    fn into_dyn_light(self, ctx: &Context) -> Box<dyn ShadowableLight> {
        Box::new(self.as_dlight(ctx))
    }
}

impl super::ConfigAmbientLight {
    pub fn as_alight(self, ctx: &Context) -> AmbientLight {
        AmbientLight::new(&ctx, self.intensity, self.color)
    }
}
impl IntoDynLight for super::ConfigAmbientLight {
    fn into_dyn_light(self, ctx: &Context) -> Box<dyn ShadowableLight> {
        Box::new(self.as_alight(ctx))
    }
}

