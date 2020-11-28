use std::{
    collections::HashMap,
    fmt,
    io::{self},
};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));

    let image = SpaceImage::new(&input, 25, 6);

    let result = test_image(&image);
    println!("Test image data: {}", result);
    println!("{}", image);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
#[repr(u32)]
enum Color {
    Black,
    White,
    Transparent,
}

impl From<u32> for Color {
    fn from(val: u32) -> Self {
        match val {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            other => panic!("Invalid color value {}", other),
        }
    }
}

type ColorCount = HashMap<Color, u32>;

struct SpaceImage {
    layers: Vec<Vec<Color>>,
    width: u32,
}

impl fmt::Display for SpaceImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.flatten()
                .chunks(self.width as usize)
                .map(|row| {
                    row.iter()
                        .map(|&color| match color {
                            Color::White => "X",
                            Color::Black => " ",
                            _other => panic!("Unknown color"),
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl SpaceImage {
    fn new(pixels: &str, width: u32, height: u32) -> SpaceImage {
        let colors: Vec<Color> = pixels
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("Expected a digit").into())
            .collect();
        let layers: Vec<_> = colors
            .chunks_exact((width * height) as usize)
            .map(|c| c.to_vec())
            .collect();

        SpaceImage { layers, width }
    }

    fn colors_per_layer(&self) -> Vec<ColorCount> {
        self.layers
            .iter()
            .map(|layer| {
                let mut colors = HashMap::new();
                for &color in layer {
                    *colors.entry(color).or_insert(0) += 1;
                }
                colors
            })
            .collect()
    }

    fn flatten(&self) -> Vec<Color> {
        let num_pixels = self.layers[0].len();
        (0..num_pixels)
            .into_iter()
            .map(|index| {
                self.layers
                    .iter()
                    .map(|layer| layer[index])
                    .skip_while(|&color| color == Color::Transparent)
                    .next()
                    .expect("All locations must have a colored pixel")
            })
            .collect()
    }
}

fn test_image(image: &SpaceImage) -> u32 {
    let colors_per_layer = image.colors_per_layer();
    let layer_with_fewer_zeros = colors_per_layer
        .iter()
        .min_by_key(|&color_counts| color_counts.get(&Color::Black).copied().unwrap_or(0))
        .unwrap();

    let num_white = layer_with_fewer_zeros
        .get(&Color::White)
        .copied()
        .unwrap_or(0);
    let num_transparent = layer_with_fewer_zeros
        .get(&Color::Transparent)
        .copied()
        .unwrap_or(0);

    num_white * num_transparent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_colors() {
        let image = SpaceImage::new("0222112222120000", 2, 2);
        assert_eq!(
            image.flatten(),
            vec![Color::Black, Color::White, Color::White, Color::Black]
        );
    }
}
