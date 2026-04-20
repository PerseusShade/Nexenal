#define MyAppVersion "0.1.0"

[Setup]
AppName=Nexenal
AppVersion={#MyAppVersion}
AppPublisher=PerseusShade
DefaultDirName=C:\UserApps\Nexenal
DefaultGroupName=Nexenal
OutputDir=..\Output
; Utilisation de la macro pour le nom du fichier final
OutputBaseFilename=Nexenal_Installer_v{#MyAppVersion}
Compression=lzma
SolidCompression=yes
ChangesEnvironment=yes
DirExistsWarning=no

[Files]
Source: "..\target\release\nexenal.exe"; DestDir: "{app}"; Flags: ignoreversion
; On utilise onlyifdoesntexist pour protéger la configuration de l'utilisateur lors d'une MAJ
Source: "..\assets\config.json"; DestDir: "{app}"; Flags: onlyifdoesntexist
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE.md"; DestDir: "{app}"; Flags: ignoreversion

[Code]
const
    EnvironmentKey = 'Environment';

function NeedsAddPath(Param: string): boolean;
var
    OrigPath: string;
begin
    if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath) then
    begin
        Result := True;
        exit;
    end;
    Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
    OrigPath: string;
begin
    if CurStep = ssPostInstall then
    begin
        if NeedsAddPath(ExpandConstant('{app}')) then
        begin
            RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath);
            RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath + ';' + ExpandConstant('{app}'));
        end;
    end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
    OrigPath: string;
    AppPath: string;
begin
    if CurUninstallStep = usPostUninstall then
    begin
        AppPath := ExpandConstant('{app}');
        if RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath) then
        begin
            StringChangeEx(OrigPath, ';' + AppPath, '', True);
            StringChangeEx(OrigPath, AppPath + ';', '', True);
            StringChangeEx(OrigPath, AppPath, '', True);
            RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath);
        end;
    end;
end;