// Socket

struct Socket {
    _description: String,
    _status: bool,
    _power_consumption: u32,
}

impl Socket {
    fn new(_description: String, _status: bool) -> Self {
        Self {
            _description,
            _status,
            _power_consumption: 0,
        }
    }

    // description

    fn _get_description(&self) -> String {
        todo!()
    }

    // status

    fn _get_status() -> bool {
        todo!()
    }

    fn _set_status(_new_status: bool) {
        todo!()
    }

    fn _turn_on() {
        Self::_set_status(true)
    }

    fn _turn_off() {
        Self::_set_status(false)
    }

    // power consumption

    fn _get_power_consumption(&self) -> u32 {
        todo!()
    }
}

// Thermometer

struct Thermometer {
    _temperature: f32,
}

impl Thermometer {
    fn new() -> Self {
        Self { _temperature: 0.0 }
    }

    fn _get_temperature(&self) -> f32 {
        todo!()
    }
}

//

fn main() {
    let _thermometer = Thermometer::new();
    let _socket = Socket::new("My smart socket".to_string(), false);
}
