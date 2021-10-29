// Copyright © Zach Nielsen 2021

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
    pub duty: u16,
}

pub struct LightStrip5050 {
    multi_led_info: Arc<Mutex<HashMap<LedPin, PinPwmPair>>>,
}
impl LightStrip5050 {
    pub fn new() -> LightStrip5050 {
        let mut default_map = HashMap::new();
        default_map.insert(LedPin::Red,   PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(RED_PIN).unwrap(),   duty: 0 });
        default_map.insert(LedPin::Green, PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(GREEN_PIN).unwrap(), duty: 0 });
        default_map.insert(LedPin::Blue,  PinPwmPair{ pin: gpio::sysfs::SysFsGpioOutput::open(BLUE_PIN).unwrap(),  duty: 0 });

        LightStrip5050 {
            multi_led_info: Arc::new(Mutex::new(default_map)),
        }
    }
}
impl LedStrip for LightStrip5050 {
    fn set_brightness(&self, brightness: u16) {
    }

    fn set_color(&self, color: PresetColor) {
        // match color {
        //     PresetColor::Red:
        //     PresetColor::Green:
        //     PresetColor::Blue:
        // }
    }

    fn set_rgb(&self, red_val: u8, green_val: u8, blue_val: u8) {
    }

    fn init(&self) {
        // Launch a thread to manage PWM
        let mut thread_leds = Arc::clone(&self.multi_led_info);
        let mut counter: u16 = 0;
        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_millis(PWM_UPDATE as u64));
            let mut leds = thread_leds.lock().unwrap();

            // TODO - increment based on actual time elapsed, rather than nominal time waited
            counter = (counter + PWM_UPDATE) % PWM_PERIOD;

            for color in vec![LedPin::Red, LedPin::Green, LedPin::Blue].into_iter(){
                if counter <= leds[&color].duty {
                    // Turn on
                    leds.get_mut(&color).unwrap().pin.set_high().unwrap();
                }
                else {
                    // Turn off
                    leds.get_mut(&color).unwrap().pin.set_low().unwrap();
                }
            }

            drop(leds);
        });
    }
}
