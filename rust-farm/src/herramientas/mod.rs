use godot::{classes::{Texture2D, TileMapLayer}, prelude::*};

use crate::item::item_resource::IItem;

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Efecto{
    QuitarPasto,
    AumentarFertilidad,
    RevivirTierra,
    ConstruirPuente,
    RecogerFruto
}


#[derive(GodotClass)]
#[class(init, base=Resource)]
struct Herramienta{
    base : Base<Resource>,
    #[export]
    #[init(val=Efecto::QuitarPasto)]
    efecto : Efecto,
    #[export]
    nombre : GString,
    #[export]
    descripcion : GString,
    #[export]
    max_stack : u16,
    #[export]
    precio : u16,
    #[export]
    textura : Option<Gd<Texture2D>>,
    #[export]
    usos : u16
}

#[godot_dyn]
impl IItem for Herramienta {
    fn pick(&self) -> DynGd<RefCounted, dyn IItem>{
        self.to_gd().duplicate_ex().subresources(true).done().unwrap().to_variant().to()
    }
    fn get_description(&self) -> GString {
        self.descripcion.clone()
    }
    fn get_max_stack(&self) -> u16 {
        self.max_stack
    }
    fn get_name(&self) -> GString {
        self.nombre.clone()
    }
    fn get_precio(&self) -> u16 {
        self.precio
    }
    fn get_sprite(&self) -> Gd<Texture2D> {
        self.textura.clone().unwrap_or(Texture2D::new_gd())
    }
    fn interact(&mut self, world : Gd<Node2D>, position : Vector2, _objeto : Option<Gd<Node2D>>) -> bool {
        let mut tiles = world.get_node_as::<TileMapLayer>("Suelo");
        let tile_position = tiles.local_to_map(position);
        let atlas_cords = tiles.get_cell_atlas_coords(tile_position);
        let tile_data = tiles.get_cell_tile_data(tile_position).unwrap();

        let modificable : bool = tile_data.get_custom_data("modificable").to();


        let mut used : bool = false;
        match self.efecto {
            Efecto::AumentarFertilidad => {
                if modificable {
                    if atlas_cords.x >=1 && atlas_cords.x <= 2 {
                        let _ = tiles.set_cell_ex(tile_position)
                        .atlas_coords(Vector2i { x: atlas_cords.x + 1, y: atlas_cords.y })
                        .source_id(1)
                        .done();
                        used = true;
                    }
                }else{
                    used = false
                }
            },
            Efecto::ConstruirPuente =>{
                if atlas_cords.x == 6 {
                    let _ = tiles.set_cell_ex(tile_position)
                        .atlas_coords(Vector2i { x: 8, y: atlas_cords.y })
                        .source_id(1)
                        .done();
                    used = true;
                }else{
                    used = false
                }
            },
            Efecto::QuitarPasto=>{
                if atlas_cords.x == 0 {
                    let _ = tiles.set_cell_ex(tile_position)
                        .atlas_coords(Vector2i { x: atlas_cords.x + 1, y: atlas_cords.y })
                        .source_id(1)
                        .done();
                    used = true;
                }else {
                    used = false
                }
            },
            Efecto::RecogerFruto=>{godot_print!("Probablemente no se implemente"); used =false},
            Efecto::RevivirTierra=>{
                if atlas_cords.x == 4 {
                    let _ = tiles.set_cell_ex(tile_position)
                        .atlas_coords(Vector2i { x: 1, y: atlas_cords.y })
                        .source_id(1)
                        .done();
                    used = true;
                }else {
                    used = false
                }
            }
        };
        godot_print!("{}", self.usos);
        if used {
            self.usos -= 1;
        }
        godot_print!("{}", self.usos);
        self.usos <= 0
    }
}