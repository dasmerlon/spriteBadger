use color_eyre::eyre::{Ok, Result};
use image::{ImageReader, Rgba};
use serde::{Deserialize, Serialize};

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
const TRANSPARENT: Rgba<u8> = Rgba([0, 0, 0, 0]);

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];
    let output_file = &args[2];

    let img = ImageReader::open(file)?.decode()?.to_rgba8();

    let mut current_pixel_row = 0;
    let mut sprites: Vec<Vec<String>> = Vec::new();
    let mut current_sprites: Vec<Vec<String>> = Vec::new();

    for row in img.rows() {
        for (index, pixel) in row.enumerate() {
            if pixel == &TRANSPARENT {
                continue;
            }

            let sprite_index = index / 44;
            let sprite = current_sprites.get_mut(sprite_index);
            let sprite = match sprite {
                Some(sprite) => sprite,
                None => {
                    current_sprites.push(Vec::new());
                    current_sprites.get_mut(sprite_index).unwrap()
                }
            };

            let sprite_row = match sprite.get_mut(current_pixel_row) {
                Some(row) => row,
                None => {
                    sprite.push(String::new());
                    sprite.get_mut(current_pixel_row).unwrap()
                }
            };

            if pixel == &BLACK {
                sprite_row.push('_');
            } else {
                sprite_row.push('X');
            }
        }
        current_pixel_row += 1;

        if current_pixel_row == 11 {
            sprites.append(&mut current_sprites);
            current_pixel_row = 0;
        }
    }

    let mut animation = vec![];
    for _ in 0..11 {
        animation.push(String::new())
    }

    for sprite in sprites {
        for (index, line) in sprite.iter().enumerate() {
            let animation_line = animation.get_mut(index).unwrap();
            if !animation_line.is_empty() {
                animation_line.push_str("____");
            }
            animation_line.push_str(line);
        }
    }

    let animation_string = animation.join("\n");

    let root = Root {
        message: vec![Message {
            speed: 6,
            mode: "fast".to_string(),
            bitstring: animation_string,
        }],
    };

    let toml_output = toml::to_string_pretty(&root)?;

    std::fs::write(output_file, toml_output)?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Root {
    message: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    speed: u8,
    mode: String,
    bitstring: String,
}
