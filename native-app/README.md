# Native-app

Native app written in Rust and wrapped in Windows service to provide interaction between `Mystic Light SDK` and `Mystic Light Browser Cinema` browser extension

## Debug

Logs for service and install/uninstall tools are stored in directory `%ProgramData%\\Mystic Light\\Mystic Light Browser Cinema\\logs`

By default, logs contains events from app itself, underlying sdk and install/uninstall tools.

### Control log output

Log details can be controlled by Environment Variable `MYSTIC_LIGHT_LOG`.

If env is not specified the default value will be used that roughly equals to `error,mystic_light_sdk=debug,mystic_light_browser_cinema=debug,install=debug,uninstall=debug,service=debug`

By specifying this options you may control what will be output to the logs files. To figure out how to specify this env refer to [tracing_subscriber](https://docs.rs/tracing-subscriber/0.3.15/tracing_subscriber/struct.EnvFilter.html#directives) documentation and look for the available targets below. This env should reside under `System Variables` not `User variables` as all of the apps are running from the administrator account, not user.

> After adding/changing env value you have to restart Mystic Light Browser Cinema service. It might be done through `Services` panel in Windows - just find the service, open context menu and pick up restart

The most important targets in logging are:
- mystic_light_browser_cinema - logs from the app itself
- install - logs from install tool
- uninstall - logs from uninstall tool
- service - logs from running windows service
- mystic_light_sdk - logs from the underlying sdk library
- tower_http - web-server implementation used under hood
- async_graphql - graphql library used in the app

Some of the examples:
- `error` - show only errors
- `error,mystic_light_browser_cinema=debug,tower_http=debug` - show all errors and debug info from the app and http server
- `install=debug,uninstall=debug,service=debug` - show only info from wrapper tools

## Installer

[Inno Setup](https://jrsoftware.org/isinfo.php) is used to build an installer for native application.

### Build an installer

- Make sure the `iscc.exe` is available in terminal
- In terminal move to directory `./native-app`
- Run command `iscc.exe installer/inno-setup.iss`