use crate::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Scene {
    pub gravity: Vector2,
    pub temperature: f64,
    pub pressure: f64,
}

pub enum FluidMolarWeight {
    // g over mol
    Air = 29,
}

impl Scene {
    pub fn fluid_density(&self, molar_weight: FluidMolarWeight) -> f64 {
        // we rewrite the ideal gas law to calculate density
        // since density=m/V
        // we start with the ideal gas law
        // PV=nRT
        // then we isolate n
        // PV/RT = n
        // since n can be described as m over M, we can times with M on both sides to get m
        // PV/RT = m/M
        // PV/RT * M = m
        // then we simply divide by volume to make it become m/V
        // (PV/RT * M) / V = m / V
        // which, when reorganised becomes
        // (1 / (R * T)) * P * M * V * (1 / V) = m / V (move all divisions into 1 over x)
        // (1 / (R * T)) * P * M = m / V (calculate V*1/V = 1)
        // (P * M) / (R * T) = m / V (move 1 / x back into the original)
        // PM / RT = m / V = density (simplify)
        (self.pressure * molar_weight as u64 as f64) / (8.31 * self.temperature)
    }
}
