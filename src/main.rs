use fltk::{app, frame::{Frame}, button::Button,  prelude::*, window::{Window, DoubleWindow}};
use std::error::Error;
mod img;

fn main() -> Result<(), Box<dyn Error>> {
    //Defining window and frames
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::default().with_size(350, 400);
    wind.set_color(fltk::enums::Color::from_rgb(255, 255, 255));
    let mut frame1 = Frame::default().with_size(150, 150).center_x(&wind);
    let mut frame2 = Frame::default().with_size(150, 150).below_of(&frame1,0);
    frame1.set_pos(0,20);

    let mut flip:u8 = 0; //We use this var to track image is flipped or not



    wind.end();
    wind.make_resizable(false);
    wind.show();
    app.run()?;


    Ok(())
}

