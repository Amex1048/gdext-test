use godot::engine::{Sprite2D, Sprite2DVirtual};
use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    #[base]
    base: Base<Sprite2D>,

    speed: f32,
    angular_speed: f32,
}

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Player {
            base,
            speed: 400.0,
            angular_speed: std::f32::consts::PI,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let rotation = self.angular_speed * delta as f32;
        self.base.rotate(rotation);

        let velocity = Vector2::UP.rotated(self.get_rotation()) * self.speed;
        self.base.translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f32) {
        self.speed += amount;
        self.emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}

// struct SomeStruct;
//
// impl SomeStruct {
//     fn new();
// }
