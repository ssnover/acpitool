use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("acpitool")
        .version("1.0")
        .author("Shane Snover <ssnover95@gmail.com>")
        .arg(
            Arg::with_name("battery")
                .short("b")
                .long("battery")
                .help("Battery information"),
        )
        .arg(Arg::with_name("details").short("i").long("details").help(
            "Show additional details if available on battery capacity and temperature trip points",
        ))
        .arg(
            Arg::with_name("ac-adapter")
                .short("a")
                .long("ac-adapter")
                .help("AC adapter information"),
        )
        .arg(
            Arg::with_name("thermal")
                .short("t")
                .long("thermal")
                .help("Thermal information"),
        )
        .arg(
            Arg::with_name("cooling")
                .short("c")
                .long("cooling")
                .help("Cooling information"),
        )
        .arg(
            Arg::with_name("everything")
                .short("V")
                .long("everything")
                .help("Show every device, overrides above options"),
        )
        .arg(
            Arg::with_name("show-empty")
                .short("s")
                .long("show-empty")
                .help("Show non-operational devices"),
        )
        .arg(
            Arg::with_name("fahrenheit")
                .short("f")
                .long("fahrenheit")
                .help("Use Fahrenheit as the temperature unit"),
        )
        .arg(
            Arg::with_name("kelvin")
                .short("k")
                .long("kelvin")
                .help("Use Kelvin as the temperature unit"),
        )
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIR")
                .help("Path to ACPI info; default is /sys/class")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Display this help and exit"),
        )
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Output version information and exit"),
        )
        .get_matches();
    if matches.is_present("help") {
        print_usage_and_exit();
        Ok(())
    } else {
        let acpi_path = std::path::Path::new(matches.value_of("directory").unwrap_or("/sys/class"))
            .to_path_buf();
        let cfg = acpitool::Config {
            acpi_path,
            show_battery: matches.is_present("battery") || matches.is_present("everything"),
            show_ac_adapter: matches.is_present("ac-adapter") || matches.is_present("everything"),
            detailed: matches.is_present("details") || matches.is_present("everything"),
        };
        match acpitool::run(cfg) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn print_usage_and_exit() {
    println!("Shows information from the /proc filesystem such as battery status or");
    println!("thermal information.");
    println!("");
    println!("   -b, --battery           Battery information");
    println!("   -i, --details           Show additional details if available:");
    println!("                             - Battery capacity information");
    println!("                             - Temperature trip points");
    println!("   -a, --ac-adapter        AC adapter information");
    println!("   -V, --everything        Show every device, overrides above options");
    println!("   -d, --directory <DIR>   Path to ACPI info; default is /sys/class");
    println!("   -h, --help              Display this usage and exit");
    println!("");
    std::process::exit(0);
}
