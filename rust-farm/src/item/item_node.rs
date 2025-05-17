use godot::{classes::Sprite2D, prelude::*};

use crate::world_interactables::IWorldPickable;

use super::item_resource::IItem;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Item {
    #[export]
    item_path : GString,
    #[var]
    item_resource : Option<DynGd<RefCounted, dyn IItem>>,
    base: Base<Node2D>
}
#[godot_dyn]
pub impl IWorldPickable for Item{
    fn pick(&self) -> Option<&DynGd<RefCounted, dyn IItem>> {
        self.item_resource.as_ref()
    }
    fn has_been_picked(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl INode2D for Item{
    fn ready(&mut self,) {
        if !self.item_path.is_empty(){
            let resource: Gd<Resource> = load(&self.item_path);
            self.item_resource = resource.to_variant().to();
        }
        

        

        let mut sprite = self.base_mut().get_node_as::<Sprite2D>("./Sprite2D");
        let textura = &self.item_resource.as_ref().expect("No hay recurso").dyn_bind().get_sprite();
        sprite.set_texture(textura);
    }
}

impl Item {
    pub fn from_resource(item : DynGd<RefCounted, dyn IItem>) -> Gd<Item>{
        let escene = load::<PackedScene>("res://Items/ItemNode.tscn");
        let mut item_node = escene.instantiate_as::<Item>();
        item_node.bind_mut().set_item_resource(Some(item.to_variant().to()));

        item_node
    }
}

