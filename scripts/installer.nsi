; Parallel Mengene NSIS Installer Script
; This script creates a Windows installer for Parallel Mengene

!define APPNAME "Parallel Mengene"
!define COMPANYNAME "Parallel-Mengene Team"
!define DESCRIPTION "High-performance parallel file compression tool"
!define VERSIONMAJOR 0
!define VERSIONMINOR 1
!define VERSIONBUILD 0
!define HELPURL "https://github.com/hocestnonsatis/parallel-mengene"
!define UPDATEURL "https://github.com/hocestnonsatis/parallel-mengene"
!define ABOUTURL "https://github.com/hocestnonsatis/parallel-mengene"
!define INSTALLSIZE 10000

; Request application privileges for Windows Vista and higher
RequestExecutionLevel admin

; Main Install settings
Name "${APPNAME}"
InstallDir "$PROGRAMFILES\${APPNAME}"
InstallDirRegKey HKLM "Software\${APPNAME}" ""
OutFile "parallel-mengene-installer.exe"

; Modern UI
!include "MUI2.nsh"

; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE.txt"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "English"

; Version Information
VIProductVersion "${VERSIONMAJOR}.${VERSIONMINOR}.${VERSIONBUILD}.0"
VIFileVersion "${VERSIONMAJOR}.${VERSIONMINOR}.${VERSIONBUILD}.0"

; Installer sections
Section "install" InstallSection
    SetOutPath "$INSTDIR"
    
    ; Copy the main executable
    File "target\x86_64-pc-windows-msvc\release\parallel-mengene.exe"
    
    ; Copy documentation
    File "README.md"
    File "docs\USER_GUIDE.md"
    File "docs\API_REFERENCE.md"
    
    ; Create uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"
    
    ; Add to PATH
    EnVar::SetHKCU
    EnVar::AddValue "PATH" "$INSTDIR"
    
    ; Registry information for add/remove programs
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayName" "${APPNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "QuietUninstallString" "$\"$INSTDIR\uninstall.exe$\" /S"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "InstallLocation" "$\"$INSTDIR$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayIcon" "$\"$INSTDIR\parallel-mengene.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "Publisher" "${COMPANYNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "HelpLink" "${HELPURL}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "URLUpdateInfo" "${UPDATEURL}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "URLInfoAbout" "${ABOUTURL}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayVersion" "${VERSIONMAJOR}.${VERSIONMINOR}.${VERSIONBUILD}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMajor" "${VERSIONMAJOR}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMinor" "${VERSIONMINOR}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoRepair" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "EstimatedSize" "${INSTALLSIZE}"
    
    ; Create Start Menu shortcuts
    CreateDirectory "$SMPROGRAMS\${APPNAME}"
    CreateShortCut "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk" "$INSTDIR\parallel-mengene.exe" "" "$INSTDIR\parallel-mengene.exe" 0
    CreateShortCut "$SMPROGRAMS\${APPNAME}\User Guide.lnk" "$INSTDIR\USER_GUIDE.md"
    CreateShortCut "$SMPROGRAMS\${APPNAME}\API Reference.lnk" "$INSTDIR\API_REFERENCE.md"
    CreateShortCut "$SMPROGRAMS\${APPNAME}\Uninstall.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
    
    ; Create Desktop shortcut
    CreateShortCut "$DESKTOP\${APPNAME}.lnk" "$INSTDIR\parallel-mengene.exe" "" "$INSTDIR\parallel-mengene.exe" 0
    
    ; Create file associations for .pmz files
    WriteRegStr HKCR ".pmz" "" "ParallelMengeneFile"
    WriteRegStr HKCR "ParallelMengeneFile" "" "Parallel Mengene Compressed File"
    WriteRegStr HKCR "ParallelMengeneFile\DefaultIcon" "" "$INSTDIR\parallel-mengene.exe,0"
    WriteRegStr HKCR "ParallelMengeneFile\shell\open\command" "" "$\"$INSTDIR\parallel-mengene.exe$\" decompress $\"%1$\""
    
SectionEnd

; Uninstaller section
Section "uninstall"
    ; Remove files
    Delete "$INSTDIR\parallel-mengene.exe"
    Delete "$INSTDIR\README.md"
    Delete "$INSTDIR\USER_GUIDE.md"
    Delete "$INSTDIR\API_REFERENCE.md"
    Delete "$INSTDIR\uninstall.exe"
    
    ; Remove directory
    RMDir "$INSTDIR"
    
    ; Remove Start Menu shortcuts
    Delete "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk"
    Delete "$SMPROGRAMS\${APPNAME}\User Guide.lnk"
    Delete "$SMPROGRAMS\${APPNAME}\API Reference.lnk"
    Delete "$SMPROGRAMS\${APPNAME}\Uninstall.lnk"
    RMDir "$SMPROGRAMS\${APPNAME}"
    
    ; Remove Desktop shortcut
    Delete "$DESKTOP\${APPNAME}.lnk"
    
    ; Remove from PATH
    EnVar::SetHKCU
    EnVar::DeleteValue "PATH" "$INSTDIR"
    
    ; Remove file associations
    DeleteRegKey HKCR ".pmz"
    DeleteRegKey HKCR "ParallelMengeneFile"
    
    ; Remove registry information
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"
    
SectionEnd

; Functions
Function .onInit
    ; Check if already installed
    ReadRegStr $R0 HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "UninstallString"
    StrCmp $R0 "" done
    
    MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
    "${APPNAME} is already installed. $\n$\nClick 'OK' to remove the \
    previous version or 'Cancel' to cancel this upgrade." \
    IDOK uninst
    Abort
    
    uninst:
        ClearErrors
        ExecWait '$R0 _?=$INSTDIR'
        
        IfErrors no_remove_uninstaller done
        no_remove_uninstaller:
    
    done:
FunctionEnd
