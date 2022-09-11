## Installer

[Inno Setup](https://jrsoftware.org/isinfo.php) is used to build an installer for native application.

### Build an installer

- Make sure the `iscc.exe` is available in terminal
- In terminal move to directory `./native-app`
- Run command `iscc.exe installer/inno-setup.iss`

## Debug

### Enable additional logging

Enable these tracing modules:
- mystic_light_browser_cinema - logs from the app itself
- mystic_light_sdk - logs from the underlying sdk library
- tower_http - web-server implementation used under hood
- async_graphql - graphql library used in the app