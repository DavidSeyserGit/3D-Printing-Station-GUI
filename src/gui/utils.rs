use raylib::prelude::*;

pub fn calculate_model_transform(vertices: &Vec<[f32; 3]>, width: i32, height: i32) -> (Vector3, f32) {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;

    for vertex in vertices {
        min_x = min_x.min(vertex[0]);
        max_x = max_x.max(vertex[0]);
        min_y = min_y.min(vertex[1]);
        max_y = max_y.max(vertex[1]);
        min_z = min_z.min(vertex[2]);
        max_z = max_z.max(vertex[2]);
    }

    let center = Vector3::new(
        (min_x + max_x) / 2.0,
        (min_y + max_y) / 2.0,
        (min_z + max_z) / 2.0,
    );

    let max_dimension = f32::max(max_x - min_x, f32::max(max_y - min_y, max_z - min_z));
    let scale = f32::min(width as f32, height as f32) / max_dimension * 0.4;

    (center, scale)
}

pub fn rotate_y(v: Vector3, angle: f32) -> Vector3 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    Vector3 {
        x: v.x * cos_theta - v.z * sin_theta,
        y: v.y,
        z: v.x * sin_theta + v.z * cos_theta,
    }
}

pub fn rotate_x(v: Vector3, angle: f32) -> Vector3 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    Vector3 {
        x: v.x,
        y: v.y * cos_theta - v.z * sin_theta,
        z: v.y * sin_theta + v.z * cos_theta,
    }
}
