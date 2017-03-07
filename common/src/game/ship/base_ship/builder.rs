use super::*;

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Debug)]
pub struct BaseShipBuilder {
    laser_dmg_mult: u32,
    plasma_dmg_mult: u32,
    accel: i32,
    max_speed: i32,
    max_health: u32,
    weapons: Vec<Weapon>,
    #[cfg(feature = "graphics")]
    texture: Rc<graphics::CompositeTexture>,
}

impl BaseShipBuilder {
    #[cfg(feature = "graphics")]
    pub fn build(&self) -> BaseShip {
        BaseShip {
            target: Weak::new(),
            pos: 0,
            pos_y: 0,
            laser_dmg_mult: self.laser_dmg_mult,
            plasma_dmg_mult: self.plasma_dmg_mult,
            speed: 0,
            accel: self.accel,
            max_speed: self.max_speed,
            health: self.max_health,
            max_health: self.max_health,
            weapons: self.weapons.clone(),
            texture: self.texture.clone(),
        }
    }
    #[cfg(not(feature = "graphics"))]
    pub fn build(&self) -> BaseShip {
        BaseShip {
            target: Weak::new(),
            pos: 0,
            pos_y: 0,
            laser_dmg_mult: self.laser_dmg_mult,
            plasma_dmg_mult: self.plasma_dmg_mult,
            speed: 0,
            accel: self.accel,
            max_speed: self.max_speed,
            health: self.max_health,
            max_health: self.max_health,
            weapons: self.weapons.clone(),
        }
    }
}
