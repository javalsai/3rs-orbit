pub struct SceneLights {
    pub lights: Vec<Box<dyn ShadowableLight>>,
}

impl SceneLights {
    pub fn render(
        &mut self,
        texture_size: u32,
        geometries: &[&three_d::Mesh],
    ) -> Vec<&dyn three_d::Light> {
        for light in &mut self.lights {
            light.clear_shadow_map();
            light.generate_shadow_map(texture_size, geometries);
        }
        self.lights
            .iter()
            .map(|l| l.as_ref() as &dyn three_d::Light)
            .collect()
    }
}

pub trait ShadowableLight: three_d::Light {
    fn clear_shadow_map(&mut self);

    fn generate_shadow_map(
        &mut self,
        texture_size: u32,
        geometries: &[&three_d::Mesh],
    );
}

macro_rules! impl_shadowable_light {
    ($light_type:ty) => {
        impl ShadowableLight for $light_type {
            fn clear_shadow_map(&mut self) {
                <$light_type>::clear_shadow_map(self)
            }
            fn generate_shadow_map(
                &mut self,
                texture_size: u32,
                geometries: &[&three_d::Mesh],
            ) {
                <$light_type>::generate_shadow_map(self, texture_size, geometries)
            }
        }
    };
}

impl<T: three_d::Light> ShadowableLight for T {
    default fn clear_shadow_map(&mut self) {}

    default fn generate_shadow_map(
        &mut self,
        _texture_size: u32,
        _geometries: &[&three_d::Mesh],
    ) {}
}
impl_shadowable_light!(three_d::DirectionalLight);
impl_shadowable_light!(three_d::SpotLight);
