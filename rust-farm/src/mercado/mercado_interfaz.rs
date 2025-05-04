use godot::{classes::{Control, IControl}, prelude:: *};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MarketUI{
    base : Base<Control>
}