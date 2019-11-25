use std::path;

fn main() -> std::io::Result<()> {
    let power_supply_path = path::Path::new("/sys/class/power_supply");
    let batteries: Vec<acpi_client::BatteryInfo> =
        match acpi_client::get_battery_info(&power_supply_path) {
            Ok(bat) => bat,
            Err(e) => {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        };
    let adapters: Vec<acpi_client::ACAdapterInfo> =
        match acpi_client::get_ac_adapter_info(&power_supply_path) {
            Ok(ac) => ac,
            Err(e) => {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        };

    for bat in batteries {
        display_power_supply_info(&bat);
    }
    for ac in adapters {
        display_ac_adapter_info(&ac);
    }

    Ok(())
}

fn display_power_supply_info(bat: &acpi_client::BatteryInfo) {
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
