use godot::{classes::{class_macros::registry::class, CanvasLayer}, prelude::*};


#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
struct Mercado {
    base : Base<CanvasLayer>
}