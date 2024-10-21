mod ext;
mod serializers;

use serializers as ser;

use serde::{Deserialize, Serialize};
use three_d::{degrees, vec3, Degrees, Srgba, Vector3, Zero};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ConfigGlobal {
    pub window_name: String,
    pub window_size: Option<(u32, u32)>,
    pub const_g: f32,
}

impl Default for ConfigGlobal {
    fn default() -> Self {
        Self {
            window_name: String::from("N-Body Gravity Simlation!"),
            window_size: Some((1280, 720)),
            const_g: crate::consts::GRAVITATIONAL_CONSTANT,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ConfigCheats {
    pub time_mult: f64,
}

impl Default for ConfigCheats {
    fn default() -> Self {
        Self {
            time_mult: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ConfigCamera {
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    pub position: Vector3<f32>,
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    pub target: Vector3<f32>,
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    pub up: Vector3<f32>,
    #[serde(
        serialize_with = "ser::serialize_degrees",
        deserialize_with = "ser::deserialize_degrees"
    )]
    pub fov: Degrees,
    pub render_distance: (f32, f32),
}

impl Default for ConfigCamera {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            target: vec3(0.0, 0.5, 0.0),
            up: Vector3::unit_z(),
            fov: degrees(45.0),
            render_distance: (0.01, f32::MAX / 2.0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigBody {
    pub name: String,
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    pub position: Vector3<f32>,
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    pub velocity: Vector3<f32>,
    #[serde(
        serialize_with = "ser::serialize_srgba",
        deserialize_with = "ser::deserialize_srgba"
    )]
    pub color: Srgba,
    pub radius: f32,
    pub mass: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigDirectionalLight {
    intensity: f32,
    #[serde(
        serialize_with = "ser::serialize_srgba",
        deserialize_with = "ser::deserialize_srgba"
    )]
    color: Srgba,
    #[serde(
        serialize_with = "ser::serialize_vector3",
        deserialize_with = "ser::deserialize_vector3"
    )]
    position: Vector3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub global: ConfigGlobal,
    pub camera: ConfigCamera,
    pub bodies: Vec<ConfigBody>,
    pub directional_lights: Vec<ConfigDirectionalLight>,
    pub cheats: ConfigCheats,
}
