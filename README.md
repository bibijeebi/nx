# nx

A friendly command-line wrapper for common Nix commands that simplifies package management and system configuration on NixOS.

## Features

- Simple package installation and removal
- Easy system configuration management
- Support for temporary and permanent system changes
- Streamlined garbage collection and store optimization
- Unfree package handling
- Background process support for long-running operations

## Installation

1. Ensure you have Rust and Cargo installed
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```
4. The binary will be available at `target/release/nx`

## Usage

### Package Management

```bash
# Install a package
nx i <package>
nx install <package>

# Remove a package
nx rm <package>
nx remove <package>

# Remove all packages
nx rm --all

# Update a specific package
nx u <package>
nx update <package>

# Update all packages
nx update --all

# List installed packages
nx ls
nx list

# Search for packages
nx s <query>
nx search <query>
```

### System Management

```bash
# Add a system package
nx sys pkg <package>
nx system package <package> --permanent

# Enable a program or service
nx sys en programs.fish
nx system enable services.docker --permanent

# Set a system option
nx sys set networking.hostName "myhost"
nx system set-option programs.fish.enable true --permanent

# Show pending temporary changes
nx sys show

# Apply temporary changes
nx sys apply
```

### Maintenance

```bash
# Run garbage collection
nx gc
nx garbage-collect

# Run garbage collection and delete old generations
nx gc -d
nx garbage-collect --delete-old

# Optimize the Nix store
nx o
nx optimize

# Run operations in foreground (default is background)
nx gc -f
nx optimize --foreground
```

### Configuration

```bash
# Allow unfree packages
nx config --allow-unfree true

# Show current configuration
nx config
```

## Additional Arguments

You can pass additional arguments to the underlying Nix commands by adding them after `--`:

```bash
nx install firefox -- --option substitute false
```

## Configuration

The configuration file is stored at:
- Linux: `~/.config/nx-wrapper/config.json`
- macOS: `~/Library/Application Support/com.nx.nx-wrapper/config.json`

Current configuration options:
- `allow_unfree`: Boolean to control installation of unfree packages

## Development

### Dependencies

This project requires:
- Rust 2021 edition
- The following crates:
  - clap
  - directories
  - serde
  - serde_json
  - tokio

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

## License

[Add your chosen license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- The Nix/NixOS community
- All contributors to the project