use godot::{classes::{Button, CanvasItem, ColorRect, Control, IControl, Label, TextureRect}, obj::WithBaseField, prelude:: *};

use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct GridSlot{
    base : Base<Control>,
    item : Option<DynGd<RefCounted, dyn IItem>>,
    factor_precio : f32,
    stock : u16
} 

#[godot_api]
impl IControl for GridSlot {
    fn ready(&mut self,) {

        let item = self.item.as_ref().expect("Sin item");

        let inner_border = self.base().get_node_as::<ColorRect>("InnerBorder");
        let mut texture = inner_border.get_node_as::<TextureRect>("Texture");
        let mut price = inner_border.get_node_as::<Label>("Price");
        let mut stock_label = inner_border.get_node_as::<Label>("Stock");

        
        texture.set_texture(&item.dyn_bind().get_sprite());
        let real_price = (item.dyn_bind().get_precio() as f32 * self.factor_precio) as u16;
        price.set_text(&format!("{} $", real_price));
        stock_label.set_text(&format!("{}", self.stock));

        let details_panel = self.base().get_node_as::<ColorRect>("DetailsPanel");
        let mut name = details_panel.get_node_as::<Label>("Name"); 
        let mut description = details_panel.get_node_as::<Label>("Description"); 

        name.set_text(&item.dyn_bind().get_name());
        description.set_text(&item.dyn_bind().get_description());

        let mut button = self.base().get_node_as::<Button>("ItemButton");
        let mouse_enter_callable = self.base_mut().callable("on_mouse_entered");
        button.connect("mouse_entered", &mouse_enter_callable);
        let mouse_exit_callable = self.base_mut().callable("on_mouse_exited");
        button.connect("mouse_exited", &mouse_exit_callable);
        let buy_callable = self.base_mut().callable("buy_button_pressed");
        button.connect("pressed", &buy_callable);
    }
}

#[godot_api]
impl GridSlot {
    #[func]
    pub fn from_item_resource(&mut self, resource : DynGd<RefCounted, dyn IItem>, stock : u16, factor_precio : f32)  {
        self.item = Some(resource);
        self.stock = stock;
        self.factor_precio = factor_precio;
    }

    #[func]
    pub fn on_mouse_entered(&mut self,){
        self.base_mut().get_node_as::<CanvasItem>("DetailsPanel").set_visible(true);
    }
    #[func]
    pub fn on_mouse_exited(&mut self){
        self.base_mut().get_node_as::<CanvasItem>("DetailsPanel").set_visible(false);
    }

    #[func]
    pub fn buy_button_pressed(&mut self){
        let item = self.item.as_ref().expect("Slot sin Item").to_variant();

        self.base().get_viewport().unwrap().set_input_as_handled();

        self.base_mut().emit_signal("selected_item", &[item]);
    }
}