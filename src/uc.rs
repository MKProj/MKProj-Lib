pub mod uniconv {
    //! This is used to convert units of various measurements  
    //!
    //! All methods use the same parameters `(unit1: f32, mut unit2: f32)->f32`  
    //!
    //! - unit1 represents the unit to be converted
    //! - unit2 represents the converted unit
    // Ex. function
    // pub fn (unit1: f32, mut unit2: f32) -> f32{}
    pub mod Temperature {
        //! This is used to convert temperatures
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
    pub mod Speed {
        //! This is used to convert speeds
        /// Metres per Second to Kilometre per Hour
        pub fn mps_to_kph(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 3.6;
            unit2
        }
        /// Kilometre per Hour to Metres per Second
        pub fn kph_to_mps(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 3.6;
            unit2
        }
        /// Metres per Second to Knots
        pub fn mps_to_knots(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 1.944;
            unit2
        }
        /// Kilometre per Hour to Knots
        pub fn kph_to_knots(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 1.852;
            unit2
        }
    }
    pub mod Length {
        //! This is used to convert lengths  
        //!
        //! **Only metres will be used for imperial unit conversions!**
        /// Metres to Inches
        pub fn m_to_inch(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 39.37;
            unit2
        }
        /// Metres to Feet
        pub fn m_to_foot(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 3.281;
            unit2
        }
        /// Metres to Yards
        pub fn m_to_yard(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 1.094;
            unit2
        }
        /// Metres to Miles
        pub fn m_to_mile(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 1609.0;
            unit2
        }
        /// Inches to Metres
        pub fn inch_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 39.37;
            unit2
        }
        /// Feet to Metres
        pub fn foot_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 3.281;
            unit2
        }
        /// Yards to Metres
        pub fn yard_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 1.094;
            unit2
        }
        /// Miles to Metres
        pub fn mile_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 1609.0;
            unit2
        }
        /// Milimetres to Metres
        pub fn mm_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 1000.0;
            unit2
        }
        /// Centimetres to Metres
        pub fn cm_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 / 100.0;
            unit2
        }
        /// Kilometres to Metres
        pub fn km_to_m(unit1: f32, mut unit2: f32) -> f32 {
            unit2 = unit1 * 1000.0;
            unit2
        }
    }
}
