pub mod base_ship;
pub mod weapon;
pub mod mothership;

use super::Lane;
use graphics;

pub use self::base_ship::BaseShip;
pub use self::mothership::Mothership;

pub enum Damage {
    Laser(u32),
}

pub enum Ship {
    BaseShip(BaseShip),
    Mothership(Mothership),
}

macro_rules! impl_method {
    ($name:ident,$ret:ty,$($parname:ident : $partype:ty),*)=>{
        fn $name(&self,$($parname:$partype),*)->$ret{
            match *self{
                Ship::Mothership(ref m)=>m.$name($($parname),*),
                Ship::BaseShip(ref m)=>m.$name($($parname),*),
            }
        }
    }
}
macro_rules! impl_mut_method {
    ($name:ident,$ret:ty,$($parname:ident : $partype:ty),*)=>{
        fn $name(&mut self,$($parname:$partype),*)->$ret{
            match *self{
                Ship::Mothership(ref mut m)=>m.$name($($parname),*),
                Ship::BaseShip(ref mut m)=>m.$name($($parname),*),
            }
        }
    }
}

impl ShipTrait for Ship {
    impl_method!(pos_x,i32,);
    impl_method!(pos_y,i32,);
    impl_method!(health,u32,);
    impl_method!(max_health,u32,);
    impl_method!(calc_damage,u32,dmg:&Damage);
    impl_mut_method!(tick,(),lane:usize,others:&[Lane]);
    impl_mut_method!(apply_damage,(),dmg:&Damage);
    impl_mut_method!(lane_changed,(),l:&Lane);
    #[cfg(feature = "graphics")]
    fn draw<T: graphics::RenderTarget>(&self, t: &mut T, lane: &Lane) {
        match *self {
            Ship::Mothership(ref m) => m.draw(t, lane),
            Ship::BaseShip(ref s) => s.draw(t, lane),
        }
    }
}


pub trait ShipTrait {
    fn pos_x(&self) -> i32;
    fn pos_y(&self) -> i32;
    fn tick(&mut self, lane: usize, others: &[Lane]);
    fn lane_changed(&mut self, _: &Lane) {}
    fn health(&self) -> u32;
    fn max_health(&self) -> u32;
    fn calc_damage(&self, dmg: &Damage) -> u32;
    fn apply_damage(&mut self, dmg: &Damage);
    #[cfg(feature = "graphics")]
    fn draw<T: graphics::RenderTarget>(&self, t: &mut T, lane: &Lane);
}
