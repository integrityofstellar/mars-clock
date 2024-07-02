use chrono::{DateTime, Utc};

const MARS_YEAR_LENGTH: f64 = 668.5991; // Martian year in Earth days
const MARS_SOL_LENGTH: f64 = 88775.244; // Martian sol in seconds
const MARS_EPOCH: f64 = 2451549.5; // J2000 epoch (January 1, 2000, 12:00 UTC) in Julian days
const EARTH_SECONDS_PER_DAY: f64 = 86400.0;

pub struct MarsTime {
    amt: f64,
    ls: f64,
}

pub fn earth_time_to_mars_time(earth_time: DateTime<Utc>) -> MarsTime {
    let julian_date = earth_time_to_julian_date(earth_time);
    let mars_sol = (julian_date - MARS_EPOCH) / (MARS_SOL_LENGTH / EARTH_SECONDS_PER_DAY);

    let amt = (mars_sol.fract() * 24.0) % 24.0;
    let ls = calculate_solar_longitude(julian_date);

    MarsTime { amt, ls }
}

fn earth_time_to_julian_date(earth_time: DateTime<Utc>) -> f64 {
    2440587.5 + (earth_time.timestamp() as f64) / EARTH_SECONDS_PER_DAY
}

fn calculate_solar_longitude(jd: f64) -> f64 {
    // Constants for Mars orbit
    let m_alpha: f64 = 19.3870;
    let m_phi: f64 = 0.089920;
    let m_epsilon: f64 = 0.42184;
    let m_tau: f64 = 1.90258;
    let m_lm: f64 = 0.01680;

    let t = (jd - 2451545.0) / 36525.0;
    let m = (19.3870 + 0.52402075 * t).to_radians();

    let alpha = (m_alpha + m_phi * (jd - MARS_EPOCH)).to_radians();
    let pbs = m_epsilon * (alpha + m_tau - m).sin();

    let ls = (alpha + 2.0 * m_epsilon * (alpha - m).sin() - pbs + std::f64::consts::PI)
        % (2.0 * std::f64::consts::PI);
    ls.to_degrees()
}

pub fn format_mars_time(mars_time: &MarsTime) -> String {
    format!(
        "AMT (LMST at 0°): {:02}:{:02}:{:02}\nLs{:.2}°",
        mars_time.amt as u32,
        ((mars_time.amt.fract() * 60.0) as u32) % 60,
        ((mars_time.amt.fract() * 3600.0) as u32) % 60,
        mars_time.ls
    )
}

fn main() {
    let earth_time = Utc::now();
    let mars_time = earth_time_to_mars_time(earth_time);
    println!("Current Earth time: {}", earth_time);
    println!("Current Mars time:\n{}", format_mars_time(&mars_time));
}
