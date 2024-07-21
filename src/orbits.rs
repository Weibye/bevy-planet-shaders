use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    math::{Quat, Vec3},
    prelude::{Component, Entity, Gizmos, Query, Res},
    time::Time,
    transform::components::Transform,
    utils::hashbrown::HashMap,
};

/// A node in an orbital system
#[derive(Component)]
pub(crate) enum OrbitalNode {
    /// The root node of the system
    Root,
    Intermediate {
        /// The radius of the orbit around the parent node
        radius: f32,
        /// The parent node of this node
        parent_node: Entity,
        /// Orbital period of the node
        orbital_period: f32,
    },
}

/// A body in an orbital system
#[derive(Component)]
pub(crate) struct OrbitalBody {
    // /// The orbital node of this body
    // parent_node: Entity,
    /// The mass of the body
    pub(crate) mass: f32,
    /// The radius of the body
    pub(crate) radius: f32,
    /// Angular velocity of the body
    pub(crate) angular_momentum: f32,
}

impl OrbitalBody {
    /// Get the angular velocity of the body
    fn get_angular_velocity(&self) -> f32 {
        let moment_of_inertia = (2.0 / 5.0) * self.mass * self.radius.powi(2);
        self.angular_momentum / moment_of_inertia
    }
}

pub(crate) struct OrbitalPlugin;

impl Plugin for OrbitalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_orbital_nodes,
                update_orbital_bodies,
                draw_orbit_gizmos,
            ),
        );
    }
}

fn update_orbital_nodes(mut q: Query<(&mut Transform, &OrbitalNode, Entity)>, time: Res<Time>) {
    let map: HashMap<Entity, Transform> = q
        .iter()
        .map(|(transform, _, entity)| (entity, *transform))
        .collect();

    for (mut transform, node, entity) in q.iter_mut() {
        match node {
            OrbitalNode::Root => {}
            OrbitalNode::Intermediate {
                radius,
                parent_node,
                orbital_period,
            } => {
                let parent_transform = map.get(parent_node).unwrap();

                // Calculate the angular velocity of the node
                let omega = 2.0 * PI / orbital_period;

                // Calculate the angular displacement of the node
                let theta = omega * time.elapsed_seconds() as f32;

                // Assuming the orbit lies in the XZ plane and rotates around the Y axis
                // Calculate the new position using quaternion rotation
                let rotation = Quat::from_rotation_y(theta); // Rotate around Y axis
                let relative_position = Vec3::new(*radius, 0.0, 0.0); // Position relative to parent, assuming starting at (radius, 0, 0)
                let rotated_position = rotation.mul_vec3(relative_position);

                // Update the planet's position
                transform.translation = parent_transform.translation + rotated_position;
            }
        }
    }
}

fn update_orbital_bodies(mut q: Query<(&mut Transform, &OrbitalBody)>, time: Res<Time>) {
    for (mut transform, body) in q.iter_mut() {
        let angular_velocity = body.get_angular_velocity();
        transform.rotate(Quat::from_rotation_y(
            angular_velocity as f32 * time.delta_seconds() as f32,
        ));
    }
}

fn draw_orbit_gizmos(mut gizmos: Gizmos, q: Query<(&Transform, &OrbitalNode, Entity)>) {
    let map: HashMap<Entity, &Transform> = q
        .iter()
        .map(|(transform, _, entity)| (entity, transform))
        .collect();

    for (_, node, _) in q.iter() {
        match node {
            OrbitalNode::Root => {}
            OrbitalNode::Intermediate {
                radius,
                parent_node,
                orbital_period: _,
            } => {
                let parent_transform = map.get(parent_node).unwrap();

                // Draw a line from the parent node to this node
                gizmos.circle(
                    parent_transform.translation,
                    parent_transform.up(),
                    *radius as f32,
                    Color::WHITE,
                );
            }
        }
    }
}
