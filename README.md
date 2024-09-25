<div align="center">
  <h1>uwatch</h1>
</div>

## About

`uwatch` monitors the state of a systemd unit and outputs messages based on its status.

## Installation

### Build from source

Run the following commands:

```console
git clone https://github.com/e-tho/uwatch
cd uwatch
cargo build --release
```

An executable file will be generated at `target/release/uwatch`, which you can then copy to a directory in your `$PATH`.

### Nix

Add the flake as an input:

```nix
uwatch.url = "github:e-tho/uwatch";
```

Install the package:

```nix
environment.systemPackages = [ inputs.uwatch.packages.${pkgs.system}.default ];
```

## Usage

Specify the systemd unit to watch, along with the output for active and inactive states:

```console
uwatch --unit myservice.service --active-output "<active_output>" --inactive-output "<inactive_output>"
```

### Available Options

| Flag                | Description                                   | Supported Values       | Default Value |
| ------------------- | --------------------------------------------- | ---------------------- | ------------- |
| `--unit`            | The systemd unit to monitor.                  | Any valid systemd unit | `None`        |
| `--active-output`   | Output when the unit is active.               | Any string             | `None`        |
| `--inactive-output` | Output when the unit is inactive.             | Any string             | `None`        |
| `--streaming`       | Enable continuous monitoring (default).       | None                   | `enabled`     |
| `--oneshot`         | Disable continuous monitoring (single check). | None                   | `disabled`    |

## License

GPLv3
