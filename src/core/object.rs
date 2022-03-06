use crate::util::vec::Vector3f;

pub trait Object {
    fn signed_dst_from_point(&self, point: &Vector3f) -> f32;
}

struct Sphere {
    // Centro
    c: Vector3f,
    // Radio
    r: f32,
}

// Todos los puntos de la caja están a la misma distancia del centro
struct Box {
    // Tamaño de la caja desde el centro
    s: Vector3f,
    // Centro
    c: Vector3f,
}

impl Object for Sphere {
    // Distancia con signo a una esfera desde un punto
    fn signed_dst_from_point(&self, point: &Vector3f) -> f32 {
        let c_p = Vector3f {
            x: self.c.x - point.x,
            y: self.c.y - point.y,
            z: self.c.z - point.z,
        };

        return c_p.length() - self.r;
    }
}

impl Object for Box {
    // Distancia con signo a una caja desde un punto
    // https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
    // https://www.youtube.com/watch?v=Cp5WWtMoeKg&t=236s
    fn signed_dst_from_point(&self, p: &Vector3f) -> f32 {
        let o = Vector3f {
            x: (p.x - self.c.x).abs() - self.s.x,
            y: (p.y - self.c.y).abs() - self.s.y,
            z: (p.z - self.c.z).abs() - self.s.z,
        };
        
        let ud = Vector3f {
            x: o.x.max(0.0),
            y: o.y.max(0.0),
            z: o.z.max(0.0),
        }.length();

        let n = f32::max(f32::max(f32::min(o.x, 0f32), f32::min(o.y, 0f32)), f32::min(o.z, 0f32));
    
        return ud + n;
    }
}
