use godot::{classes::{AudioStream, Texture2D, TileMapLayer}, prelude::*};

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
    usos : u16,
    #[export]
    sonido : Option<Gd<AudioStream>>
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
    fn interact(&mut self, world : Gd<Node2D>, position : Vector2, _objeto : Option<Gd<Node2D>>) -> Result<bool, GString> {
        let mut tiles = world.get_node_as::<TileMapLayer>("Suelo");
        let tile_position = tiles.local_to_map(position);
        let tile_data = tiles.get_cell_tile_data(tile_position).unwrap();
        let tile_terrain = tile_data.get_terrain();

        let mut _used : bool = false;
        match self.efecto {
            Efecto::AumentarFertilidad => {
                if tile_terrain==2 || tile_terrain==3 {
                    let _ = tiles.set_cells_terrain_connect(&array![tile_position], 0, tile_terrain-1);
                    _used = true;
                }
                else{
                    _used = false;
                }
            },
            Efecto::ConstruirPuente =>{
                if tile_terrain==5{
                    let _ = tiles.set_cells_terrain_connect(&array![tile_position], 0, 6);
                    _used = true;
                }else{
                    _used = false;
                }
            },
            Efecto::QuitarPasto=>{
                if tile_terrain == 4{
                    let _ = tiles.set_cells_terrain_connect(&array![tile_position], 0, 3);
                    _used = true;
                }else {
                    _used = false;
                }
            },
            Efecto::RecogerFruto=>{godot_print!("Probablemente no se implemente"); _used =false},
            Efecto::RevivirTierra=>{
                if tile_terrain == 0 {
                    let _ = tiles.set_cells_terrain_connect(&array![tile_position], 0, 3);
                    _used = true;
                }else {
                    _used = false;
                }
            }
        };
        if _used {
            self.usos -= 1;
        }else {
            return Err("No fue usada".into());
        }
        Ok(self.usos <= 0)
    }

    fn get_interact_sound(&self) -> Option<Gd<AudioStream>> {
        self.sonido.clone()
    } 
}