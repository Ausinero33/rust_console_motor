use crate::util::vec::Vector3f;

pub trait Object {
    fn signed_dst_from_point(&self, point: Vector3f) -> f32;
}

pub struct Sphere {
    // Centro
    c: Vector3f,
    // Radio
    r: f32,
}

// Todos los puntos de la caja están a la misma distancia del centro
pub struct Cube {
    // Tamaño de la caja desde el centro
    s: Vector3f,
    // Centro
    c: Vector3f,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Sphere {
        Sphere {
            c: Vector3f {x, y, z},
            r,
        }
    }
}

impl Cube {
    pub fn new(sx: f32, sy: f32, sz: f32, cx: f32, cy: f32, cz: f32) -> Cube {
        Cube { 
            s: Vector3f { x: sx, y: sy, z: sz }, 
            c: Vector3f { x: cx, y: cy, z: cz } 
        }
    }
}

impl Object for Sphere {
    // Distancia con signo a una esfera desde un punto
    fn signed_dst_from_point(&self, point: Vector3f) -> f32 {
        let c_p = self.c - point;

        // let c_p = Vector3f {
        //    x: self.c.x - point.x,
        //    y: self.c.y - point.y,
        //    z: self.c.z - point.z,
        // };

        return c_p.length() - self.r;
    }
}

impl Object for Cube {
    // Distancia con signo a una caja desde un punto
    // https://iquilezles.org/www/articles/distfunctions/distfunctions.htm
    // https://www.youtube.com/watch?v=Cp5WWtMoeKg&t=236s
    fn signed_dst_from_point(&self, p: Vector3f) -> f32 {
        // let o = Vector3f {
        //     x: (p.x - self.c.x).abs() - self.s.x,
        //     y: (p.y - self.c.y).abs() - self.s.y,
        //     z: (p.z - self.c.z).abs() - self.s.z,
        // };
        
        let o = (p - self.c).abs() - self.s;
        
        // let ud = Vector3f {
        //     x: o.x.max(0.0),
        //     y: o.y.max(0.0),
        //     z: o.z.max(0.0),
        // }.length();

        let ud = o.max_vec(0.0).length();

        let n = f32::max(f32::max(f32::min(o.x, 0f32), f32::min(o.y, 0f32)), f32::min(o.z, 0f32));
    
        return ud + n;
    }
}
