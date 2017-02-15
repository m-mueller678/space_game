use super::*;

#[cfg(feature = "graphics")]
pub struct DrawArgs<'a, 'b> {
    pub target: &'a Ship,
    pub parent: &'b BaseShip,
}

pub struct TargetArgs<'a> {
    pub ship: &'a mut Ship,
    pub distance: i32,
}

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Debug, Clone)]
enum WeaponClass {
    Laser {
        color: [u8; 3],
        power: u32,
    }
}

#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    range: i32,
    offset: graphics::IfGraphics<(i32, i32)>,
    priority: i32,
    class: WeaponClass,
}

impl Weapon {
    pub fn new_laser(pow: u32, range: i32, priority: i32) -> Self {
        Weapon {
            range: range,
            offset: Default::default(),
            priority: priority,
            class: WeaponClass::Laser {
                color: Default::default(),
                power: pow,
            }
        }
    }

    pub fn control_move(&self, distance: i32) -> i32 {
        if self.range() >= distance {
            -self.priority
        } else {
            self.priority
        }
    }

    pub fn tick(&mut self, target: &mut TargetArgs) {
        match self.class {
            WeaponClass::Laser { power, .. } => if target.distance < self.range {
                target.ship.apply_damage(&Damage::Laser(power))
            }
        }
    }

    pub fn range(&self) -> i32 {
        self.range
    }

    pub fn damage_100(&self, target: &Ship) -> u32 {
        match self.class {
            WeaponClass::Laser { power, .. } => target.calc_damage(&Damage::Laser(power)) * 100
        }
    }

    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T, draw: &DrawArgs) {
        use sfml::graphics::*;
        use sfml::system::Vector2f;
        match self.class {
            WeaponClass::Laser { ref color, .. } => {
                if (draw.target.pos_x() - draw.parent.pos_x()).abs() <= self.range {
                    let x1 = (draw.parent.pos_x() + self.offset.0) as f32;
                    let y1 = (draw.parent.pos_y() + self.offset.1) as f32;
                    let x2 = draw.target.pos_x() as f32;
                    let y2 = draw.target.pos_y() as f32;
                    let color = Color::new_rgb(color[0], color[1], color[2]);
                    let ver = [
                        Vertex::new_with_pos_color(&Vector2f::new(x1, y1), &color),
                        Vertex::new_with_pos_color(&Vector2f::new(x2, y2), &color),
                    ];
                    rt.draw_primitives(&ver, PrimitiveType::sfLines, &mut RenderStates::default());
                }
            }
        }
    }
}