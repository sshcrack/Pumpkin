use pumpkin_data::particle::ParticleType;

pub struct Particle {
    particle_type: ParticleType
}

impl Particle {
    pub const fn new(particle_type: ParticleType) -> Self {
        Self { particle_type }
    }

    pub const fn get_type(&self) -> ParticleType {
        self.particle_type
    }
}