// Copyright © Zach Nielsen 2021

// The 5050 manager works by setting a brightness ratio for each color,
// multiplied by the requested brightness.

use crate::PresetColor;
use crate::LedStrip;

use gpio::GpioOut;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

const RED_PIN:   u16 = 23;
const GREEN_PIN: u16 = 23;
const BLUE_PIN:  u16 = 23;

#[derive(Eq, Hash, PartialEq)]
enum LedPin {
    Red,
    Green,
    Blue
}

// PWM Config - 100ms period, update every 1ms
const PWM_PERIOD: u16 = 100;
const PWM_UPDATE: u16 = 1;
struct PinPwmPair {
    pin: gpio::sysfs::SysFsGpioOutput,
    pub ratio: f32,
}

pub struct LightStrip5050 {
    multi_led_info: HashMap<LedPin, PinPwmPair>,
    brightness: u16,
}
impl LightStrip5050 {
    pub fn new() -> LightStrip5050 {
        let mut default_map = HashMap::new();
        default_map.insert(LedPin::Red,   PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(RED_PIN).unwrap(),   ratio: 1.0 });
        default_map.insert(LedPin::Green, PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(GREEN_PIN).unwrap(), ratio: 1.0 });
        default_map.insert(LedPin::Blue,  PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(BLUE_PIN).unwrap(),  ratio: 1.0 });

        LightStrip5050 {
            multi_led_info: default_map,
            brightness: 0,
        }
    }
}
impl LedStrip for LightStrip5050 {
    fn set_brightness(&mut self, brightness: u16) {
        self.brightness = brightness;
    }

    fn set_color(&mut self, color: PresetColor) {
        self.multi_led_info.get_mut(&LedPin::Red).unwrap().ratio = 0.0;
        self.multi_led_info.get_mut(&LedPin::Green).unwrap().ratio = 0.0;
        self.multi_led_info.get_mut(&LedPin::Blue).unwrap().ratio = 0.0;
        match color {
            PresetColor::Red   => { self.multi_led_info.get_mut(&LedPin::Red).unwrap().ratio   = 1.0; }
            PresetColor::Green => { self.multi_led_info.get_mut(&LedPin::Green).unwrap().ratio = 1.0; }
            PresetColor::Blue  => { self.multi_led_info.get_mut(&LedPin::Blue).unwrap().ratio  = 1.0; }
        }
    }

    fn set_rgb(&mut self, red_val: u8, green_val: u8, blue_val: u8) {
        self.multi_led_info.get_mut(&LedPin::Red).unwrap().ratio   = red_val   as f32 / 255.0;
        self.multi_led_info.get_mut(&LedPin::Green).unwrap().ratio = green_val as f32 / 255.0;
        self.multi_led_info.get_mut(&LedPin::Blue).unwrap().ratio  = blue_val  as f32 / 255.0;
    }
}

pub struct LightStrip5050Manager {
    pub lights: Arc<Mutex<LightStrip5050>>
}
impl LightStrip5050Manager {
    pub fn new() -> LightStrip5050Manager { LightStrip5050Manager{ lights: Arc::new(Mutex::new(LightStrip5050::new())) } }

    pub fn init(&self) {
        // Launch a thread to manage PWM
        let thread_lights = Arc::clone(&self.lights);
        let mut counter: u16 = 0;
        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_millis(PWM_UPDATE as u64));
            let mut lights = thread_lights.lock().unwrap();

            // TODO - increment based on actual time elapsed, rather than nominal time waited
            counter = (counter + PWM_UPDATE) % PWM_PERIOD;

            for color in vec![LedPin::Red, LedPin::Green, LedPin::Blue].into_iter(){
                if counter <= lights.multi_led_info[&color].ratio as u16 * lights.brightness {
                    // Turn on
                    lights.multi_led_info.get_mut(&color).unwrap().pin.set_high().unwrap();
                }
                else {
                    // Turn off
                    lights.multi_led_info.get_mut(&color).unwrap().pin.set_low().unwrap();
                }
            }

            drop(lights);
        });
    }
}

