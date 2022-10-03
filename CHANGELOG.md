## [1.0.5](https://github.com/meskill/mystic-light-browser-cinema/compare/v1.0.4...v1.0.5) (2022-10-03)


### Chores

* add devcontainer setup and contribution guide ([e10d101](https://github.com/meskill/mystic-light-browser-cinema/commit/e10d101d774138fb53e6467be27d6f5f5f3589cf))
* **chrome-extension:** simplify check for fileSchemaAccess ([7198e54](https://github.com/meskill/mystic-light-browser-cinema/commit/7198e54eea1ae75cedf5ed58b000c09ab0ce6c01))


### Continuous Integration

* **chrome-extension:** automate publish to chrome web store ([be7f4a6](https://github.com/meskill/mystic-light-browser-cinema/commit/be7f4a667914dde0337f6e453135a2da92462c8b))
* fix Browser Publisher action version ([829eaef](https://github.com/meskill/mystic-light-browser-cinema/commit/829eaef886b7dd91484912e80e146c3ea1faee40))

## [1.0.4](https://github.com/meskill/mystic-light-browser-cinema/compare/v1.0.3...v1.0.4) (2022-09-29)


### Bug Fixes

* **chrome-extension:** consistent Mystic Light naming ([bffde64](https://github.com/meskill/mystic-light-browser-cinema/commit/bffde64e6ddc79b718483092adf264ed51424adb))

## [1.0.3](https://github.com/meskill/mystic-light-browser-cinema/compare/v1.0.2...v1.0.3) (2022-09-29)


### Bug Fixes

* **chrome-extension:** fixes of the alert messages ([799229d](https://github.com/meskill/mystic-light-browser-cinema/commit/799229dc5d58232721faea678beced8939c77c10))


### Continuous Integration

* fix required checks ([5f24a69](https://github.com/meskill/mystic-light-browser-cinema/commit/5f24a69b6ea5e26c47647f6a5dea53a834916f81))


### Documentation

* add READMEs ([a54991e](https://github.com/meskill/mystic-light-browser-cinema/commit/a54991e285db3823ebd160eb959de747097e2231))

## [1.0.2](https://github.com/meskill/mystic-light-browser-cinema/compare/v1.0.1...v1.0.2) (2022-09-25)


### Bug Fixes

* **native-app:** enable fail actions for the internal errors as well ([e622d4b](https://github.com/meskill/mystic-light-browser-cinema/commit/e622d4bdf7c8b4545ca2e6b84720ea7ca00267f1))


### Continuous Integration

* fix Cargo.lock update ([b3c8f2d](https://github.com/meskill/mystic-light-browser-cinema/commit/b3c8f2df3df73974065a51d9c2e6338d8309aba7))


### Code Refactoring

* **native-app:** change the way random port for listening is picked ([a16cf59](https://github.com/meskill/mystic-light-browser-cinema/commit/a16cf59526c1268e23b96f4c0b62a5b0a4a42fef))
* **native-app:** change the way to override logs are written ([36f42d4](https://github.com/meskill/mystic-light-browser-cinema/commit/36f42d4ff280a5a460251e7c7f01cce9b56e9087))

## [1.0.1](https://github.com/meskill/mystic-light-browser-cinema/compare/v1.0.0...v1.0.1) (2022-09-25)


### Bug Fixes

* **native-app:** add failure actions for service ([e12ed5f](https://github.com/meskill/mystic-light-browser-cinema/commit/e12ed5fc2bac5ae73e0da447a5b015ecf6c22334))


### Continuous Integration

* fix some publish details ([af88dcf](https://github.com/meskill/mystic-light-browser-cinema/commit/af88dcff2fce7c5b4d907b494b4c6613a3c7899b))

## 1.0.0 (2022-09-25)


### Features

* **chrome-extension:** add chrome extension ([834958b](https://github.com/meskill/mystic-light-browser-cinema/commit/834958bfedaaec93ab069f080cbfe8f2ad8d681d))
* **native-app:** add inno setup installer ([43e905e](https://github.com/meskill/mystic-light-browser-cinema/commit/43e905edd11bc1f6f388c683054968a8ddcb57c5))
* **native-app:** add native-app ([b02ab2f](https://github.com/meskill/mystic-light-browser-cinema/commit/b02ab2f25b8ad5fec6183518b4c6fbd4ca9ccf70))
* **native-app:** implement windows service wrappers ([249dd02](https://github.com/meskill/mystic-light-browser-cinema/commit/249dd02e21f8452c263c08b2cfc5dbe47af3a52c))
* **native-app:** migrate to tracing library for logging ([e4f522f](https://github.com/meskill/mystic-light-browser-cinema/commit/e4f522fe4d853c71e98ed30a1c9efff90328f287))


### Bug Fixes

* **chrome-extension:** remove iframe after its use ([5197033](https://github.com/meskill/mystic-light-browser-cinema/commit/51970339c9dbeaad6d14e0af9705f0acb324b933))
* **chrome-extension:** restore state on tab close ([e018168](https://github.com/meskill/mystic-light-browser-cinema/commit/e018168e1ddd9e43f86f1d7d2fa0b0c5933d8fab))
* **native-app:** enable very permissive cors handling ([4b2867d](https://github.com/meskill/mystic-light-browser-cinema/commit/4b2867dd3d6d3e983f428684524c86cb0131b10f))
* **native-app:** store logs in ProgramData ([d6ed5d8](https://github.com/meskill/mystic-light-browser-cinema/commit/d6ed5d8ae06916c3dc8fd15b971de41a320c9fd1))


### Performance

* **native-app:** optimize binaries output size ([cfeceba](https://github.com/meskill/mystic-light-browser-cinema/commit/cfeceba2ba28614c22ef462f473b18fb2cf8dd72))


### Chores

* add basic gitignore ([df51077](https://github.com/meskill/mystic-light-browser-cinema/commit/df51077fe49797ba8600e0cfb73f3ba0d614fc00))
* **chrome-extension:** update eslint config ([25aef55](https://github.com/meskill/mystic-light-browser-cinema/commit/25aef55dcbbf97df1e99ab530a275f83c2894dc0))


### Code Refactoring

* **native-app:** change the way the logging, sdk and server is instantiated ([7a8e980](https://github.com/meskill/mystic-light-browser-cinema/commit/7a8e980ceaf66b73e4688b05b4256804a59180ff))
* **native-app:** generate graphql schema from a separate bin ([b0df1ef](https://github.com/meskill/mystic-light-browser-cinema/commit/b0df1ef7028d0937b05050f4f5865b5722da8bbc))
* **native-app:** move windows service code to library ([5bd0430](https://github.com/meskill/mystic-light-browser-cinema/commit/5bd043043492da2e260145b03a2e976e1bb91ac8))


### Continuous Integration

* init ci ([c9d99e1](https://github.com/meskill/mystic-light-browser-cinema/commit/c9d99e144c11c127a0bcac63102ac2f45bd51780))
