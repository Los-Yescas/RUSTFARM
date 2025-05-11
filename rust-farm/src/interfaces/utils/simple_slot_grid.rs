use godot::{classes::{Button, CanvasItem, ColorRect, Control, IControl, Label, TextureRect}, prelude::*};

use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct SimpleGridSlot{
    base : Base<Control>,
    index : usize,
    #[var]
    item : Option<DynGd<RefCounted, dyn IItem>>,
    #[var]
    stock : u16
} 

#[godot_api]
impl IControl for SimpleGridSlot {
    fn ready(&mut self,) {
        let mut button = self.base().get_node_as::<Button>("ItemButton");
        let item_selected = self.base_mut().callable("item_selected");
        button.connect("pressed", &item_selected);

        if self.item == None{
            return;
        }
        
        let mouse_enter_callable = self.base_mut().callable("on_mouse_entered");
        button.connect("mouse_entered", &mouse_enter_callable);
        let mouse_exit_callable = self.base_mut().callable("on_mouse_exited");
        button.connect("mouse_exited", &mouse_exit_callable);

        let item = self.item.as_ref().expect("Sin item");

        let inner_border = self.base().get_node_as::<ColorRect>("InnerBorder");
        let mut texture = inner_border.get_node_as::<TextureRect>("Texture");
        let mut stock_label = inner_border.get_node_as::<Label>("Stock");

        
        texture.set_texture(&item.dyn_bind().get_sprite());
        stock_label.set_text(&format!("{}", self.stock));

        let details_panel = self.base().get_node_as::<ColorRect>("DetailsPanel");
        let mut name = details_panel.get_node_as::<Label>("Name"); 

        name.set_text(&item.dyn_bind().get_name());
    }
}

#[godot_api]
impl SimpleGridSlot {
    pub fn from_item_resource(resource : &DynGd<RefCounted, dyn IItem>, stock : u16, index : usize) -> Gd<SimpleGridSlot> {
        let slot = load::<PackedScene>("res://Interfaces/SimpleSlot.tscn");
        let resource = resource.clone();
        let mut slot = slot.instantiate_as::<SimpleGridSlot>();

        slot.bind_mut().set_index(index);
        slot.bind_mut().set_item(Some(resource.into_gd()));
        slot.bind_mut().set_stock(stock);

        slot
    }

    fn set_index(&mut self, index : usize){
        self.index = index;
    }

    pub fn new(index : usize) -> Gd<Self>{
        let slot = load::<PackedScene>("res://Interfaces/SimpleSlot.tscn");
        let mut slot = slot.instantiate_as::<SimpleGridSlot>();

        slot.bind_mut().set_index(index);

        slot
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
    pub fn item_selected(&mut self){
        self.base().get_viewport().unwrap().set_input_as_handled();

        let index = self.index as u16;
        self.base_mut().emit_signal("selected_item", &[index.to_variant()]);
    }

    pub fn disable(&self){
        let mut button = self.base().get_node_as::<Button>("ItemButton");
        button.set_disabled(true);
    }
}