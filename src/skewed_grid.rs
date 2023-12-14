use rand::{rngs::SmallRng, Rng, SeedableRng};
use rerun::external::glam;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("skewed grid").spawn()?;

    // Skew the points so that the spheres are packed in a hexagonal pattern.
    let cos_30 = 30.0_f32.to_radians().cos();
    let mut z_axis = glam::vec3(0.5, cos_30 / 3.0, 0.0);
    z_axis.z = (1.0 - z_axis.x * z_axis.x - z_axis.y * z_axis.y).sqrt();
    let world_from_grid =
        glam::Mat3::from_cols_array_2d(&[[1.0, 0.0, 0.0], [0.5, cos_30, 0.0], z_axis.into()]);
    rec.log_timeless(
        "axes",
        &rerun::Arrows3D::from_vectors([
            world_from_grid.x_axis,
            world_from_grid.y_axis,
            world_from_grid.z_axis,
        ])
        .with_colors([
            rerun::Color::from_u32(0xaa0000ff),
            rerun::Color::from_u32(0x00aa00ff),
            rerun::Color::from_u32(0x0000aaff),
        ])
        .with_origins([glam::vec3(0.0, 0.0, 0.0); 3]),
    )?;
    rec.log_timeless(
        "bounds",
        &rerun::Arrows3D::from_vectors([
            world_from_grid.x_axis,
            world_from_grid.x_axis,
            world_from_grid.x_axis,
            world_from_grid.x_axis,
            world_from_grid.y_axis,
            world_from_grid.y_axis,
            world_from_grid.y_axis,
            world_from_grid.y_axis,
            world_from_grid.z_axis,
            world_from_grid.z_axis,
            world_from_grid.z_axis,
            world_from_grid.z_axis,
        ])
        .with_colors([rerun::Color::from_u32(0xaaaaaaff)])
        .with_origins([
            glam::vec3(0.0, 0.0, 0.0),
            world_from_grid.y_axis,
            world_from_grid.z_axis,
            world_from_grid.y_axis + world_from_grid.z_axis,
            glam::vec3(0.0, 0.0, 0.0),
            world_from_grid.x_axis,
            world_from_grid.z_axis,
            world_from_grid.x_axis + world_from_grid.z_axis,
            glam::vec3(0.0, 0.0, 0.0),
            world_from_grid.x_axis,
            world_from_grid.y_axis,
            world_from_grid.x_axis + world_from_grid.y_axis,
        ]),
    )?;

    let mut rng = SmallRng::seed_from_u64(89236740);
    for lod in 2..8 {
        rec.set_time_sequence("LOD", lod as i64);
        let n = 1 << lod;
        let node_radius = 0.6 / n as f32;
        let jitter_radius = 0.5 / n as f32;
        let mut points = Vec::new();
        let positive_sphere_center = world_from_grid * glam::vec3(0.5, 0.5, 0.5);
        let positive_sphere_radius = 0.3;
        let negative_sphere_center = world_from_grid * glam::vec3(0.35, 0.5, 0.5);
        let negative_sphere_radius = 0.25;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let p_in_grid = glam::vec3(i as f32, j as f32, k as f32) / n as f32;
                    let mut jitter = glam::vec3(
                        rng.gen::<f32>() * jitter_radius,
                        rng.gen::<f32>() * jitter_radius,
                        rng.gen::<f32>() * jitter_radius,
                    );
                    // Reject samples outside the jitter radius.
                    while jitter.length_squared() > jitter_radius * jitter_radius {
                        jitter = glam::vec3(
                            rng.gen::<f32>() * jitter_radius,
                            rng.gen::<f32>() * jitter_radius,
                            rng.gen::<f32>() * jitter_radius,
                        );
                    }
                    let p = world_from_grid * p_in_grid + jitter;
                    if (p - negative_sphere_center).length() + node_radius < negative_sphere_radius
                    {
                        continue;
                    }
                    if (p - positive_sphere_center).length() + node_radius < positive_sphere_radius
                    {
                        points.push(p);
                    }
                }
            }
        }
        rec.log(
            "my_points",
            &rerun::Points3D::new(points.clone()).with_radii([node_radius]),
        )?;
    }

    Ok(())
}
