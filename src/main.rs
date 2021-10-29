// Copyright © Zach Nielsen 2021

// TODO - trait for setting colors, different strips might work differently

#[macro_use] extern crate rocket;

use gpio::GpioOut;

use std::sync::{Arc, Mutex};
use std::thread;

const RED_PIN:   u16 = 23;
const GREEN_PIN: u16 = 23;
const BLUE_PIN:  u16 = 23;

// TODO - move to specific implementation of the LedStrip trait?
// PWM Config - 100ms period, update every 1ms
const PWM_PERIOD: u16 = 100;
const PWM_UPDATE: u16 = 1;
struct PwmDuty {
    pub red_duty: u16,
    pub green_duty: u16,
    pub blue_duty: u16,
}
impl PwmDuty {
    fn new() -> PwmDuty { PwmDuty{ red_duty: 0, green_duty: 0, blue_duty: 0 } }

    fn get_red_duty  (&self) -> u16 { self.red_duty }
    fn get_green_duty(&self) -> u16 { self.green_duty }
    fn get_blue_duty (&self) -> u16 { self.blue_duty }

    fn set_red_duty  (&mut self, val: u16) { self.red_duty = val }
    fn set_green_duty(&mut self, val: u16) { self.green_duty = val }
    fn set_blue_duty (&mut self, val: u16) { self.blue_duty = val }
}

enum PresetColor {
    Red,
    Green,
    Blue,
}

// #[post("/set_color/<color>")]
// fn preset_color_handler(color: PresetColor) {
// }

#[post("/set_color_rgb/<red_val>/<green_val>/<blue_val>")]
fn ad_hoc_color_handler(red_val: u16, green_val: u16, blue_val: u16) {
}

#[post("/set_duty_all/<brightness>")]
fn brightness_handler(brightness: u16) {
}

#[launch]
fn rocket() -> _ {
    let mut red_pin   = gpio::sysfs::SysFsGpioOutput::open(RED_PIN).unwrap();
    let mut green_pin = gpio::sysfs::SysFsGpioOutput::open(GREEN_PIN).unwrap();
    let mut blue_pin  = gpio::sysfs::SysFsGpioOutput::open(BLUE_PIN).unwrap();

    // TODO - move to a specific implementation of the LedStrip trait?
    // Launch a thread that reads from the info struct
    let multi_pwm = Arc::new(Mutex::new(PwmDuty::new()));
    let thread_pwm = Arc::clone(&multi_pwm);
    let mut counter: u16 = 0;
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(PWM_UPDATE as u64));

        let pwm = thread_pwm.lock().unwrap();
        let pin_pairs = vec![(&mut red_pin,   pwm.get_red_duty()),
                                 (&mut green_pin, pwm.get_green_duty()),
                                 (&mut blue_pin,  pwm.get_blue_duty())];

        // TODO - increment based on actual time elapsed, rather than nominal time
        counter = (counter + PWM_UPDATE) % PWM_PERIOD;

        for (pin, duty) in pin_pairs {
            if counter <= duty {
                // Turn on
                pin.set_high().unwrap();
            }
            else {
                // Turn off
                pin.set_low().unwrap();
            }
        }

        drop(pwm);
    });

    rocket::build()
        .manage(multi_pwm)
        .mount("/", routes![ad_hoc_color_handler])
        .mount("/", routes![brightness_handler])
}
