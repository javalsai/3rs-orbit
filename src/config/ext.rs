use three_d::*;

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
        let mut body = crate::physics::GBody::new(
            &ctx,
            self.name,
            self.color,
            self.radius,
            self.mass
        )?;
        body.set_motion(self.position, self.velocity);
        Ok(body)
    }
}

impl super::ConfigDirectionalLight {
    pub fn as_dlight(self, ctx: &Context) -> DirectionalLight {
        DirectionalLight::new(
            &ctx,
            self.intensity,
            self.color,
            &self.position,
        )
    }
}
