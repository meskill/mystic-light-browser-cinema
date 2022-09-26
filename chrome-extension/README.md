# Browser extension

Extension that can communicate with [native-app](../native-app/README.md) to change the state of MysticLight devices on browser events

## How does it work

- the extension is build using [Plasmo Framework](https://docs.plasmo.com)
- extension reads config file that is populated from the native-app (and that is why extension requires "Allow access to file URLs") in order to figure out how to communicate with it
- extension adds listener for browser fullscreen events
- when browser enters fullscreen the extension:
  - requests current devices state and stores it in memory
  - sends new state for devices with disabled lighting
- when browser exists fullscreen the extension:
  - sends previous stored state to revert it to previous state
