//! Demonstrates the most barebone usage of the Rerun SDK.

use rerun::external::glam;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("skewed grid").spawn()?;

    // Skew the points so that the circles are stacked
    let cos_30 = 30.0_f32.to_radians().cos();
    let mut z_axis = glam::vec3(0.5, cos_30 / 3.0, 0.0);
    z_axis.z = (1.0 - z_axis.x * z_axis.x - z_axis.y * z_axis.y).sqrt();
    let skewed_from_straight =
        glam::Mat3::from_cols_array_2d(&[[1.0, 0.0, 0.0], [0.5, cos_30, 0.0], z_axis.into()]);
    let mut points = Vec::new();
    let n = 100;
    let positive_sphere_center =
        skewed_from_straight * glam::vec3(0.5 * n as f32, 0.5 * n as f32, 0.5 * n as f32);
    let positive_sphere_radius = 0.3 * n as f32;
    let negative_sphere_center =
        skewed_from_straight * glam::vec3(0.25 * n as f32, 0.5 * n as f32, 0.5 * n as f32);
    let negative_sphere_radius = 0.2 * n as f32;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let p_in_grid = glam::vec3(i as f32, j as f32, k as f32);
                let p = skewed_from_straight * p_in_grid;
                if (p - negative_sphere_center).length() < negative_sphere_radius {
                    continue;
                }
                if (p - positive_sphere_center).length() < positive_sphere_radius {
                    points.push(p);
                }
            }
        }
    }
    rec.log(
        "my_points",
        &rerun::Points3D::new(points.clone()).with_radii([0.6]),
    )?;
    rec.log(
        "axes",
        &rerun::Arrows3D::from_vectors([
            skewed_from_straight.x_axis,
            skewed_from_straight.y_axis,
            skewed_from_straight.z_axis,
        ])
        .with_colors([
            rerun::Color::from_u32(0xaa0000ff),
            rerun::Color::from_u32(0x00aa00ff),
            rerun::Color::from_u32(0x0000aaff),
        ])
        .with_origins([glam::vec3(11.0, 9.0, 6.0); 3]),
    )?;

    Ok(())
}
