/*8- Defina la estructura Cancion con campos para el título, el artista y el género. El género puede ser rock, pop, rap, jazz, otros.
 Luego modele una playlist. La playlist está compuesta por una lista de canciones y un nombre, y se
  permiten hacer las siguientes acciones sobre ella:

agregar canción.

eliminar canción.

mover canción // mueve la canción a una determinada posición de la playlist.

buscar canción por nombre.

obtener las canciones de un determinado género.

obtener las canciones de un determinado artista.

modificar título de la playlist.

eliminar todas las canciones. */
#[derive(Clone, Debug, PartialEq)]

enum Genres {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

struct Playlist {
    name: String,
    songs_list: Vec<Song>,
}
impl Playlist {
    fn delete_playlist(&mut self) {
        self.songs_list.clear();
    }
    fn modify_playlist_title(&mut self, title: String) {
        self.name = title;
    }
    fn get_genre(&self, genre: Genres) -> Option<Vec<Song>> {
        let songs: Vec<Song> = self
            .songs_list
            .iter()
            .filter(|song| song.genre == genre)
            .cloned()
            .collect();
        if !songs.is_empty() { Some(songs) } else { None }
    }
    fn get_artist(&self, artist: String) -> Vec<Song> {
        self.songs_list
            .iter()
            .filter(|song| song.artist == artist)
            .cloned()
            .collect()
    }
    fn find_by_title(&self, title: String) -> Option<usize> {
        self.songs_list.iter().position(|song| song.title == title)
    }
    fn order_left_right(vec: &mut Vec<Song>, position: usize, index: usize) {
        let mut placeholder: Song;
        placeholder = vec[position].clone();
        for i in (index + 1..=position).rev() {
            vec[i] = vec[i - 1].clone();
        }
        vec[index] = placeholder;
    }
    fn order_right_left(vec: &mut Vec<Song>, position: usize, index: usize) {
        let mut placeholder: Song;
        placeholder = vec[position].clone();
        for i in position..index {
            vec[i] = vec[i + 1].clone();
        }
        vec[index] = placeholder;
    }
    fn mover_cancion(&mut self, song: &Song, index: usize) {
        if !self.songs_list.is_empty() {
            if index >= 0 && index <= self.songs_list.len() {
                let mut song_founded: Option<usize> = None;
                let mut order: &str = "";
                let position: usize;
                // searching the song and declarating the order of moving
                for i in 0..self.songs_list.len() {
                    if self.songs_list[i].artist == song.artist
                        && self.songs_list[i].title == song.title
                    {
                        song_founded = Some(i);
                        break;
                    }
                }
                if let Some(data) = song_founded {
                    if data > index {
                        order = "left-right";
                    } else {
                        order = "right-left";
                    }
                    if data == index {
                        return;
                    }
                    let position = data;

                    // implement algortihmic order
                    match order {
                        "left-right" => {
                            Self::order_left_right(&mut self.songs_list, position, index)
                        }
                        "right-left" => {
                            Self::order_right_left(&mut self.songs_list, position, index)
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn agregar_cancion(&mut self, song: Song) {
        self.songs_list.push(song);
    }
    fn eliminar_cancion(&mut self, song: Song) {
        if !self.songs_list.is_empty() {
            let mut pos: Option<usize> = None;
            for i in 0..self.songs_list.len() {
                if self.songs_list[i].artist == song.artist
                    && self.songs_list[i].title == song.title
                {
                    pos = Some(i);
                    break;
                }
            }
            if let Some(index) = pos {
                self.songs_list.remove(index);
            }
        }
    }
}
#[derive(Clone)]
struct Song {
    title: String,
    artist: String,
    genre: Genres,
}
fn main() {
    let mut playlist = Playlist {
        name: String::from("Mi Playlist"),
        songs_list: Vec::new(),
    };

    let song1 = Song {
        title: String::from("Numb"),
        artist: String::from("Linkin Park"),
        genre: Genres::Rock,
    };

    let song2 = Song {
        title: String::from("Lose Yourself"),
        artist: String::from("Eminem"),
        genre: Genres::Rap,
    };

    let song3 = Song {
        title: String::from("Billie Jean"),
        artist: String::from("Michael Jackson"),
        genre: Genres::Pop,
    };

    let song4 = Song {
        title: String::from("Smooth Criminal"),
        artist: String::from("Michael Jackson"),
        genre: Genres::Pop,
    };

    // agregar canciones
    playlist.agregar_cancion(song1.clone());
    playlist.agregar_cancion(song2.clone());
    playlist.agregar_cancion(song3.clone());
    playlist.agregar_cancion(song4.clone());

    println!("Playlist inicial: {}", playlist.name);
    println!("Cantidad de canciones: {}", playlist.songs_list.len());

    // buscar por título
    if let Some(pos) = playlist.find_by_title(String::from("Numb")) {
        println!("Numb está en la posición {}", pos);
    }

    // obtener por género
    if let Some(rap_songs) = playlist.get_genre(Genres::Rap) {
        println!("Canciones Rap: {:?}", rap_songs.len());
    }

    // obtener por artista
    let mj_songs = playlist.get_artist(String::from("Michael Jackson"));
    println!("Canciones de Michael Jackson: {}", mj_songs.len());

    // mover canción
    playlist.mover_cancion(&song2, 0);

    println!("Después de mover canción:");
    for song in &playlist.songs_list {
        println!("{} - {}", song.artist, song.title);
    }

    // eliminar canción
    playlist.eliminar_cancion(song3);

    println!("Después de eliminar una canción:");
    for song in &playlist.songs_list {
        println!("{} - {}", song.artist, song.title);
    }

    // modificar nombre playlist
    playlist.modify_playlist_title(String::from("Playlist Nueva"));
    println!("Nuevo nombre: {}", playlist.name);

    // borrar playlist
    playlist.delete_playlist();
    println!("Playlist vacía: {}", playlist.songs_list.len());
}
