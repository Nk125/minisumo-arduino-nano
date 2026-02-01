use super::traits::{Locomotion, Sensors};

// Constantes de calibración
const SPEED_DEFAULT: u8 = 255;
const BACKUP_TIME_MS: u32 = 500;

pub struct Brain;

impl Brain {
    /// Esta función se llama en cada iteración del bucle principal
    pub fn step<L: Locomotion, S: Sensors>(locomotion: &mut L, sensors: &mut S) {
        // 1. Prioridad: Evitar salirse de la línea
        // Lógica de TCRT
        if sensors.line_detected_left()
            || sensors.line_detected_center()
            || sensors.line_detected_right()
        {
            locomotion.set_speed(SPEED_DEFAULT);
            locomotion.backward();
            locomotion.wait_ms(BACKUP_TIME_MS); // Delay de retroceso
            locomotion.stop();
            return; // Termina el ciclo aquí para priorizar seguridad
        }

        // 2. Prioridad: Combate / Navegación
        // Lógica de movimiento
        if sensors.obstacle_front() {
            locomotion.set_speed(SPEED_DEFAULT);
            locomotion.forward();
        } else if sensors.obstacle_right() {
            locomotion.set_speed(SPEED_DEFAULT);
            locomotion.turn_right();
        } else if sensors.obstacle_left() {
            locomotion.set_speed(SPEED_DEFAULT);
            locomotion.turn_left();
        } else {
            // Si no hay línea ni enemigos, patrullar
            locomotion.stop();
        }
    }
}
