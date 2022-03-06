use crate::util::vec::Vector3f;

pub mod object;

// Distancia con signo a una esfera desde un punto P desde con centro C y radio C
pub fn signed_dst_to_sphere(p: Vector3f, c: Vector3f, r: f32) -> f32 {
    let dst_vect = Vector3f {
        x: c.x - p.x,
        y: c.y - p.y,
        z: c.z - p.z,
    };

    return dst_vect.length() - r;
}
