// Copyright © Zach Nielsen 2021

mod light_strip_5050;
use crate::light_strip_5050::LightStrip5050Manager;

#[macro_use] extern crate rocket;

use rocket::request::FromParam;
use rocket::State;

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
    fn set_brightness(&mut self, brightness: u16);
    fn set_color(&mut self, color: PresetColor);
    // Values are 0-255
    fn set_rgb(&mut self, red_val: u8, green_val: u8, blue_val: u8);
}

///////////////////////////////////////////////////////////////////////////////
// 5050 Handlers
///////////////////////////////////////////////////////////////////////////////
#[post("/set_color/<color>")]
fn preset_color_handler(manager: &State<LightStrip5050Manager>, color: PresetColor) {
    let mut lock = manager.lights.lock().unwrap();
    lock.set_color(color);
}
#[post("/set_color_rgb/<red_val>/<green_val>/<blue_val>")]
fn ad_hoc_color_handler(manager: &State<LightStrip5050Manager>, red_val: u8, green_val: u8, blue_val: u8) {
    let mut lock = manager.lights.lock().unwrap();
    lock.set_rgb(red_val, green_val, blue_val);
}
#[post("/set_duty_all/<brightness>")]
fn brightness_handler(manager: &State<LightStrip5050Manager>, brightness: u16) {
    let mut lock = manager.lights.lock().unwrap();
    lock.set_brightness(brightness);
}

#[launch]
fn rocket() -> _ {
    let light_manager = LightStrip5050Manager::new();
    light_manager.init();

    rocket::build()
        .manage(light_manager)
        .mount("/", routes![preset_color_handler])
        .mount("/", routes![ad_hoc_color_handler])
        .mount("/", routes![brightness_handler])
}

