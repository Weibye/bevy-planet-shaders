use bevy::{app::{App, Plugin, Startup, Update}, asset::Assets, math::{Rect, Vec2, Vec3}, pbr::{wireframe::{Wireframe, WireframeConfig, WireframePlugin}, PbrBundle, StandardMaterial}, prelude::{Commands, Component, Cuboid, Direction3d, Plane3d, Query, ResMut, Sphere}, render::{camera::Camera, color::Color, mesh::{shape, Indices, Mesh, Meshable, PrimitiveTopology}, render_asset::RenderAssetUsages}, transform::{components::{GlobalTransform, Transform}, TransformBundle}, ui::node_bundles};

pub(crate) struct PcgPlanetPlugin;

impl Plugin for PcgPlanetPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_planet);
        app.add_plugins(WireframePlugin);
        app.insert_resource(WireframeConfig {
                // The global wireframe config enables drawing of wireframes on every mesh,
                // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
                // regardless of the global configuration.
                global: false,
                // Controls the default color of all wireframes. Used as the default color for global wireframes.
                // Can be changed per mesh using the `WireframeColor` component.
                default_color: Color::WHITE,
            });
        // app.add_systems(Update, hierarcical_lod);
    }
}

fn spawn_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const RADIUS: f32 = 100.0;

    // commands.spawn((PbrBundle {
    //     mesh: meshes.add(spherical_cuboid(RADIUS, 4, false)),
    //     material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
    //     ..Default::default()
    //     },
    //     Wireframe
    // ));
    const LOD_LEVELS: u32 = 4;
    for i in 0..LOD_LEVELS {

    }
    let quad = Quad {
        rect: Rect::from_center_size(Vec2::ZERO, Vec2::splat(RADIUS)),
        transform: Transform::from_translation(Vec3::ZERO),
    };
    let children = calculate_lod_quads(&quad);

    for n in children {
        commands.spawn((n.clone(), n.transform.clone()));
        commands.spawn((PbrBundle {
            mesh: meshes.add(n.mesh()),
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            transform: n.transform.clone(),
            ..Default::default()
            },
            Wireframe
        ));
    }
    commands.spawn((quad.clone(), TransformBundle::default()));


    commands.spawn((PbrBundle {
            mesh: meshes.add(quad.mesh()),
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            transform: quad.transform.clone(),
            ..Default::default()
            },
            Wireframe
        ));

    // // Each quad should have its own AABB
}

pub(crate) fn spherical_cuboid(radius: f32, subdivisions: u32, inflate: bool) -> Mesh {

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
                indices.push(start);
                indices.push(start + 1);
                indices.push(start + subdivisions + 1);
                indices.push(start + 1);
                indices.push(start + subdivisions + 2);
                indices.push(start + subdivisions + 1);
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

fn calculate_lod_quads(parent: &Quad) -> Vec<Quad>
{
    let mut quads = Vec::new();
    for dir in [
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, 1.0)
    ] {

        let quad = Quad {
            rect: Rect::from_center_size(Vec2::ZERO, parent.rect.half_size()),
            transform: parent.transform * Transform::from_translation(
                Vec3::new(dir.x, 0.0, dir.y)
                * Vec3::new(parent.rect.half_size().x, 0.0, parent.rect.half_size().y) / 2.0
            )
        };
        quads.push(quad);
    }
    quads
}

fn hierarcical_lod(
    camera: Query<(&Camera, &GlobalTransform)>,
    quads: Query<(&Quad, &GlobalTransform)>,
) {
    let (player_camera, player_camera_transform) = camera.get_single().unwrap();

    // Query all LodQuads
    // For each quad, check its visible size to the camera
    for (quad, transform) in quads.iter() {
        // Calculate the distance from the camera to the quad
        let distance = (transform.translation() - player_camera_transform.translation()).length();
        println!("Distance: {}", distance);
        // Calculate the size of the quad in screen space
        // Calculate the LOD level based on the distance and size
        // If the quad is too small, merge it with its parent
    }
    // if the quad is too large, split it into 4 children


}

#[derive(Component, Clone)]
struct Quad {
    pub rect: Rect,
    pub transform: Transform,
}

impl Meshable for Quad {
    type Output = Mesh;

    fn mesh(&self) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new(); // Optional: For texture mapping
        let mut indices = Vec::new();

        // Generate vertices and normals for each face
        positions.push([self.rect.min.x, 0.0, self.rect.min.y]);
        positions.push([self.rect.max.x, 0.0, self.rect.min.y]);
        positions.push([self.rect.max.x, 0.0, self.rect.max.y]);
        positions.push([self.rect.min.x, 0.0, self.rect.max.y]);

        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);

        uvs.push([0.0, 0.0]);
        uvs.push([1.0, 0.0]);
        uvs.push([1.0, 1.0]);
        uvs.push([0.0, 1.0]);

        indices.push(0);
        indices.push(2);
        indices.push(1);
        indices.push(0);
        indices.push(3);
        indices.push(2);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }
}