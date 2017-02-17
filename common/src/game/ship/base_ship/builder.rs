use super::*;

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize)]
pub struct BaseShipBuilder {
    laser_dmg_mult: u32,
    accel: i32,
    max_speed: i32,
    max_health: u32,
    weapons: Vec<Weapon>,
    #[cfg_attr(not(feature = "graphics"), serde(skip_deserializing))]
    texture: graphics::IfGraphics<Rc<graphics::CompositeTexture>>,
}

impl BaseShipBuilder {
    pub fn build(&self) -> BaseShip {
        BaseShip {
            target: Weak::new(),
            pos: 0,
            pos_y: 0,
            laser_dmg_mult: self.laser_dmg_mult,
            speed: 0,
            accel: self.accel,
            max_speed: self.max_speed,
            health: self.max_health,
            max_health: self.max_health,
            weapons: self.weapons.clone(),
            draw_move: Default::default(),
            texture: self.texture.clone(),
        }
    }
}
