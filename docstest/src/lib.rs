use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

/// docs for the struct itself!
#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    /// docs for speed
    /// with newline
    /// 
    /// and with paragraph
    #[var]
    speed: f64,
    #[var]
    angular_speed: f64,

    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
}

#[godot_api]
impl Player {
    /// docs for increase_speed
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
    }

    #[func]
    fn decrease_speed(&mut self, amount: f64) {
        self.speed -= amount;
    }
}