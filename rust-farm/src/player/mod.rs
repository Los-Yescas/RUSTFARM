use godot::classes::Area2D;
use godot::classes::INode2D;
use godot::classes::InputEvent;
use godot::classes::Marker2D;
use godot::classes::Node2D;
use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::item::item_node::Item;
use crate::item::item_resource::IItem;
use crate::item::item_resource::plant_items::seed_item::SeedItemResource;
use crate::plant::FasesPlantas;
use crate::plant::Planta;
use crate::plant::plant_resource::PlantResource;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    #[export]
    speed: f32,
    input: Gd<Input>,
    is_moving: bool,
    target_position: Vector2,
    inventory: Vec<(Gd<RefCounted>, u16)>,
    item_actual: usize,
    debug_inventory: Vec<String>,
    #[export]
    inventario_maximo: u16,
    base: Base<Node2D>,
    #[export]
    dinero: u32,
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            speed: 500.0,
            base,
            input: Input::singleton(),
            is_moving: false,
            target_position: Vector2::ZERO,
            inventory: Vec::new(),
            item_actual: 0,
            inventario_maximo: 40,
            debug_inventory: Vec::new(),
            dinero: 0,
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.input.is_action_just_pressed("inventory") {
            godot_print!("{:#?}", self.inventory);
        }

        if self.is_moving {
            return;
        }
        self.interaction_system();

        if self.input.is_action_pressed("up") {
            self.move_to(Vector2i::UP);
        } else if self.input.is_action_pressed("down") {
            self.move_to(Vector2i::DOWN)
        } else if self.input.is_action_pressed("right") {
            self.move_to(Vector2i::RIGHT)
        } else if self.input.is_action_pressed("left") {
            self.move_to(Vector2i::LEFT)
        }
    }
    fn physics_process(&mut self, delta: f64) {
        if self.is_moving {
            let global_position = self.base().get_global_position();
            if global_position == self.target_position {
                self.is_moving = false;
                return;
            }
            let new_position =
                global_position.move_toward(self.target_position, self.speed * delta as f32);
            self.base_mut().set_global_position(new_position);
        }
    }
    // fn input(&mut self, event: Gd<InputEvent>) {
    //     self.interaction_system_inputs(event);
    // }
}

#[godot_api]
impl Player {
    #[func]
    fn move_to(&mut self, direction: Vector2i) {
        let map = self.base().get_node_as::<TileMapLayer>("../Suelo");
        let current_tile = map.local_to_map(self.base().get_global_position());
        let target_tile = Vector2i {
            x: current_tile.x + direction.x,
            y: current_tile.y + direction.y,
        };

        let tile_data = map.get_cell_tile_data(target_tile);
        let walkable: bool = match tile_data {
            None => return,
            Some(tile) => tile.get_custom_data("walkable").to::<bool>(),
        };

        if walkable {
            self.is_moving = true;
            self.target_position = map.map_to_local(target_tile);
            self.base_mut()
                .get_node_as::<Node2D>("./InteractZone")
                .set_rotation(direction.cast_float().angle());
        }
    }

    fn interaction_system(&mut self) {
        if self.input.is_action_just_pressed("pick") {
            if let Some(object) = self.check_for_item() {
                if let Ok(item) = object.try_cast::<Item>() {
                    self.add_to_inventory(item);
                }
            }
        } else if self.input.is_action_just_pressed("interact") {
            self.interact();
        } else if self.input.is_action_just_pressed("harvest") {
            // Nueva acción
            self.harvest();
        }
    }
    fn check_for_item(&self) -> Option<Gd<Node2D>> {
        let collider: Gd<Area2D> = self.base().get_node_as("./InteractZone/Area2D");
        let objects_in_area = collider.get_overlapping_areas();
        let object = objects_in_area.get(0);
        match object {
            None => None,
            Some(area2d) => Some(area2d.get_parent().expect("Sin padre").cast()),
        }
    }
    #[func]
    pub fn add_to_inventory(&mut self, mut item: Gd<Item>) {
        // Get the item resource as a dynamic IItem
        let Some(dyn_item) = item.bind().get_item_resource_as_dyn() else {
            godot_error!("El ítem no tiene recurso asociado");
            return;
        };

        // Get basic item properties through the IItem interface
        let max_stack = dyn_item.dyn_bind().get_max_stack();
        let item_name = dyn_item.dyn_bind().get_name();
        let item_resource = dyn_item.into_gd();

        // Try to find existing stack of this item
        let existing_stack = self.inventory.iter_mut().find(|(res, _)| {
            // Compare either as SeedItemResource or PlantResource
            if let Ok(res1) = res.clone().try_cast::<SeedItemResource>() {
                if let Ok(res2) = item_resource.clone().try_cast::<SeedItemResource>() {
                    return res1.bind().get_name() == res2.bind().get_name();
                }
            }
            if let Ok(res1) = res.clone().try_cast::<PlantResource>() {
                if let Ok(res2) = item_resource.clone().try_cast::<PlantResource>() {
                    return res1.bind().get_name() == res2.bind().get_name();
                }
            }
            false
        });

        match existing_stack {
            Some((_, stack)) => {
                if *stack < max_stack {
                    *stack += 1;
                    item.queue_free();
                    godot_print!("Apilado {} ({}/{})", item_name, *stack, max_stack);
                } else {
                    godot_print!("¡Stack máximo para {}!", item_name);
                }
            }
            None => {
                if self.inventory.len() < self.inventario_maximo as usize {
                    self.inventory.push((item_resource, 1));
                    item.queue_free();
                    godot_print!("¡Añadido {} al inventario!", item_name);
                } else {
                    godot_print!("¡Inventario lleno!");
                }
            }
        }
    }

    fn interact(&mut self) {
        // 1. Extraer todas las propiedades necesarias primero
        let position = self
            .base()
            .get_node_as::<Marker2D>("./InteractZone/SpawnerPos")
            .get_global_position();

        let parent = self.base().get_parent().unwrap().cast::<Node2D>();

        // 2. Verificar si podemos plantar aquí
        if !self.can_plant_here(position) {
            godot_print!("¡No puedes plantar aquí!");
            return;
        }

        // 3. Obtener item del inventario (sin mantener el borrow)
        let item_and_stack = self.inventory.get(self.item_actual).cloned();

        // 4. Procesar la interacción
        if let Some((item, stack)) = item_and_stack {
            if let Ok(seed) = item.try_cast::<SeedItemResource>() {
                // Interactuar sin borrow conflictivo
                seed.bind().interact(parent, position);

                // Actualizar inventario
                if stack == 1 {
                    self.inventory.remove(self.item_actual);
                } else {
                    self.inventory[self.item_actual].1 -= 1;
                }
            }
        }
    }

    // Función optimizada para verificación
    fn can_plant_here(&self, position: Vector2) -> bool {
        let tilemap = self.base().get_node_as::<TileMapLayer>("../Suelo");
        let tile_pos = tilemap.local_to_map(position);

        // Verifica 1: Tile marcado como plantable
        let is_plantable = tilemap
            .get_cell_tile_data(tile_pos)
            .map(|data| data.get_custom_data("plantable").to::<bool>())
            .unwrap_or(false);

        // Verifica 2: No hay objetos en el área
        is_plantable && self.check_for_obstacles(position).is_none()
    }
    // Función auxiliar simplificada
    fn check_for_obstacles(&self, position: Vector2) -> Option<Gd<Node2D>> {
        let area = self.base().get_node_as::<Area2D>("./InteractZone/Area2D");
        area.get_overlapping_bodies()
            .iter_shared()
            .next()
            .map(|body| body.cast::<Node2D>())
    }
    pub fn get_equiped_item(&self) -> Option<&(Gd<RefCounted>, u16)> {
        self.inventory.get(self.item_actual)
    }
    fn create_item_from_resource(&self, resource: Gd<PlantResource>) -> Gd<Item> {
        // Convertir a RefCounted manteniendo el tipo concreto
        let resource_rc = resource.upcast::<RefCounted>();

        // Crear el ítem directamente
        Item::create_with_resource(resource_rc)
    }

    fn harvest(&mut self) {
        // 1. Check for nearby plant node
        let Some(plant_node) = self.check_for_item() else {
            godot_print!("No hay plantas para cosechar");
            return;
        };

        // 2. Verify it's a valid plant
        let Ok(mut plant) = plant_node.try_cast::<Planta>() else {
            godot_print!("El objeto no es una planta válida");
            return;
        };

        // 3. Check if plant is mature
        if !plant.bind().is_mature() {
            godot_print!("La planta no está madura todavía");
            return;
        }

        // 4. Try to harvest the resource
        let Some(resource) = plant.bind_mut().try_harvest() else {
            godot_print!("La planta no tiene recurso cosechable");
            return;
        };

        // 5. Create item from resource using the proper method
        let item = Item::create_with_resource(resource.upcast::<RefCounted>());

        // 6. Check if we can add to inventory
        if self.can_add_to_inventory(&item) {
            self.add_to_inventory(item);
            plant.queue_free();
            godot_print!("Planta cosechada con éxito");
        } else {
            godot_print!("Inventario lleno, no se cosechó la planta");
            // Optionally, you could drop the item on the ground here
        }
    }
    fn can_add_to_inventory(&self, item: &Gd<Item>) -> bool {
        let Some(dyn_item) = item.bind().get_item_resource_as_dyn() else {
            godot_error!("El ítem no tiene recurso asociado");
            return false;
        };

        let max_stack = dyn_item.dyn_bind().get_max_stack();
        let item_resource = dyn_item.into_gd();

        match self.inventory.iter().find(|(res, _)| {
            // Similar comparison logic as above
            if let Ok(res1) = res.clone().try_cast::<SeedItemResource>() {
                if let Ok(res2) = item_resource.clone().try_cast::<SeedItemResource>() {
                    return res1.bind().get_name() == res2.bind().get_name();
                }
            }
            if let Ok(res1) = res.clone().try_cast::<PlantResource>() {
                if let Ok(res2) = item_resource.clone().try_cast::<PlantResource>() {
                    return res1.bind().get_name() == res2.bind().get_name();
                }
            }
            false
        }) {
            Some((_, current_stack)) => *current_stack < max_stack,
            None => self.inventory.len() < self.inventario_maximo as usize,
        }
    }

    fn check_for_plant(&self) -> Option<Gd<Planta>> {
        let collider = self.base().get_node_as::<Area2D>("./InteractZone/Area2D");
        godot_print!(
            "Áreas superpuestas: {}",
            collider.get_overlapping_areas().len()
        );

        for (i, area) in collider.get_overlapping_areas().iter_shared().enumerate() {
            godot_print!("Objeto {}: {:?}", i, area.get_class());

            if let Some(parent) = area.get_parent() {
                godot_print!("Padre: {:?}", parent.get_class());
                if let Ok(plant) = parent.try_cast::<Planta>() {
                    godot_print!("Planta encontrada!");
                    return Some(plant);
                }
            }
        }
        None
    }
    fn add_to_debug_inventory(&mut self, item_name: String) {
        // Versión con Vec (sin contador)
        self.debug_inventory.push(item_name.clone());
        godot_print!("Añadido al inventario debug: {}", item_name);

        // O versión con HashMap (con contador)
        // let count = self.debug_inventory.entry(item_name).or_insert(0);
        // *count += 1;
    }

    // Mostrar el inventario en consola
    #[func]
    pub fn show_debug_inventory(&self) {
        godot_print!("=== INVENTARIO DEBUG ===");

        // Versión Vec
        for (i, item) in self.debug_inventory.iter().enumerate() {
            godot_print!("Slot {}: {}", i, item);
        }
    }
    #[func]
    pub fn sell_current_item(&mut self) {
        if self.inventory.is_empty() {
            godot_print!("Inventario vacío. Nada que vender.");
            return;
        }

        if self.item_actual >= self.inventory.len() {
            godot_print!("Ítem seleccionado inválido.");
            return;
        }

        let (recurso, cantidad) = &mut self.inventory[self.item_actual];

        // Solo permitir venta si es una planta
        if let Ok(planta) = recurso.clone().try_cast::<PlantResource>() {
            let precio = planta.bind().get_price();

            if precio == 0 {
                godot_print!("Esta planta no tiene precio de venta.");
                return;
            }

            self.dinero += precio as u32;
            *cantidad -= 1;

            godot_print!(
                "Vendiste una planta por {} monedas. Dinero total: {}",
                precio,
                self.dinero
            );

            if *cantidad == 0 {
                self.inventory.remove(self.item_actual);
                if self.item_actual >= self.inventory.len() && self.item_actual > 0 {
                    self.item_actual -= 1;
                }
            }
        } else {
            godot_print!("Solo se pueden vender plantas.");
        }
    }
}
