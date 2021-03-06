use crate::scene::{FluidMolarWeight, Scene};
use crate::vector2::Vector2;

pub trait RigidBody {
    fn vel(&self) -> Vector2;
    fn pos(&self) -> Vector2;
    fn mass(&self) -> f64;
    fn tick(&mut self, delta: f64);
    fn owner(&self) -> Scene;
    fn drag_coefficient(&self) -> f64;
    fn area(&self) -> f64;
}

pub struct Rectangle {
    pub m_pos: Vector2,
    pub m_vel: Vector2,
    pub m_mass: f64,
    pub m_owner: Scene,
    pub width: f64,
    pub height: f64,
    pub m_time_fallen: f64,
    pub m_drag_coefficient: f64,
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
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn owner(&self) -> Scene {
        self.m_owner
    }
    fn drag_coefficient(&self) -> f64 {
        self.m_drag_coefficient
    }
    fn tick(&mut self, delta: f64) {
        // f=m*a where a = gravity_constant
        let gravity_constant = self.owner().gravity;
        let gravity_force =
            Vector2::from(self.mass()) * gravity_constant * Vector2::from(self.m_time_fallen);
        // D = C * A * .5 * r * v^2
        // where
        // D=drag force
        // C=drag coefficient
        // A=area
        // r=density
        // v=speed
        // we convert velocity to speed by getting the length of it
        let area = self.width * self.height;
        let drag_force = self.m_drag_coefficient
            * area
            * self.owner().fluid_density(FluidMolarWeight::Air)
            * self.vel().len().powf(2.0)
            * 0.5
            * 0.001 // because we got air density in g/m^3 and we want kg/m^3
            * -1.0; // because it's in the opposite direction

        println!("drag: {}", drag_force);
        println!(
            "air resistance: {}",
            self.owner().fluid_density(FluidMolarWeight::Air)
        );

        // since F=ma, then a=F/m; times by the time taken to get velocity
        let drag_velocity = (drag_force / self.m_mass) * delta;
        println!("drag_vel: {}", drag_velocity);
        // then convert to a vector in the proper direction
        let drag_vector = self.m_vel.normalized() * Vector2::from(drag_velocity);
        let res_force = (self.m_vel + drag_vector + gravity_force) * Vector2::from(delta);
        self.m_pos = self.m_pos + res_force;
        self.m_time_fallen += delta;
    }
}
