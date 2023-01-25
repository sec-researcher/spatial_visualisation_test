use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::{DoubleWindow, Window},
};
use std::error::Error;
mod img;

fn main() -> Result<(), Box<dyn Error>> {
    //Defining window and frames
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(350, 400);
    wind.set_color(fltk::enums::Color::from_rgb(255, 255, 255));
    let mut frame1 = Frame::default().with_size(150, 150).center_x(&wind);
    let mut frame2 = Frame::default().with_size(150, 150).below_of(&frame1, 0);
    frame1.set_pos(0, 20);

    let mut flip: u8 = 0; //We use this var to track image is flipped or not

    wind.end();
    wind.make_resizable(false);
    wind.show();
    app.run()?;

    Ok(())
}

fn update(wind: &mut DoubleWindow, frame1: &mut Frame, frame2: &mut Frame, flip: &mut u8) {
    wind.set_color(fltk::enums::Color::from_rgb(255, 255, 255));
    let img1;
    let img = img::new("R");
    unsafe {
        img1 = fltk::image::RgbImage::from_data(
            &img.0 .2,
            img.0 .0 as i32,
            img.0 .1 as i32,
            fltk::enums::ColorDepth::Rgba8,
        )
        .unwrap();
    }
    let img2;
    unsafe {
        img2 = fltk::image::RgbImage::from_data(
            &img.1 .2,
            img.1 .0 as i32,
            img.1 .1 as i32,
            fltk::enums::ColorDepth::Rgba8,
        )
        .unwrap();
    }
    frame1.set_image(Some(img1));
    frame1.redraw_label();

    println!("image1 height: {}", img.0 .1);
    frame2.set_pos(0, frame1.y() + frame1.height() as i32 + 20);
    frame2.set_image(Some(img2));
    frame2.redraw_label();
    println!("frame2 y: {}", frame2.y());
    *flip = img.2 as u8;
}

