
#ifndef AppVersion
  #define AppVersion "dev"
#endif

#ifndef CargoOutput
  #if GetEnv("CARGO_BUILD_TARGET")
    #define CargoOutput "target\" + GetEnv("CARGO_BUILD_TARGET") + "\release"
  #else
    #define CargoOutput "target\release"
  #endif
#endif

[Setup]
SourceDir="..\"
; # App info
AppName=Mystic Light Browser Cinema
AppVersion={#AppVersion}
AppPublisher=meskill
AppPublisherURL=https://github.com/meskill
AppSupportURL=https://github.com/meskill/mystic-light-browser-cinema/issues
AppUpdatesURL=https://github.com/meskill/mystic-light-browser-cinema/releases
; # Installation info
DefaultDirName={autopf}\Mystic Light Browser Cinema
DisableProgramGroupPage=yes
ArchitecturesInstallIn64BitMode=x64
; we will start the service by ourselves
RestartApplications=no
; close currently running service
CloseApplications=yes
; # Output Info
OutputDir="dist"
; # Looking
WizardStyle=modern
SetupIconFile="installer\assets\icon.ico"
UninstallDisplayIcon="{app}\icon.ico"

[Languages]
Name: "en"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "installer\assets\icon.ico"; DestDir: "{app}"
#if isWin64()
  Source: "{#CargoOutput}\sdk\MysticLight_SDK_x64.dll"; DestDir: "{app}/sdk"
#else
  Source: "{#CargoOutput}\sdk\MysticLight_SDK.dll"; DestDir: "{app}/sdk"
#endif
Source: "{#CargoOutput}\service.exe"; DestDir: "{app}"
Source: "{#CargoOutput}\install.exe"; DestDir: "{app}"
Source: "{#CargoOutput}\uninstall.exe"; DestDir: "{app}"

[Run]
Filename: "{app}\install.exe"; Flags: runhidden; WorkingDir: "{app}"

[UninstallRun]
Filename: "{app}\uninstall.exe"; Flags: runhidden; WorkingDir: "{app}"; RunOnceId: "UninstallService"

[UninstallDelete]
Type: filesandordirs; Name: "{commonappdata}\Mystic Light\Mystic Light Browser Cinema"