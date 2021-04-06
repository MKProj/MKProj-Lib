pub mod uniconv {
    //! This is used to convert units of various measurements
    //! All methods use the same parameters `(unit1: f32, mut unit2: f32)->f32`
    //! - unit1 represents the unit to be converted
    //! - unit2 represents the converted unit
    pub mod Temperature {
        /// Fahrenheit to Celsius degrees
        pub fn fahr_to_cels(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = (unit1 - 32.0) * (5.0 / 9.0);
            unit2
        }
        /// Fahrenheit to Kelvin degrees
        pub fn fahr_to_kelv(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = (5.0 / 9.0) * (unit1 - 32.0) + 273.0;
            unit2
        }
        /// Celsius to Fahrenheit degrees
        pub fn cels_to_fahr(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = (unit1 * 9.0 / 5.0) + 32.0;
            unit2
        }
        /// Celsius to Kelvin degrees
        pub fn cels_to_kelv(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 + 273.0;
            unit2
        }
        /// Kelvin to Fahrenheit degrees
        pub fn kelv_to_fahr(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = (9.0 / 5.0) * (unit1 - 273.0) + 32.0;
            unit2
        }
        /// Kelvin to Celsius degrees
        pub fn kelv_to_cels(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 - 273.0;
            unit2
        }
    }
}
