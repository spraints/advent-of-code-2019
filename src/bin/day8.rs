use std::io;

#[cfg(test)]
mod tests {
    use super::OutputPixel::*;
    use super::*;

    #[test]
    fn test_parse_image() {
        let image = parse_image("123456789012", 3, 2);
        assert_eq!(
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]],
            ],
            image
        );
        assert_eq!(
            vec![
                [0, 1, 1, 1, 1, 1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0, 0, 0, 1, 1, 1],
            ],
            score_image(&image)
        );
    }

    #[test]
    fn test_render() {
        let image = parse_image("0222112222120000", 2, 2);
        assert_eq!(
            vec![vec![Black, White], vec![White, Black],],
            render(&image)
        );
    }
}

fn main() {
    let verbose = false;

    let image = read_image(25, 6);
    if verbose {
        dump(&image);
    }

    let scores = score_image(&image);
    let best_layer = scores.iter().fold(
        [99999; 10],
        |res, score| if score[0] < res[0] { *score } else { res },
    );
    println!("CHECKSUM: {}", best_layer[1] * best_layer[2]);
}

fn dump(image: &Image) {
    for layer in image {
        println!("------");
        for row in layer {
            println!("{:?}", row);
        }
    }
}

type Image = Vec<Layer>;
type Layer = Vec<Vec<u8>>;

type ColorizedImage = Vec<ColorizedLayer>;
type ColorizedLayer = Vec<Vec<InputPixel>>;
#[derive(Debug)]
enum InputPixel {
    Color(OutputPixel),
    Transparent,
}

type Rendered = Vec<Vec<OutputPixel>>;
#[derive(Clone, Copy, Debug, PartialEq)]
enum OutputPixel {
    Black, // 0
    White, // 1
}

type ImageScore = Vec<LayerScore>;
type LayerScore = [u32; 10];

fn render(image: &Image) -> Rendered {
    let (width, height, _) = get_dims(&image);
    let image = colorize(image);
    let mut res = vec![];
    for i in 0..height {
        let mut row = vec![];
        for j in 0..width {
            let mut output_color = OutputPixel::Black;
            for layer in image.iter().rev() {
                match layer[i][j] {
                    InputPixel::Transparent => (),
                    InputPixel::Color(color) => output_color = color,
                };
            }
            row.push(output_color);
        }
        res.push(row);
    }
    res
}

fn colorize(image: &Image) -> ColorizedImage {
    image.iter().map(|layer| colorize_layer(layer)).collect()
}

fn colorize_layer(layer: &Layer) -> ColorizedLayer {
    layer
        .iter()
        .map(|row| row.iter().map(|cell| decode_pixel(cell)).collect())
        .collect()
}

fn decode_pixel(enc: &u8) -> InputPixel {
    match enc {
        0 => InputPixel::Color(OutputPixel::Black),
        1 => InputPixel::Color(OutputPixel::White),
        _ => InputPixel::Transparent,
    }
}

// Returns (width, height, depth)
// width = number of columns
// height = number of rows
// depth = number of layers
fn get_dims(image: &Image) -> (usize, usize, usize) {
    let depth = image.len();
    let height = image[0].len();
    let width = image[0][0].len();
    (width, height, depth)
}

fn score_image(image: &Image) -> ImageScore {
    image.iter().map(|layer| score_layer(layer)).collect()
}

fn score_layer(layer: &Layer) -> LayerScore {
    let mut counts = [0; 10];
    for row in layer {
        for cell in row {
            counts[*cell as usize] += 1;
        }
    }
    counts
}

fn parse_image(input: &str, width: usize, height: usize) -> Image {
    let mut res = vec![];
    let mut input = input.chars().map(|c| c.to_digit(10).unwrap() as u8);

    while let Some(val) = input.next() {
        res.push(build_layer(val, &mut input, width, height));
    }

    res
}

fn build_layer(val: u8, input: &mut dyn Iterator<Item = u8>, width: usize, height: usize) -> Layer {
    let mut layer = vec![];
    for i in 0..height {
        let mut row = vec![];
        for j in 0..width {
            if i == 0 && j == 0 {
                row.push(val);
            } else {
                row.push(input.next().unwrap());
            }
        }
        layer.push(row);
    }
    layer
}

fn read_image(width: usize, height: usize) -> Image {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading program from STDIN");
    parse_image(line.trim(), width, height)
}
