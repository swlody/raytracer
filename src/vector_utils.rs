use cgmath::Vector3;

pub fn dot(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub fn vector_multiply(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn unit_vector(vector: &Vector3<f32>) -> Vector3<f32> {
    return vector / dot(vector, vector).sqrt();
}

pub fn vector_length(vector: &Vector3<f32>) -> f32 {
    dot(&vector, &vector).sqrt()
}
