use std::error::Error;
use std::path;

pub struct Config {
    pub acpi_path: path::PathBuf,
    pub show_battery: bool,
    pub show_ac_adapter: bool,
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    if cfg.show_battery {
        let batteries: Vec<acpi_client::BatteryInfo> =
            match acpi_client::get_battery_info(&cfg.acpi_path.join("power_supply")) {
                Ok(bat) => bat,
                Err(e) => {
                    eprintln!("Application error: {}", e);
                    std::process::exit(1);
                }
            };
        for bat in batteries {
            display_battery_info(&bat);
        }
    }
    if cfg.show_ac_adapter {
        let adapters: Vec<acpi_client::ACAdapterInfo> =
            match acpi_client::get_ac_adapter_info(&cfg.acpi_path.join("power_supply")) {
                Ok(ac) => ac,
                Err(e) => {
                    eprintln!("Application error: {}", e);
                    std::process::exit(1);
                }
            };
        for ac in adapters {
            display_ac_adapter_info(&ac);
        }
    }
    Ok(())
}

fn display_battery_info(bat: &acpi_client::BatteryInfo) {
    let state = match &bat.state {
        acpi_client::ChargingState::Charging => "Charging",
        acpi_client::ChargingState::Discharging => "Discharging",
        acpi_client::ChargingState::Full => "Full",
    };
    let mut seconds = bat.time_remaining.as_secs();
    let hours = seconds / 3600;
    seconds = seconds - hours * 3600;
    let minutes = seconds / 60;
    seconds = seconds - minutes * 60;
    let not_full_string = format!(", {:02}:{:02}:{:02}", hours, minutes, seconds);
    let charge_time_string = match &bat.state {
        acpi_client::ChargingState::Charging => {
            if bat.present_rate > 0 {
                format!("{} {}", not_full_string, "until charged")
            } else {
                format!(", charging at zero rate")
            }
        }
        acpi_client::ChargingState::Discharging => format!("{} {}", not_full_string, "remaining"),
        _ => String::from(""),
    };
    println!(
        "{}: {}, {:.1}%{}",
        &bat.name, state, bat.percentage, charge_time_string
    );
}

fn display_ac_adapter_info(ac: &acpi_client::ACAdapterInfo) {
    let status_str = match ac.status {
        acpi_client::Status::Online => "online",
        acpi_client::Status::Offline => "offline",
    };
    println!("{}: {}", &ac.name, status_str);
}
