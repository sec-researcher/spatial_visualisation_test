use image::{DynamicImage, EncodableLayout, ImageBuffer, Rgba};
use rand::{rngs::ThreadRng, Rng};
use rusttype::{point, Font, Scale};

pub fn new(text: &str) -> ((u32, u32, Vec<u8>), (u32, u32, Vec<u8>), u32) {
    // Load the font
    let font_data = include_bytes!("../font.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");
    // The font size to use
    let scale = Scale::uniform(220.0);
    let colour = (0, 0, 0);
    let v_metrics = font.v_metrics(scale);
    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, 0.0 + v_metrics.ascent))
        .collect();
    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };
    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 20, glyphs_height + 0).to_rgba8();
    image.fill(255);
    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                )
            });
        }
    }
    for pix in image.pixels_mut() {
        if (pix[0] == 0 && pix[3] > 90) == false {
            pix[0] = 255;
            pix[1] = 255;
            pix[2] = 255;
            pix[3] = 255;
        }
    }
    let mut rng = rand::thread_rng();
    let mut last_rotate = None;
    let mut last_flip = None;
    let temp_image = rotate(&image, &mut rng, &mut last_rotate, &mut last_flip);
    let image2 = rotate(&image, &mut rng, &mut last_rotate, &mut last_flip);
    image = temp_image.0;
    (
        (image.width(), image.height(), image.as_bytes().to_vec()),
        (
            image2.0.width(),
            image2.0.height(),
            image2.0.as_bytes().to_vec(),
        ),
        (temp_image.1 == image2.1) as u32,
    )
}

fn rotate(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    rng: &mut ThreadRng,
    //flip: &mut i32,
    //just_rotate: Option<u8>,
    last_rotate: &mut Option<i32>,
    last_flip: &mut Option<i32>,
) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, u32) {
    let flip;
    
    let mut image2;
    let mut rotate;

    if last_flip.is_none() {
        flip = rng.gen_range(0..=2);
        match flip {
            0 => image2 = image::imageops::flip_vertical(&*image),
            1 => image2 = image::imageops::flip_horizontal(&*image),
            _ => image2 = image.clone(),
        }
        *last_flip=Some(flip);
    }
    else {
        image2 = image.clone();
        flip=2; //Actually prevent flipping
    }

    println!("flip: {}", flip);

    if last_rotate.is_some() {
        loop {
            rotate = rng.gen_range(0..3);
            if rotate != *last_rotate.as_ref().unwrap() {
                break;
            }
        }
    } else {
        rotate = rng.gen_range(0..3);
        *last_rotate = Some(rotate);        
    }

    println!("rotate: {}", rotate);
    match rotate {
        0 => image2 = image::imageops::rotate90(&image2),
        1 => image2 = image::imageops::rotate180(&image2),
        2 => image2 = image::imageops::rotate270(&image2),
        _ => (),
    }

    (image2, flip as u32)
}
