use godot::{classes::Sprite2D, prelude::*};

use super::item_resource::{plant_items::seed_item::SeedItemResource, IItem};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Item {
    #[export]
    item_path : GString,
    #[var]
    item_resource : Option<DynGd<RefCounted, dyn IItem>>,
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Item{
    fn ready(&mut self,) {
        let resource:Gd<Resource> = load(&self.item_path);
        let variant : Variant;
        if let Ok(semilla) = resource.try_cast::<SeedItemResource>() {
            variant = semilla.to_variant();
        }else {
            godot_error!("No hay item conocido");
            return;
        }
        self.item_resource = variant.to();

        //cambiando sprite
        let mut sprite = self.base_mut().get_node_as::<Sprite2D>("./Sprite2D");
        let textura = &self.item_resource.as_ref().expect("No hay recurso").dyn_bind().get_sprite();
        sprite.set_texture(textura);
    }
}

// #[godot_api]
// impl Item{
//     fn get_resource_as_dyngd(&self) -> Option<DynGd<RefCounted, dyn IItem>>{

//     }
// }