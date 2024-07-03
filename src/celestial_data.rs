use bevy::prelude::Entity;

struct Sun {
    pub seed: u64,
    pub mass: f64,
    pub radius: f64,
    pub temperature: f64,
}

struct Planet {
    pub mass: f64,
    pub radius: f64,
    pub seed: u64,
}

struct Moon {
    pub mass: f64,
    pub radius: f64,
    pub seed: u64,
}

struct Orbit {
    pub radius: f64,
    pub primary_body: Option<Entity>,
}
