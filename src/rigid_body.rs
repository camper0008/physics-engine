use crate::scene::{FluidMolarWeight, Scene};
use crate::vector2::Vector2;

pub trait RigidBody {
    fn vel(&self) -> Vector2;
    fn pos(&self) -> Vector2;
    fn mass(&self) -> f64;
    fn tick(&mut self, delta: f64);
    fn owner(&self) -> Scene;
}

pub struct Rectangle {
    pub m_pos: Vector2,
    pub m_vel: Vector2,
    pub m_mass: f64,
    pub m_owner: Scene,
    pub width: f64,
    pub height: f64,
    pub m_time_fallen: f64,
}

impl RigidBody for Rectangle {
    fn vel(&self) -> Vector2 {
        self.m_vel
    }
    fn pos(&self) -> Vector2 {
        self.m_pos
    }
    fn mass(&self) -> f64 {
        self.m_mass
    }
    fn owner(&self) -> Scene {
        self.m_owner
    }
    fn tick(&mut self, delta: f64) {
        // f=m*a hvor a = gravity_constant
        let gravity_constant = self.owner().gravity;
        let gravity_force =
            Vector2::from(self.mass()) * gravity_constant * Vector2::from(self.m_time_fallen);
        // D = Cd * A * .5 * r * V^2
        // where
        // D=drag
        // Cd=drag_coefficient
        // A=area
        // r=density
        // V=velocity
        let area = self.width * self.height;
        let drag_unit = Vector2::from(self.owner().drag_coefficient)
            * Vector2::from(area)
            * Vector2::from(self.owner().fluid_density(FluidMolarWeight::Air) / 1000.0)
            * self.vel()
            * self.vel()
            * Vector2::from(0.5);
        println!(
            "fluid_density: {}, area: {},",
            self.owner().fluid_density(FluidMolarWeight::Air),
            area,
        );
        let drag_force = self.vel().normalized() * drag_unit;
        println!(
            "vel_x: {}, vel_y: {}, drag_x: {}, drag_y: {},",
            self.m_vel.x, self.m_vel.y, drag_force.x, drag_force.y
        );
        let res_force = (self.m_vel - drag_force - gravity_force) * Vector2::from(delta);
        self.m_pos = self.m_pos + res_force;
        self.m_time_fallen += delta;
    }
}
