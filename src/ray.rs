use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        return self.origin + (t * self.direction);
    }
}
