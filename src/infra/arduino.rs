use crate::domain::traits::{Locomotion, Sensors};
use arduino_hal::hal::port::*;
use arduino_hal::port::mode::{Analog, Floating, Input, Output, PwmOutput};
use arduino_hal::simple_pwm::{Timer0Pwm, Timer2Pwm};

// --- Estructura para Motores ---
pub struct NanoMotors {
    // Motor A (Izq)
    pub pin_a_pwm: Pin<PwmOutput<Timer2Pwm>, PD3>, // Pin 3
    pub pin_a_in1: Pin<Output, PB1>,               // Pin 9 (PB1 en Nano)
    pub pin_a_in2: Pin<Output, PB0>,               // Pin 8 (PB0 en Nano)

    // Motor B (Der)
    pub pin_b_pwm: Pin<PwmOutput<Timer0Pwm>, PD5>, // Pin 5
    pub pin_b_in1: Pin<Output, PB3>,               // Pin 11 (PB3)
    pub pin_b_in2: Pin<Output, PB4>,               // Pin 12 (PB4)
}

impl Locomotion for NanoMotors {
    fn set_speed(&mut self, speed: u8) {
        self.pin_a_pwm.set_duty(speed);
        self.pin_b_pwm.set_duty(speed);
        self.pin_a_pwm.enable();
        self.pin_b_pwm.enable();
    }

    /// Avanzar con ambos motores
    fn forward(&mut self) {
        self.pin_a_in1.set_high();
        self.pin_a_in2.set_low(); // Motor A
        self.pin_b_in1.set_high();
        self.pin_b_in2.set_low(); // Motor B
    }

    /// Retroceder con ambos motores
    fn backward(&mut self) {
        self.pin_a_in1.set_low();
        self.pin_a_in2.set_high();
        self.pin_b_in1.set_low();
        self.pin_b_in2.set_high();
    }

    /// Giro sobre su eje: Izq avanza, Der retrocede
    fn turn_right(&mut self) {
        self.pin_a_in1.set_high();
        self.pin_a_in2.set_low();
        self.pin_b_in1.set_low();
        self.pin_b_in2.set_high();
    }

    /// Giro sobre su eje: Izq retrocede, Der avanza
    fn turn_left(&mut self) {
        self.pin_a_in1.set_low();
        self.pin_a_in2.set_high();
        self.pin_b_in1.set_high();
        self.pin_b_in2.set_low();
    }

    fn stop(&mut self) {
        self.pin_a_in1.set_low();
        self.pin_a_in2.set_low();
        self.pin_b_in1.set_low();
        self.pin_b_in2.set_low();
        self.set_speed(0);
    }

    fn wait_ms(&mut self, ms: u32) {
        arduino_hal::delay_ms(ms);
    }
}

// --- Estructura para Sensores ---
pub struct NanoSensors {
    pub adc: arduino_hal::Adc,
    // Pines analógicos para línea
    pub pin_line_l: Pin<Analog, PC3>, // A3
    pub pin_line_c: Pin<Analog, PC4>, // A4
    pub pin_line_r: Pin<Analog, PC2>, // A2

    // Pines digitales para obstáculos
    pub pin_obs_f: Pin<Input<Floating>, PD2>, // D2
    pub pin_obs_r: Pin<Input<Floating>, PD4>, // D4
    pub pin_obs_l: Pin<Input<Floating>, PD6>, // D6

    pub threshold: u16,
}

impl Sensors for NanoSensors {
    fn line_detected_left(&mut self) -> bool {
        self.pin_line_l.analog_read(&mut self.adc) > self.threshold
    }
    fn line_detected_center(&mut self) -> bool {
        self.pin_line_c.analog_read(&mut self.adc) > self.threshold
    }
    fn line_detected_right(&mut self) -> bool {
        self.pin_line_r.analog_read(&mut self.adc) > self.threshold
    }

    fn obstacle_front(&mut self) -> bool {
        self.pin_obs_f.is_high()
    }
    fn obstacle_right(&mut self) -> bool {
        self.pin_obs_r.is_high()
    }
    fn obstacle_left(&mut self) -> bool {
        self.pin_obs_l.is_high()
    }
}
