fn main() -> std::io::Result<()> {
    let power_supplies: Vec<acpi_client::PowerSupplyInfo> =
        match acpi_client::get_power_supply_info() {
            Ok(ps) => ps,
            Err(e) => {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        };

    for ps in power_supplies {
        if ps.is_battery {
            display_power_supply_info(&ps);
        }
    }

    Ok(())
}

fn display_power_supply_info(ps: &acpi_client::PowerSupplyInfo) {
    if ps.is_battery {
        let state = match &ps.state {
            acpi_client::ChargingState::Charging => "Charging",
            acpi_client::ChargingState::Discharging => "Discharging",
            acpi_client::ChargingState::Full => "Full",
        };
        let mut seconds = ps.time_remaining.as_secs();
        let hours = seconds / 3600;
        seconds = seconds - hours * 3600;
        let minutes = seconds / 60;
        seconds = seconds - minutes * 60;
        let not_full_string = format!(", {:02}:{:02}:{:02}", hours, minutes, seconds);
        let charge_time_string = match &ps.state {
            acpi_client::ChargingState::Charging => {
                if ps.present_rate > 0 {
                    format!("{} {}", not_full_string, "until charged")
                } else {
                    format!(", charging at zero rate")
                }
            }
            acpi_client::ChargingState::Discharging => {
                format!("{} {}", not_full_string, "remaining")
            }
            _ => String::from(""),
        };
        println!(
            "{}: {}, {:.1}%{}",
            &ps.name, state, ps.percentage, charge_time_string
        );
    } else {
        println!("{}", &ps.name)
    }
}
