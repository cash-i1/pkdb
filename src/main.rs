use std::fs;

use maud::html;
use yaml_rust2::{YamlEmitter, YamlLoader};

#[derive(Debug)]
// extention agnostic image name
struct ImageLocator(String);

// path of image relative to "/"
#[derive(Debug)]
struct ImagePath(String);

#[derive(Debug)]
struct Card {
    name: String,
    image_locator: ImageLocator,
    image_path: Option<ImagePath>,
}

fn main() {
    let mut cards: Vec<Card> = vec![];

    let cards_yaml = fs::read_dir("./data/cards").unwrap();
    for card in cards_yaml {
        if let Ok(card) = card {
            let contents = fs::read_to_string(card.path()).unwrap();
            let yaml = YamlLoader::load_from_str(&contents).unwrap()[0].clone();

            cards.push(Card {
                name: yaml["name"].as_str().unwrap().to_string(),
                image_locator: ImageLocator(yaml["img"].as_str().unwrap().to_string()),
                image_path: None,
            });
        }
    }

    let images = fs::read_dir("./data/img").unwrap();
    for image in images {
        if let Ok(image) = image {
            let path = image.path();
            let path = path.strip_prefix(".").unwrap();

            for card in &mut cards {
                let path_name = path
                    .with_extension("")
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                if path_name == card.image_locator.0 {
                    card.image_path = Some(ImagePath(path.to_string_lossy().to_string()))
                }
            }
        }
    }

    let html = html!(
        h1 { "pokemons" }
        hr {}
        @for card in &cards {
            div class="card" {
                h2 { (card.name) }
                @if let Some(image_path) = &card.image_path {
                    img src=(image_path.0) {}
                }
            }
        }
    );

    fs::write("./index.html", html.clone().into_string()).unwrap();

    println!("{:?}", html.into_string());

    for card in cards {
        println!("{:?}", card);
    }
}
