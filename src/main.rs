use zbus::{proxy, Connection, Result};
use zbus::zvariant::{Str, OwnedObjectPath};
use futures_util::stream::StreamExt;
use std::{io, io::Write};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Systemd unit name")]
    unit: String,

    #[arg(long, help = "Output to display when the unit is active")]
    active_output: String,

    #[arg(long, help = "Output to display when the unit is inactive")]
    inactive_output: String,

    #[arg(long, default_value_t = true, help = "Enable streaming mode")]
    streaming: bool,

    #[arg(long, help = "Enable oneshot mode")]
    oneshot: bool,
}

const VALID_UNIT_TYPES: [&str; 3] = [
    ".service", ".socket", ".device",
];

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !is_valid_unit(&args.unit) {
        eprintln!("Error: The unit '{}' is not a valid systemd unit.", args.unit);
        std::process::exit(1);
    }

    let connection = Connection::session().await?;
    let manager_proxy = ManagerProxy::new(&connection).await?;

    let unit_path = manager_proxy.load_unit(&args.unit).await?;

    let unit_proxy = UnitProxy::builder(&connection)
        .path(unit_path.clone())?
        .build()
        .await?;

    let active_state = unit_proxy.active_state().await?;
    let mut last_output = map_status_output(&active_state, &args);
    print_output(last_output, args.streaming);

    if args.oneshot {
        return Ok(());
    }

    let properties_proxy = zbus::fdo::PropertiesProxy::builder(&connection)
        .destination("org.freedesktop.systemd1")?
        .path(unit_path)?
        .build()
        .await?;

    let mut signal_stream = properties_proxy.receive_properties_changed().await?;

    while let Some(signal) = signal_stream.next().await {
        let args_signal = signal.args()?;
        if let Some(value) = args_signal.changed_properties().get("ActiveState") {
            match value.downcast_ref::<Str>() {
                Ok(state) => {
                    let current_output = map_status_output(state.as_str(), &args);
                    if current_output != last_output {
                        print_output(current_output, args.streaming);
                        last_output = current_output;
                    }
                }
                Err(e) => eprintln!("Failed to downcast ActiveState: {:?}", e),
            }
        }
    }

    Ok(())
}

fn is_valid_unit(unit: &str) -> bool {
    VALID_UNIT_TYPES.iter().any(|&suffix| unit.ends_with(suffix))
}

fn map_status_output<'a>(state: &str, args: &'a Args) -> &'a str {
    match state {
        "active" => &args.active_output,
        _ => &args.inactive_output,
    }
}

fn print_output(output: &str, streaming: bool) {
    println!("{}", output);
    if streaming {
        println!();
    }
    io::stdout().flush().unwrap();
}

#[proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait Manager {
    async fn load_unit(&self, name: &str) -> Result<OwnedObjectPath>;
}

#[proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1"
)]
trait Unit {
    #[zbus(property)]
    fn active_state(&self) -> Result<String>;
}
