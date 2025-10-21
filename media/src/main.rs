#[derive(Debug)]
enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    AudioBook {
        title: String,
    },
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        // }
        match self {
            Media::Book { title, author } => { format!("Book : {} {}", title, author) }
            Media::Movie { title, director } => { format!("Movie : {} {}", title, director) }
            Media::AudioBook { title } => { format!("AudioBook : {} ", title) }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        return Catalog { items: vec![] };
    }
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn main() {
    let mut c = Catalog::new();
    let book = Media::AudioBook { title: String::from("Heyy") };
    c.add(book);
    println!("{:#?}", c);
    println!("Hello, world!");
}
