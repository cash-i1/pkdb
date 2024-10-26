use std::fs;

use yaml_rust2::{YamlEmitter, YamlLoader};

#[derive(Debug)]
struct ImageName(String);

#[derive(Debug)]
struct Card {
    name: String,
    img: ImageName,
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
                img: ImageName(yaml["img"].as_str().unwrap().to_string()),
            });
        }
    }
    
    for card in cards {
        println!("{:?}", card);
    }

}
