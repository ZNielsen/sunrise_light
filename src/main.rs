// Copyright © Zach Nielsen 2021

// TODO - add readme explaining that this is supposed to hook into Shortcuts

mod light_strip_5050;
use crate::light_strip_5050::LightStrip5050;

#[macro_use] extern crate rocket;

use rocket::request::FromParam;
use rocket::State;

use gpio::GpioOut;

pub enum PresetColor {
    Red,
    Green,
    Blue,
}
impl<'r> FromParam<'r> for PresetColor {
    type Error = &'r str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "red"   => Ok(PresetColor::Red),
            "green" => Ok(PresetColor::Green),
            "blue"  => Ok(PresetColor::Blue),
            _ => Err(param),
        }
    }
}

pub trait LedStrip {
    fn set_brightness(&self, brightness: u16);
    fn set_color(&self, color: PresetColor);
    // Values are 0-255
    fn set_rgb(&self, red_val: u8, green_val: u8, blue_val: u8);
    fn init(&self);
}
// pub struct LightStrip<T: LedStrip> (pub T);

// #[post("/set_color/<color>")]
// fn preset_color_handler(config: &State<LightStrip>, color: PresetColor) {

// }

// #[post("/set_color_rgb/<red_val>/<green_val>/<blue_val>")]
// fn ad_hoc_color_handler(config: &State<LightStrip>, red_val: u16, green_val: u16, blue_val: u16) {
// }

// #[post("/set_duty_all/<brightness>")]
// fn brightness_handler(config: &State<LightStrip>, brightness: u16) {
// }

#[launch]
fn rocket() -> _ {
    let lights = LightStrip5050::new();
    lights.init();

    rocket::build()
        // .manage(LightStrip(lights))
        // .mount("/", routes![preset_color_handler])
        // .mount("/", routes![ad_hoc_color_handler])
        // .mount("/", routes![brightness_handler])
}

