use godot::{classes::class_macros::registry::class, prelude::*};


#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Mercado {
    base : Base<Node2D>
}