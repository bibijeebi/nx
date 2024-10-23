# NX - Extended Feature Set

## Package Management

### Installation Commands
```bash
nx i <pkg>              # Install package to profile
nx i -g <pkg>          # Install globally (system packages)
nx i -t <pkg>          # Install temporarily (nix-shell style)
nx i -d <pkg>          # Install with all dependencies shown
nx i --from <commit>   # Install specific version from git history
nx i --patch <pkg>     # Install with local patches applied
```

### Development Environments
```bash
nx dev                 # Enter development shell based on flake.nix
nx dev --pure          # Pure development environment
nx dev --from <lang>   # Quick dev environment for language (python/node/etc)
nx dev -p pkg1,pkg2    # Create shell with specific packages
nx dev --save          # Save current shell config to flake.nix
```

### System Management

#### Configuration
```bash
nx sys pkg <pkg>       # Add to system packages
nx sys en <program>    # Enable a program/service
nx sys set <path> <val> # Set configuration option
nx sys edit            # Edit configuration.nix with $EDITOR
nx sys history         # Show system generation history
nx sys rollback        # Rollback to previous generation
nx sys diff            # Show changes between generations
nx sys snapshot        # Create labeled snapshot of current config
nx sys restore <label> # Restore labeled snapshot
```

#### Hardware
```bash
nx hw scan            # Scan and show hardware configuration
nx hw drivers         # Show/modify driver configurations
nx hw gpu             # Manage GPU drivers/settings
nx hw audio           # Audio device management
nx hw mount           # Quick mount helper for devices
nx hw power           # Power management settings
```

### Flake Management
```bash
nx flake init         # Initialize new flake
nx flake update      # Update flake inputs
nx flake lock        # Update flake.lock
nx flake show        # Show flake outputs
nx flake edit        # Edit flake.nix
nx flake add <input> # Add new input to flake
nx flake rm <input>  # Remove input from flake
nx flake tree        # Show dependency tree
```

### Cache Operations
```bash
nx cache gc          # Garbage collection
nx cache opt         # Optimize store
nx cache stats       # Show cache statistics
nx cache verify      # Verify store integrity
nx cache repair      # Attempt to repair store issues
nx cache export      # Export paths to file
nx cache import      # Import paths from file
nx cache sign        # Sign paths
```

### Development Tools

#### Build Management
```bash
nx build .           # Build current project
nx build --pure      # Build in pure environment
nx build -a <arch>   # Cross-compile for architecture
nx build --debug     # Build with debug symbols
nx build --profile   # Build with specific profile
```

#### Testing
```bash
nx test             # Run project tests
nx test -v          # Verbose test output
nx test --coverage  # Run tests with coverage
nx test --watch     # Watch mode for tests
```

#### Packaging
```bash
nx pkg init         # Initialize new package
nx pkg build       # Build package
nx pkg publish     # Publish to cache/registry
nx pkg lint        # Lint package definition
nx pkg fmt         # Format package files
```

### Search and Discovery
```bash
nx s <query>        # Search packages
nx s -d <pkg>      # Show detailed package info
nx s --by-license  # Search by license type
nx s --maintainer  # Search by maintainer
nx s --eval        # Search with nix evaluation
nx s --json        # Output in JSON format
```

### Profile Management
```bash
nx profile list    # List installed packages
nx profile clean   # Remove old generations
nx profile diff    # Show changes between generations
nx profile export  # Export profile to file
nx profile import  # Import profile from file
nx profile switch  # Switch between profiles
```

### Maintenance & Debugging
```bash
nx doctor          # Check system health
nx doctor fix      # Attempt to fix common issues
nx why <pkg>       # Show why package is installed
nx why-depends     # Show reverse dependencies
nx check          # Validate configurations
nx logs           # Show build logs
nx trace          # Trace evaluation
```

### Shell Integration

#### Environment Management
```bash
nx env save        # Save current environment
nx env load        # Load saved environment
nx env diff        # Show environment differences
nx env clean       # Clean environment variables
```

#### Shell Helpers
```bash
nx alias           # Manage nix-related aliases
nx complete       # Manage shell completions
nx path           # Manage PATH additions
```

### Documentation
```bash
nx docs search    # Search documentation
nx docs show      # Show package documentation
nx docs generate  # Generate documentation
nx docs serve     # Serve documentation locally
```

### Updates and Maintenance
```bash
nx update          # Update all packages
nx update -s       # Update system packages
nx update --sec    # Security updates only
nx update --check  # Check for updates
nx update --diff   # Show changes before updating
```

### Advanced Features

#### Container Management
```bash
nx container run   # Run in container
nx container build # Build container
nx container push  # Push to registry
nx container shell # Shell into container
```

#### VM Management
```bash
nx vm create      # Create NixOS VM
nx vm run         # Run VM
nx vm list        # List VMs
nx vm snapshot    # Manage VM snapshots
```

#### Network
```bash
nx net proxy      # Manage proxy settings
nx net cache      # Configure binary caches
nx net mirror     # Configure mirrors
nx net speed      # Test mirror speeds
```

### Configuration Templates
```bash
nx template list  # List available templates
nx template use   # Apply template
nx template save  # Save current as template
nx template share # Share template
```

### Security Features
```bash
nx sec audit      # Security audit
nx sec harden     # System hardening
nx sec scan       # Vulnerability scan
nx sec update     # Security updates
```

### Integration Features

#### Git Integration
```bash
nx git hook       # Manage git hooks
nx git ignore     # Manage .gitignore
nx git clean      # Clean build artifacts
```

#### CI/CD Support
```bash
nx ci init        # Initialize CI config
nx ci check       # Check CI configuration
nx ci cache       # Manage CI caches
```

### Helper Commands
```bash
nx run <cmd>      # Run in nix shell
nx which <cmd>    # Show package for command
nx size <pkg>     # Show package size
nx tree <pkg>     # Show dependency tree
nx fmt            # Format nix files
nx repl           # Enhanced nix repl
```

## Advanced Use Cases

### Workspace Management
- Managed development environments
- Project-specific configurations
- Team sharing capabilities
- Remote development support

### System Profiles
- Role-based configurations (development, server, desktop)
- Environment-specific settings
- Quick switching between profiles
- Profile sharing and synchronization

### Monitoring & Analytics
- Resource usage tracking
- Build time analytics
- Cache hit rates
- System performance metrics

### Automation & Scripting
- Task automation
- Custom script integration
- Event hooks
- Scheduled operations

## Configuration Options

### Global Settings
- Default behaviors
- Path configurations
- Security policies
- Resource limits

### User Preferences
- UI/Output preferences
- Default flags
- Alias definitions
- Custom scripts

### Integration Settings
- External tool configurations
- API keys and tokens
- Remote system configs
- Network settings

Would you like me to elaborate on any of these features or add more specific implementation details for any particular area?