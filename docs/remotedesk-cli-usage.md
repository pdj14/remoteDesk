# RemoteDesk CLI Usage (Windows)

## 1. Overview

`remotedesk-cli.exe` is a service-oriented CLI for installed RemoteDesk environments.

- It is designed for installed mode only.
- It is not intended for portable mode operation.
- It controls the same Windows service used by `remotedesk.exe`.
- `remotedesk-admin.exe` is available for install/uninstall operations with console progress output.

## 2. Installation and Removal Flow

### 2.1 Install (GUI + CLI together)

1. Place these files in the same folder:
   - `remotedesk.exe`
   - `remotedesk-cli.exe`
2. Run:

```cmd
remotedesk.exe --silent-install
```

Behavior:

- Installation fails if `remotedesk-cli.exe` is missing next to `remotedesk.exe`.
- On success, both GUI and CLI are installed into the install directory.
- If you need progress output in console, use:

```cmd
remotedesk-admin.exe --install
```

This performs the same installed-mode flow as `remotedesk.exe --silent-install`, but prints running status.
It installs to a folder created next to `remotedesk-admin.exe`:

- `<admin_exe_directory>\\RemoteDesk`

### 2.2 Uninstall (GUI + CLI together)

Run:

```cmd
remotedesk.exe --uninstall
```

Behavior:

- Service and related startup/registry entries are removed.
- Install directory is removed, so both `remotedesk.exe` and `remotedesk-cli.exe` are deleted.
- You can also uninstall with progress output:

```cmd
remotedesk-admin.exe --uninstall
```

## 3. CLI Execution Rule (Installed Mode Only)

For service control commands, `remotedesk-cli.exe` must satisfy both:

- RemoteDesk is installed.
- CLI is launched from installed path.

If not, CLI exits with code `3`.

Example (recommended):

```cmd
"%ProgramFiles%\\RemoteDesk\\remotedesk-cli.exe" --status
```

## 4. Supported CLI Commands

```cmd
remotedesk-cli.exe --help
remotedesk-cli.exe --version
remotedesk-cli.exe --status
remotedesk-cli.exe --install-service
remotedesk-cli.exe --uninstall-service
remotedesk-cli.exe --start-service
remotedesk-cli.exe --stop-service
```

## 4.1 Supported Admin Commands

```cmd
remotedesk-admin.exe --help
remotedesk-admin.exe --version
remotedesk-admin.exe --status
remotedesk-admin.exe --install
remotedesk-admin.exe --uninstall
```

## 5. Service Command Semantics

### `--install-service`

- Installs the Windows service.
- Starts the service immediately.
- In short: install + start.

### `--uninstall-service`

- Stops the service.
- Deletes the service from SCM.
- In short: stop + uninstall.

### `--start-service`

- Starts an already-installed service (`sc start`).
- No install action.

### `--stop-service`

- Stops a running service (`sc stop`).
- No uninstall action.

### `--status`

- Prints:
  - installation status
  - service running status
  - install path
  - configured service binary path

## 6. Exit Codes

- `0`: success
- `1`: runtime failure
- `2`: invalid/unsupported argument usage
- `3`: installed-mode check failed (not installed or wrong launch path)

## 7. Quick Start

1. Install with progress output:

```cmd
remotedesk-admin.exe --install
```

2. Install and start service:

```cmd
"<admin_exe_directory>\\RemoteDesk\\remotedesk-cli.exe" --install-service
```

3. Check status:

```cmd
"<admin_exe_directory>\\RemoteDesk\\remotedesk-cli.exe" --status
```

4. Stop/start service as needed:

```cmd
"<admin_exe_directory>\\RemoteDesk\\remotedesk-cli.exe" --stop-service
"<admin_exe_directory>\\RemoteDesk\\remotedesk-cli.exe" --start-service
```
