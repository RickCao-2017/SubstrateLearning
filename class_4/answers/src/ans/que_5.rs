enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn duration(&self) -> u32;
}

impl Duration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 45,
        }
    }
}

pub fn show_lights() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("The red light lasts {} seconds", red_light.duration());
    println!("The yellow light lasts {} seconds", yellow_light.duration());
    println!("The green light lasts {} seconds", green_light.duration());
}
