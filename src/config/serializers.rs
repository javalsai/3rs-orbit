use serde::{ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};
use three_d::{Srgba, Vector3, Degrees, degrees};

pub fn serialize_vector3<S, T: Serialize>(
    vec: &Vector3<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tup = serializer.serialize_tuple(3)?;
    tup.serialize_element(&vec.x)?;
    tup.serialize_element(&vec.y)?;
    tup.serialize_element(&vec.z)?;
    tup.end()
}

pub fn deserialize_vector3<'de, D, T: Copy + Deserialize<'de>>(
    deserializer: D,
) -> Result<Vector3<T>, D::Error>
where
    D: Deserializer<'de>,
{
    let tuple = <[T; 3]>::deserialize(deserializer)?;
    Ok(Vector3 {
        x: tuple[0],
        y: tuple[1],
        z: tuple[2],
    })
}

pub fn serialize_srgba<S>(
    color: &Srgba,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tup = serializer.serialize_tuple(4)?;
    tup.serialize_element(&color.r)?;
    tup.serialize_element(&color.g)?;
    tup.serialize_element(&color.b)?;
    tup.serialize_element(&color.a)?;
    tup.end()
}

pub fn deserialize_srgba<'de, D>(
    deserializer: D,
) -> Result<Srgba, D::Error>
where
    D: Deserializer<'de>,
{
    let tuple = <[u8; 4]>::deserialize(deserializer)?;
    Ok(Srgba {
        r: tuple[0],
        g: tuple[1],
        b: tuple[2],
        a: tuple[3],
    })
}

pub fn serialize_degrees<S>(
    deg: &Degrees,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f32(deg.0)
}

pub fn deserialize_degrees<'de, D>(
    deserializer: D,
) -> Result<Degrees, D::Error>
where
    D: Deserializer<'de>,
{
    let deg = <f32>::deserialize(deserializer)?;
    Ok(degrees(deg))
}
