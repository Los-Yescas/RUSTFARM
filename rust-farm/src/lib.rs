use godot::{classes::Engine, prelude::*};
//Singletons
mod game_manager; //Este es anadido aqui mismo
mod time_system;

//Otras entidades
pub mod player;
pub mod plant;
pub mod item;
pub mod mercado;
pub mod level_manager;
pub mod seed_processor;
pub mod herramientas;

pub mod world_interactables;


//Interfaces
pub mod interfaces;
struct MyExtension;



#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton()
            .register_singleton("GameManager", &game_manager::GameManager::new_alloc());
        }
    }
    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Let's keep a variable of our Engine singleton instance,
            // and MyEngineSingleton name.
            let mut engine = Engine::singleton();
            let singleton_name = "GameManager";

            // Here, we manually retrieve our singleton(s) that we've registered,
            // so we can unregister them and free them from memory - unregistering
            // singletons isn't handled automatically by the library.
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // You can either recover, or panic from here.
                godot_error!("Failed to get singleton");
            }
        }
    }
}