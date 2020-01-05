use std::io;

#[cfg(test)]
mod tests {
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
}

fn main() {
    let image = read_image(25, 6);
    let scores = score_image(&image);
    let best_layer = scores.iter().fold(
        [0; 10],
        |res, score| if score[0] > res[0] { *score } else { res },
    );
    println!("CHECKSUM: {}", best_layer[1] * best_layer[2]);
}

type Image = Vec<Layer>;
type Layer = Vec<Vec<u8>>;
type LayerScore = [u32; 10];
type ImageScore = Vec<LayerScore>;

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
