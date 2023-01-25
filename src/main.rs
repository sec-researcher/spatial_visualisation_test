use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    window::{DoubleWindow, Window},
};
use std::error::Error;
mod img;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Matched,
    NotMatched,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(350, 400);
    wind.set_color(fltk::enums::Color::from_rgb(255, 255, 255));
    let mut frame1 = Frame::default().with_size(150, 150).center_x(&wind);
    let mut frame2 = Frame::default().with_size(150, 150).below_of(&frame1, 0);
    frame1.set_pos(0, 20);
    let mut flip: u8 = 0;
    update(&mut wind, &mut frame1, &mut frame2, &mut flip);

    let mut frame = Frame::default().with_size(100, 40);
    frame.set_pos(200, 150);

    frame.set_label_size(20);
    let mut matched = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("Match");
    let mut not_matched = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("Not match");
    wind.end();
    wind.make_resizable(false);
    wind.show();

    let (s, r) = app::channel::<Message>();
    matched.emit(s, Message::Matched);
    not_matched.emit(s, Message::NotMatched);

    let mut mode = 0;
    let default_color = matched.color();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Matched => {
                    let mut btn = matched.clone();
                    if mode == 0 {
                        if flip==1 {
                            btn.set_color(fltk::enums::Color::Green);
                            btn.set_label("Right")
                        } else {
                            btn.set_color(fltk::enums::Color::Red);
                            btn.set_label("Wrong")
                        }
                        mode = 1;
                    } else {
                        btn.set_color(default_color);
                        btn.set_label("Match");
                        mode = 0;
                        update(&mut wind, &mut frame1, &mut frame2, &mut flip);
                    }
                }
                Message::NotMatched => {
                    let mut btn = not_matched.clone();
                    if mode == 0 {
                        if flip==1 {
                            btn.set_color(fltk::enums::Color::Red);
                            btn.set_label("Worng")
                        } else {
                            btn.set_color(fltk::enums::Color::Green);
                            btn.set_label("Right")
                        }
                        mode = 1;
                    } else {
                        btn.set_color(default_color);
                        btn.set_label("Not match");
                        mode = 0;
                        update(&mut wind, &mut frame1, &mut frame2, &mut flip);
                    }
                }
            }
        }
    }
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

