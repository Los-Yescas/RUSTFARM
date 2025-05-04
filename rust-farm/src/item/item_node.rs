use godot::{classes::Sprite2D, prelude::*};

use super::item_resource::{IItem, plant_items::seed_item::SeedItemResource};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Item {
    #[export]
    item_path: GString,
    #[var]
    item_resource: Option<DynGd<RefCounted, dyn IItem>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Item {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            item_path: GString::new(),
            item_resource: None,
            base,
        }
    }
    fn ready(&mut self) {
        let resource: Gd<Resource> = load(&self.item_path);
        let variant: Variant;
        if let Ok(semilla) = resource.try_cast::<SeedItemResource>() {
            variant = semilla.to_variant();
        } else {
            godot_error!("No hay item conocido");
            return;
        }
        self.item_resource = variant.to();

        //cambiando sprite
        let mut sprite = self.base_mut().get_node_as::<Sprite2D>("./Sprite2D");
        let textura = &self
            .item_resource
            .as_ref()
            .expect("No hay recurso")
            .dyn_bind()
            .get_sprite();
        sprite.set_texture(textura);
    }
}
#[godot_api]
impl Item {
    #[func]
    pub fn create_with_resource(resource: Gd<RefCounted>) -> Gd<Self> {
        // 1. Obtener el path de forma segura
        let path = match resource.clone().try_cast::<Resource>() {
            Ok(res) => res.get_path(),
            Err(_) => {
                godot_error!("El recurso no es de tipo Resource");
                "".into()
            }
        };

        // 2. Conversión a DynGd con manejo de errores
        let dyn_item = match resource
            .to_variant()
            .try_to::<DynGd<RefCounted, dyn IItem>>()
        {
            Ok(dyn_item) => dyn_item,
            Err(_) => {
                godot_error!("El recurso no implementa IItem");
                return Gd::from_init_fn(|base| Self {
                    item_path: "invalid".into(),
                    item_resource: None,
                    base,
                });
            }
        };

        Gd::from_init_fn(|base| Self {
            item_path: path,
            item_resource: Some(dyn_item),
            base,
        })
    }
    #[func]
    pub fn get_item_resource_dyn(&self) -> Option<Gd<RefCounted>> {
        self.item_resource.clone().map(|dyn_gd| dyn_gd.into_gd())
    }

    // Versión adicional que devuelve DynGd (opcional)
    #[func]
    pub fn get_item_resource_as_dyn(&self) -> Option<DynGd<RefCounted, dyn IItem>> {
        self.item_resource.clone()
    }
    #[func]
    pub fn set_item_resource_dyn(&mut self, resource: Gd<RefCounted>) {
        match resource
            .to_variant()
            .try_to::<DynGd<RefCounted, dyn IItem>>()
        {
            Ok(dyn_item) => self.item_resource = Some(dyn_item),
            Err(_) => godot_error!("El recurso no implementa IItem"),
        }
    }
}
