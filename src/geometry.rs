use bevy::{math::Vec3, render::{mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}};

/// A module for generating various geometric shapes.

/// Creates a spherical cuboid mesh with the given radius and subdivisions.
pub(crate) fn spherical_cuboid(radius: f32, subdivisions: u32, invert: bool, inflate: bool) -> Mesh {

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new(); // Optional: For texture mapping
    let mut indices = Vec::new();

    // Find the details of the inscribed cube
    let half = radius / 2.0;
    let step = radius / subdivisions as f32;

    // Generate vertices and normals for each face
    for face in 0..6 {
        let (dir, u, v) = match face {
            0 => (Vec3::X, Vec3::Y, Vec3::Z), // Positive X
            1 => (-Vec3::X, Vec3::Z, Vec3::Y), // Negative X
            2 => (Vec3::Y, Vec3::Z, Vec3::X), // Positive Y
            3 => (-Vec3::Y, Vec3::X, Vec3::Z), // Negative Y
            4 => (Vec3::Z, Vec3::X, Vec3::Y), // Positive Z
            5 => (-Vec3::Z, Vec3::Y, Vec3::X), // Negative Z
            _ => unreachable!(),
        };

        for i in 0..=subdivisions {
            for j in 0..=subdivisions {
                let offset = (dir + u + v) * -half;

                // Calculate the vertex position of the cubiod
                let mut pos = offset + step * (i as f32 * u + j as f32 * v);

                let normal = if inflate {
                    (pos - Vec3::ZERO).normalize()
                } else {
                    dir
                };

                // Displace the vertex of the cubiod according to the radius
                // This will turn it into a sphere
                if inflate { pos = normal * radius; }

                positions.push([pos.x, pos.y, pos.z]);
                normals.push([normal.x, normal.y, normal.z]);

                // Optional: Calculate UVs for texture mapping
                uvs.push([i as f32 / subdivisions as f32, j as f32 / subdivisions as f32]);
            }
        }

        // Generate indices for each face
        let offset = face * (subdivisions + 1) * (subdivisions + 1);
        for i in 0..subdivisions {
            for j in 0..subdivisions {
                let start = offset + i * (subdivisions + 1) + j;
                if invert {
                    // Clockwise winding
                    indices.push(start);
                    indices.push(start + subdivisions + 1);
                    indices.push(start + 1);
                    indices.push(start + 1);
                    indices.push(start + subdivisions + 1);
                    indices.push(start + subdivisions + 2);
                } else {
                    // Counter-clockwise winding (original order)
                    indices.push(start);
                    indices.push(start + 1);
                    indices.push(start + subdivisions + 1);
                    indices.push(start + 1);
                    indices.push(start + subdivisions + 2);
                    indices.push(start + subdivisions + 1);
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}