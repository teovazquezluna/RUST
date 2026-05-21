/* 8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ Agregar canción.
➔ Eliminar canción.
➔ Mover canción (mueve la canción a una determinada posición de la playlist).
➔ Buscar canción por nombre.
➔ Obtener las canciones de un determinado género.
➔ Obtener las canciones de un determinado artista.
➔ Modificar título de la playlist.
➔ Eliminar todas las canciones. */

#[derive(Clone, Debug)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Clone, Debug,)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}
impl Cancion{
    pub fn new(titulo:String,artista:String,genero:Genero)->Cancion{
        Cancion{
        titulo,
        artista,
        genero,}
    }
}

pub struct Playlist {
    pub nombre: String,
    pub lista_de_canciones: Vec<Cancion>,
}

impl Playlist {
    pub fn new(nombre:String)->Playlist{
        Playlist{
            nombre,
            lista_de_canciones:Vec::new(),
        }
    }

    pub fn misma_cancion(c1: &Cancion, c2: &Cancion) -> bool {
        c1.titulo == c2.titulo && c1.artista == c2.artista
    }

    pub fn mismo_genero(gen1: &Genero, gen2: &Genero) -> bool {
        match (gen1, gen2) {
            (Genero::Rock, Genero::Rock) => true,
            (Genero::Pop, Genero::Pop) => true,
            (Genero::Rap, Genero::Rap) => true,
            (Genero::Jazz, Genero::Jazz) => true,
            (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.lista_de_canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) {
        for i in 0..self.lista_de_canciones.len() {
            if Playlist::misma_cancion(&self.lista_de_canciones[i], cancion) {
                self.lista_de_canciones.remove(i);
                return;
            }

        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, posicion: usize) {
        let posicion_cancion = self.buscar_cancion_por_nombre(&cancion.titulo);
        if let Some(cancion) = posicion_cancion{
            if posicion < self.lista_de_canciones.len(){
                self.lista_de_canciones.swap(cancion, posicion);
            }
        }else{
            panic!("Cancion no encontrada");
        }
    }

    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<usize> {

        for cancion in 0..self.lista_de_canciones.len() {
            if self.lista_de_canciones[cancion].titulo == titulo {
                return Some(cancion);
            }
        }
        None
    }
    

    pub fn obtener_las_canciones_de_un_determinado_genero(&self, genero: Genero) -> Vec<Cancion> {

        let mut canciones = Vec::new();
        for cancion in &self.lista_de_canciones {
            if Playlist::mismo_genero(&cancion.genero, &genero) {
                canciones.push(cancion.clone());
            }
        }
        canciones
    }

    pub fn obtener_las_canciones_de_un_determinado_artista(&self, artista: String) -> Vec<Cancion> {
        
        let mut canciones = Vec::new();
        for cancion in &self.lista_de_canciones {
            if cancion.artista == artista {
                canciones.push(cancion.clone());
            }
        }
        canciones
    }

    pub fn modificar_titulo_de_playlist(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    pub fn eliminar_todas_las_canciones(&mut self) {
        self.lista_de_canciones.clear();
    }
}

pub fn resolver() {
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new_playlist(){
        let nombre="playlist".to_string();
        let playlist=Playlist::new(nombre);

        assert_eq!(playlist.nombre,"playlist");
        assert!(playlist.lista_de_canciones.is_empty());
    }
    #[test]
    fn test_new_cancion(){
        let titulo="numb".to_string();
        let artista="linkin park".to_string();
        let genero=Genero::Rock;
        let cancion=Cancion{titulo,artista,genero};

        assert_eq!(cancion.titulo,"numb");
        assert_eq!(cancion.artista,"linkin park");
        assert!(Playlist::mismo_genero(&cancion.genero,&Genero::Rock));
}
    #[test]
    fn test_agregar_cancion(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let titulo="numb".to_string();
        let artista="linkin park".to_string();
        let genero=Genero::Rock;

        let cancion=Cancion::new(titulo, artista, genero);

        playlist.agregar_cancion(cancion);

        assert!(!playlist.lista_de_canciones.is_empty());
    }

    #[test]
    fn test_mismo_genero(){
        let genero1=Genero::Rock;
        let genero2=Genero::Rock;
        let genero3=Genero::Jazz;

        assert!(Playlist::mismo_genero(&genero1,&genero2));
        assert!(!Playlist::mismo_genero(&genero1,&genero3));
    }

    #[test]
    fn test_misma_cancion(){
        let cancion1=Cancion{titulo:"numb".to_string(),artista:"linkin park".to_string(),genero:Genero::Rock};
        let cancion2=Cancion{titulo:"numb".to_string(),artista:"linkin park".to_string(),genero:Genero::Rock};
        let cancion3=Cancion{titulo:"in the end".to_string(),artista:"linkin park".to_string(),genero:Genero::Rock};

        assert!(Playlist::misma_cancion(&cancion1,&cancion2));
        assert!(!Playlist::misma_cancion(&cancion1,&cancion3));
    }

    #[test]
    fn test_eliminar_cancion(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion=Cancion{titulo:"thriller".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};

        playlist.agregar_cancion(cancion.clone());
        playlist.eliminar_cancion(&cancion);

        assert!(playlist.lista_de_canciones.is_empty());
    }

    #[test]
    fn test_mover_cancion(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion1=Cancion{titulo:"cancion1".to_string(),artista:"artista1".to_string(),genero:Genero::Rock};
        let cancion2=Cancion{titulo:"cancion2".to_string(),artista:"artista2".to_string(),genero:Genero::Jazz};

        playlist.agregar_cancion(cancion1.clone());
        playlist.agregar_cancion(cancion2.clone());

        println!("{:?}",playlist.lista_de_canciones);

        playlist.mover_cancion(&cancion1,1);

        println!("{:?}",playlist.lista_de_canciones);

        assert_eq!(playlist.lista_de_canciones[0].titulo,"cancion2");
        assert_eq!(playlist.lista_de_canciones[1].titulo,"cancion1");
    }

    #[test]
    #[should_panic(expected = "Cancion no encontrada")]
    fn test_mover_cancion_should_panic(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion=Cancion{titulo:"thriller".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};

        playlist.mover_cancion(&cancion,0);
    }

    #[test]
    fn test_modificar_titulo_playlist(){
        let nombre="playlist vieja".to_string();
        let mut playlist=Playlist::new(nombre);
        let nuevo_titulo="playlist nueva".to_string();

        playlist.modificar_titulo_de_playlist(nuevo_titulo.clone());

        assert_eq!(playlist.nombre,nuevo_titulo);
    }

    #[test]
    fn test_eliminar_todas_las_canciones(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);
        let cancion=Cancion{titulo:"believer".to_string(),artista:"imagine dragons".to_string(),genero:Genero::Rock};

        playlist.agregar_cancion(cancion);
        playlist.eliminar_todas_las_canciones();

        assert!(playlist.lista_de_canciones.is_empty());
    }

    #[test]
    fn test_obtener_canciones_artista(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion1=Cancion{titulo:"thriller".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};
        let cancion2=Cancion{titulo:"billie jean".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};
        let cancion3=Cancion{titulo:"believer".to_string(),artista:"imagine dragons".to_string(),genero:Genero::Rock};
        playlist.agregar_cancion(cancion1);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);

        let canciones=playlist.obtener_las_canciones_de_un_determinado_artista("michael jackson".to_string());

        assert_eq!(canciones.len(),2);
    }

    #[test]
    fn test_obtener_canciones_artista_vector_vacio(){
        let nombre="playlist".to_string();
        let playlist=Playlist::new(nombre);

        let canciones=playlist.obtener_las_canciones_de_un_determinado_artista("michael jackson".to_string());

        assert!(canciones.is_empty());
    }

    #[test]
    fn test_obtener_canciones_genero(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion1=Cancion{titulo:"thriller".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};
        let cancion2=Cancion{titulo:"billie jean".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};
        let cancion3=Cancion{titulo:"believer".to_string(),artista:"imagine dragons".to_string(),genero:Genero::Rock};

        playlist.agregar_cancion(cancion1);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);

        let canciones=playlist.obtener_las_canciones_de_un_determinado_genero(Genero::Pop);

        assert_eq!(canciones.len(),2);
    }

    #[test]
    fn test_obtener_canciones_genero_vector_vacio(){
        let nombre="playlist".to_string();
        let playlist=Playlist::new(nombre);

        let canciones=playlist.obtener_las_canciones_de_un_determinado_genero(Genero::Pop);

        assert!(canciones.is_empty());
    }

    #[test]
    fn test_buscar_cancion_por_nombre(){
        let nombre="playlist".to_string();
        let mut playlist=Playlist::new(nombre);

        let cancion=Cancion{titulo:"billie jean".to_string(),artista:"michael jackson".to_string(),genero:Genero::Pop};
        playlist.agregar_cancion(cancion);
        let expected=Some(0);
        let resultado=playlist.buscar_cancion_por_nombre("billie jean");

        assert_eq!(resultado,expected);
    }

    #[test]
    fn test_buscar_cancion_por_nombre_vector_vacio(){
        let nombre="playlist".to_string();
        let playlist=Playlist::new(nombre);
        let expected=None;
        let resultado=playlist.buscar_cancion_por_nombre("billie jean");

        assert_eq!(resultado,expected);
    }
}