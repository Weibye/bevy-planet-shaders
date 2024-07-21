use bevy::prelude::Entity;

struct Sun {
    pub seed: u64,
    pub temperature: f64,
}

struct Planet {
    pub seed: u64,
}

struct Moon {
    pub seed: u64,
}
