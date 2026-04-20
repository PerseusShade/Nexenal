@echo off
echo ==========================================
echo       NEXENAL BUILD ^& DEPLOY TOOL
echo ==========================================

cd /d "%~dp0\.."

set TARGET_DIR=C:\UserApps\Nexenal
set ACTION=%1
:: Use your exact Inno Setup path
set ISCC_PATH="C:\UserApps\Inno Setup 6\ISCC.exe"

echo [1/4] Compiling with release optimizations...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo [FATAL ERROR] Compilation failed!
    pause
    exit /b %errorlevel%
)

if /I "%ACTION%"=="build" (
    echo.
    echo [INFO] Build successful. Deployment skipped.
    echo ==========================================
    exit /b 0
)

if /I "%ACTION%"=="setup" (
    echo.
    echo [2/4] Generating Inno Setup Installer...
    if not exist %ISCC_PATH% (
        echo [ERROR] Inno Setup compiler not found at %ISCC_PATH%
        pause
        exit /b 1
    )
    %ISCC_PATH% scripts\nexenal.iss
    echo.
    echo [INFO]       Installer generated in Output\ directory!
    echo ==========================================
    exit /b 0
)

echo [2/4] Creating target directory if needed...
if not exist "%TARGET_DIR%" mkdir "%TARGET_DIR%"

echo [3/4] Copying executable and documentation...
copy /Y "target\release\nexenal.exe" "%TARGET_DIR%\"
copy /Y "README.md" "%TARGET_DIR%\"
copy /Y "LICENSE.md" "%TARGET_DIR%\"

echo [4/4] Verifying configuration file...
if not exist "%TARGET_DIR%\config.json" (
    copy /Y "assets\config.json" "%TARGET_DIR%\"
    echo [INFO] Default config.json created.
) else (
    echo [INFO] Existing config.json kept.
)

echo ==========================================
echo       Local Deployment successful.
echo ==========================================
pause