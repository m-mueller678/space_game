use super::*;
use std::rc::Rc;
use std::cell::RefCell;
use graphics;

#[cfg(feature = "graphics")]
pub struct DrawArgs<'a, 'b> {
    pub target: &'a Ship,
    pub parent: &'b BaseShip,
}

pub struct TickArgs<F: FnMut(Projectile)> {
    pub target: Rc<RefCell<Ship>>,
    pub distance: i32,
    pub push_projectile: F,
    pub x: i32,
    pub y: i32,
}

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Clone, Debug)]
enum WeaponClass {
    Laser {
        #[cfg(feature = "graphics")]
        color: [u8; 3],
        power: u32,
    },
    Launcher {
        dmg: Damage,
        speed: i32,
        #[cfg_attr(not(feature = "graphics"), serde(skip_deserializing))]
        sprite: graphics::Sprite,
        cooldown: u32,
        launch_time: u32,
    }
}

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Clone, Debug)]
pub struct Weapon {
    range: i32,
    offset: (i32, i32),
    priority: i32,
    class: WeaponClass,
}

impl Weapon {
    pub fn control_move(&self, distance: i32) -> i32 {
        if self.range() >= distance {
            -self.priority
        } else {
            self.priority
        }
    }

    pub fn tick<F: FnMut(Projectile)>(&mut self, args: &mut TickArgs<F>) {
        match self.class {
            WeaponClass::Laser { power, .. } => if args.distance < self.range {
                args.target.borrow_mut().apply_damage(&Damage::Laser(power))
            },
            WeaponClass::Launcher { ref dmg, ref speed, ref cooldown, ref mut launch_time, ref sprite } => {
                *launch_time = launch_time.saturating_sub(1);
                if *launch_time == 0 && args.distance <= self.range {
                    let x = args.x + self.offset.0;
                    let y = args.y + self.offset.1;
                    (args.push_projectile)(Projectile::new(args.target.clone(), x, y, *speed, dmg.clone(), sprite.clone()));
                    *launch_time = *cooldown;
                }
            }
        }
    }

    pub fn range(&self) -> i32 {
        self.range
    }

    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T, draw: &DrawArgs) {
        match self.class {
            WeaponClass::Laser { ref color, .. } => {
                if (draw.target.pos_x() - draw.parent.pos_x()).abs() <= self.range {
                    rt.draw_line(
                        (self.offset.0 as f32, self.offset.1 as f32),
                        (
                            (draw.target.pos_x() - draw.parent.pos_x()) as f32,
                            (draw.target.pos_y() - draw.parent.pos_y()) as f32
                        ),
                        [color[0], color[1], color[2], 255]
                    );
                }
            },
            WeaponClass::Launcher { .. } => {}
        }
    }
}
