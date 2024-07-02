use chrono::{DateTime, Utc};

const SECONDS_PER_SOL: f64 = 88775.244; // Length of a Martian sol in seconds
const SECONDS_PER_DAY: f64 = 86400.0; // Length of an Earth day in seconds
const MARS_SOL_OFFSET: f64 = 24000.0; // Approximate offset of Mars epoch from Unix epoch in seconds

fn main() {
    let earth_time: DateTime<Utc> = Utc::now();
    println!("Current Earth time: {}", earth_time);

    let mtc = calculate_mtc(earth_time);
    println!("Mars Time Coordinated (MTC): {}", mtc);

    let amt = calculate_amt(mtc.clone());
    println!("Areocentric Mean Time (AMT, LMST at 0°): {}", amt);

    let longitude = 285.9306; // Example longitude
    let lst = calculate_lst(mtc.clone(), longitude);
    println!("Local Solar Time (LST) at {:.4}°: {}", longitude, lst);
}

fn calculate_mtc(earth_time: DateTime<Utc>) -> String {
    let duration_since_unix_epoch = earth_time.timestamp() as f64;
    let duration_since_mars_epoch = duration_since_unix_epoch + MARS_SOL_OFFSET;
    let total_mars_seconds = duration_since_mars_epoch / SECONDS_PER_SOL;
    let sols = total_mars_seconds.floor() as u64;
    let remaining_seconds = (total_mars_seconds - sols as f64) * SECONDS_PER_SOL;

    let hours = (remaining_seconds / 3600.0).floor() as u32;
    let minutes = ((remaining_seconds % 3600.0) / 60.0).floor() as u32;
    let seconds = (remaining_seconds % 60.0).floor() as u32;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn calculate_amt(mtc: String) -> String {
    // AMT is the same as MTC since it is LMST at 0° longitude
    mtc
}

fn calculate_lst(mtc: String, longitude: f64) -> String {
    let mtc_parts: Vec<u32> = mtc.split(':').map(|s| s.parse().unwrap()).collect();
    let mtc_seconds = (mtc_parts[0] * 3600 + mtc_parts[1] * 60 + mtc_parts[2]) as f64;
    let longitude_hours = longitude / 360.0 * 24.0;
    let lst_seconds = (mtc_seconds + longitude_hours * 3600.0) % SECONDS_PER_SOL;
    let lst_hours = (lst_seconds / 3600.0).floor() as u32;
    let lst_minutes = ((lst_seconds % 3600.0) / 60.0).floor() as u32;
    let lst_seconds = (lst_seconds % 60.0).floor() as u32;

    format!("{:02}:{:02}:{:02}", lst_hours, lst_minutes, lst_seconds)
}
