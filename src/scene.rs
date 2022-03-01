use crate::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Scene {
    pub gravity: Vector2,
    pub drag_coefficient: f64,
    pub temperature: f64,
    pub pressure: f64,
}

pub enum FluidMolarWeight {
    // g over mol
    Air = 29,
}

impl Scene {
    pub fn fluid_density(&self, molar_weight: FluidMolarWeight) -> f64 {
        // density = {pressure} over {{R over fluid_molar_weight} * temperature}
        // which is from the ideal gas law
        // where R is the universal gas constant,
        // R=8,31 [{ Pa*m^3 } over { mol*K }]
        // and the fluid_molar_weight is molar_weight
        self.pressure / ((8.31 / (molar_weight as u64 as f64)) * self.temperature)
    }
}
