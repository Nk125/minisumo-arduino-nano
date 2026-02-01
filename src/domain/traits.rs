/// Interfaz para mover el robot y controlar el tiempo
pub trait Locomotion {
    fn set_speed(&mut self, speed: u8);
    fn forward(&mut self);
    fn backward(&mut self);
    fn turn_right(&mut self);
    fn turn_left(&mut self);
    fn stop(&mut self);
    /// Abstracción del delay para no depender de librerías de hardware en la lógica
    fn wait_ms(&mut self, ms: u32);
}

/// Interfaz para leer el entorno
pub trait Sensors {
    // Retorna true si detecta línea (negro/blanco según config)
    fn line_detected_left(&mut self) -> bool;
    fn line_detected_center(&mut self) -> bool;
    fn line_detected_right(&mut self) -> bool;

    // Retorna true si detecta obstáculo
    fn obstacle_front(&mut self) -> bool;
    fn obstacle_right(&mut self) -> bool;
    fn obstacle_left(&mut self) -> bool;
}
