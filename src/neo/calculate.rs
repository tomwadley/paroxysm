use neo::Neo;
use neo::chrono::prelude::*;
use std::f64::consts::PI;

// https://en.wikipedia.org/wiki/Standard_asteroid_physical_characteristics#Density
const DENSITY_KG_KM3: f64 = 2_000_000_000_000.0;
const ALBEDO: f64 = 0.2;

impl Neo {
    fn momentum_kg_km_s(&self) -> f64 {
        mass_of_neo_kg(diameter_of_neo_km(self.magnitude)) * self.velocity
    }

    fn seconds_until_ca(&self, now: DateTime<Utc>) -> i64 {
        self.ca_time.signed_duration_since(now).num_seconds()
    }
}

fn diameter_of_neo_km(magnitude: f64) -> f64 {
    (1329.0 / ALBEDO.sqrt()) * 10.0f64.powf(-0.2 * magnitude)
}

fn mass_of_neo_kg(diameter_km: f64) -> f64 {
    // asteroids aren't really spherical but its not *too* far off
    volume_of_sphere(diameter_km / 2.0) * DENSITY_KG_KM3
}

fn volume_of_sphere(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius.powi(3)
}

#[cfg(test)]
mod tests {
    extern crate time;
    use neo::chrono::prelude::*;
    use self::time::Duration;

    #[test]
    fn calculates_diameter() {
        // Calculated using http://www.physics.sfasu.edu/astro/asteroids/sizemagnitude.html
        assert_eq!(format!("{:.3}", super::diameter_of_neo_km(24.754)), "0.033");
    }

    #[test]
    fn calculates_volume() {
        assert_eq!(format!("{:.2}", super::volume_of_sphere(10.0)), "4188.79");
        assert_eq!(format!("{:.12}", super::volume_of_sphere(0.033 / 2.0)), "0.000018816569");
    }

    #[test]
    fn calculates_mass() {
        assert_eq!(format!("{:.0}", super::mass_of_neo_kg(0.033)), "37633138");
    }

    #[test]
    fn calculates_momentum() {
        let neo = create_neo(Utc::now());
        assert_eq!(format!("{:.0}", neo.momentum_kg_km_s()), "772126687");
    }

    #[test]
    fn calculates_seconds() {
        let now = Utc::now();
        let neo = create_neo(now);
        assert_eq!(neo.seconds_until_ca(now), 100);
    }

    fn create_neo(now: DateTime<Utc>) -> super::Neo {
        super::Neo {
            designation: String::from("test"),
            velocity: 20.0,
            magnitude: 24.754,
            ca_time: now + Duration::seconds(100)
        }
    }
}