use godot::{classes::Control, prelude:: *};

#[derive(GodotClass)]
#[class(init, base=Control)]
struct MarketUI{
    base : Base<Control>
}