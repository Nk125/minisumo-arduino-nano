#![no_std]
#![no_main]

mod domain;
mod infra;

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm, Timer2Pwm};
use domain::logic::Brain;
use infra::arduino::{NanoMotors, NanoSensors};
use panic_halt as _;

const UMBRAL_SENSOR_LINEA: u16 = 500;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // El reloj TC0 debe ser usado para el pin digital 5/6.
    // https://rahix.github.io/avr-hal/arduino_hal/simple_pwm/struct.Timer0Pwm.html
    let mut d5_pin_timer = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // El reloj TC2 debe ser usado para el pin digital 3/11.
    // https://rahix.github.io/avr-hal/arduino_hal/simple_pwm/struct.Timer2Pwm.html
    let mut d3_pin_timer = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    // Inicializar ADC
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // Configurar Motores (Instanciar struct de Infraestructura)
    let mut motors = NanoMotors {
        pin_a_pwm: pins.d3.into_output().into_pwm(&mut d3_pin_timer),
        pin_a_in1: pins.d9.into_output(),
        pin_a_in2: pins.d8.into_output(),
        pin_b_pwm: pins.d5.into_output().into_pwm(&mut d5_pin_timer),
        pin_b_in1: pins.d11.into_output(),
        pin_b_in2: pins.d12.into_output(),
    };

    // Activar driver TB6612FNG (pin standby)
    pins.d10.into_output_high();

    // Configurar Sensores
    // Inicializar las entradas analógicas primero usando `adc`, luego mover `adc` dentro de la struct
    let pin_line_l = pins.a3.into_analog_input(&mut adc);
    let pin_line_c = pins.a4.into_analog_input(&mut adc);
    let pin_line_r = pins.a2.into_analog_input(&mut adc);

    let mut sensors = NanoSensors {
        adc,
        pin_line_l,
        pin_line_c,
        pin_line_r,
        pin_obs_f: pins.d2,
        pin_obs_r: pins.d4,
        pin_obs_l: pins.d6,
        threshold: UMBRAL_SENSOR_LINEA,
    };

    // Bucle Principal
    loop {
        Brain::step(&mut motors, &mut sensors);

        // Pequeña pausa para estabilidad (TIEMPO_RESPUESTA)
        arduino_hal::delay_ms(50);
    }
}
