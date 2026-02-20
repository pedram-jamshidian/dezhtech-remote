import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';

// ===========================================================================
// DezhTech Remote - Rebranded from RustDesk
// ===========================================================================

const double kDesktopRemoteTabBarHeight = 28.0;
const int kMainWindowId = 0;
const String kPeerPlatformWindows = "Windows";
const String kPeerPlatformLinux = "Linux";
const String kPeerPlatformMacOS = "Mac OS";
const String kPeerPlatformAndroid = "Android";

// DezhTech Remote App Constants
const String kAppTypeMain = "main";
const String kAppTypeDesktopRemote = "remote";
const String kAppTypeDesktopFileTransfer = "file transfer";
const String kAppTypeDesktopPortForward = "port forward";

const String kTabLabelHomePage = "Home";
const String kTabLabelSettingPage = "Settings";

const String kWindowEventActiveSession = "active_session";
const String kWindowEventActiveDisplaySession = "active_display_session";
const String kWindowEventGetRemoteList = "get_remote_list";
const String kWindowEventGetSessionIdList = "get_session_id_list";
const String kWindowEventGetCachedSessionData = "get_cached_session_data";
const String kWindowEventNewRemoteDesktop = "new_remote_desktop";
const String kWindowEventNewFileTransfer = "new_file_transfer";
const String kWindowEventNewPortForward = "new_port_forward";
const String kWindowEventNewRDP = "new_rdp";
const String kWindowEventRefreshPasswordWarning = "refresh_password_warning";
const String kWindowActionFind = "find";
const String kWindowActionRebuild = "rebuild";
const String kWindowEventShow = "show";
const String kWindowEventHide = "hide";
const String kWindowConnect = "connect";
const String kWindowBumpMouse = "bump_mouse";
const String kWindowEventMoveTabToNewWindow = "move_tab_to_new_window";
const String kWindowEventOpenMonitorSession = "open_monitor_session";
const String kWindowEventRemoteWindowCoords = "get_remote_window_coords";
const String kWindowEventMoveToNearestMonitor = "move_to_nearest_monitor";
const String kWindowRefreshCurrentUser = "refresh_current_user";

const String kWindowUrlActionShow = "show";
const String kWindowUrlActionHide = "hide";
const String kWindowUrlActionClose = "close";

const String kScrollStatusDown = "scroll_down";
const String kScrollStatusUp = "scroll_up";

const String kOptionRefreshVideo = "refresh-video";
const String kOptionScrollStyle = "scroll-style";
const String kOptionScrollStyleBar = "";
const String kOptionScrollStyleMouse = "mouse";
const String kOptionExitOnClose = "exit-on-close";
const String kOptionInstallAsset = "install-asset";
const String kOptionQuickConnectId = "quick-connect-id";
const String kOptionSessionCloseOnDisconnect = "session-close-on-disconnect";
const String kOptionForceScreenRect = "force-screen-rect";

const String kWindowMainWindowOnTop = "main_window_on_top";
const String kWindowGetWindowInfo = "get_window_info";
const String kWindowGetScreenList = "get_screen_list";
const String kWindowDisableGrabKeyboard = "disable_grab_keyboard";
const String kWindowMoveToNearestMonitor = "move_to_nearest_monitor";

const String kPointerEventKindTouch = "touch";
const String kPointerEventKindMouse = "mouse";

const String kKeyTab = "Tab";
const String kKeyShift = "VK_SHIFT";
const String kKeyControl = "VK_CONTROL";
const String kKeyAlt = "VK_MENU";
const String kKeyMeta = "VK_LWIN";
const String kKeyCommand = "VK_COMMAND";
const String kKeySuper = "VK_SUPER";
const String kKeyRawControl = "RAW_CONTROL";
const String kKeyNumLock = "VK_NUMLOCK";
const String kKeyCapsLock = "VK_CAPITAL";
const String kKeyScrollLock = "VK_SCROLL";

const String kValueTrue = "Y";
const String kValueFalse = "";
const String kTrue = "true";
const String kFalse = "false";

const String kMsgBoxTypeConnect = "connecting";
const String kMsgBoxTypeRe = "re-input-password";
const String kMsgBoxTypeTokenExists = "token-exists";

const String kPresetPasswordWarningName = "preset_password_warning";

const int kDefaultDisplayWidth = 1280;
const int kDefaultDisplayHeight = 720;
const int kDefaultDisplayIsVirtualDisplay = 0;
const int kDefaultDisplayOriginalResolutionWidth = 1280;
const int kDefaultDisplayOriginalResolutionHeight = 720;
const int kDefaultDisplayCursorEmbedded = 0;

const int kMobileDefaultDisplayWidth = 720;
const int kMobileDefaultDisplayHeight = 1280;

const int kDesktopMinWidth = 220;
const int kDesktopMinHeight = 220;

const int kDesktopDefaultWidth = 1080;
const int kDesktopDefaultHeight = 720;

const int kDesktopFileTransferMinWidth = 540;
const int kDesktopFileTransferMinHeight = 330;
const int kDesktopFileTransferDefaultWidth = 940;
const int kDesktopFileTransferDefaultHeight = 528;

const int kMobileFileTransferMinWidth = 540;
const int kMobileFileTransferMinHeight = 330;

const int kDesktopPortForwardMinWidth = 540;
const int kDesktopPortForwardMinHeight = 330;
const int kDesktopPortForwardDefaultWidth = 540;
const int kDesktopPortForwardDefaultHeight = 330;

const int kConnectionsPageControlWidth = 400;

const double kTabMenuWidth = 200;
const double kTabBarHeight = 30.0;
const double kTabSwitcherHeight = kTabBarHeight + 16;

const double kModalWidth = 300;
const double kModalRadius = 10;

const double kRemoteToolbarHeight = 48;

const double kBigFontSize = 16;

const double kStrongPasswordLen = 8;

const String kUsePermanentPassword = "use-permanent-password";
const String kUseTemporaryPassword = "use-temporary-password";
const String kUseBothPasswords = "use-both-passwords";

// App information - DezhTech Remote branding
// Note: These are display constants. The actual app name comes from bind.mainGetAppNameSync()

const String kAppHomePage = "https://dezhtech.com";
const String kAppGitHubRepo = "https://github.com/pedram-jamshidian/dezhtech-remote";

// Color Constants for DezhTech Remote Theme
const Color kDezhTechPrimary = Color(0xFF2196F3);  // Blue
const Color kDezhTechAccent = Color(0xFF1976D2);   // Dark Blue
const Color kDezhTechSecondary = Color(0xFF64B5F6); // Light Blue

// Migration from RustDesk
const String kLegacyAppId = "rustdesk";

// Settings keys
const String kOptionOpenNewConn = "option-open-new-conn";
const String kOptionDoubleTapToOpenConn = "option-double-tap-to-open-conn";
const String kOptionShowPasswordOnLaunch = "option-show-password-on-launch";
const String kOptionStopService = "option-stop-service";
const String kOptionDisplayName = "option-display-name";
const String kOptionFilterAbPeers = "option-filter-ab-peers";
const String kOptionHideHelpCards = "hide-help-cards";
const String kOptionDisableGroupPanel = "disable-group-panel";
const String kOptionViewStyle = "option-view-style";
const String kOptionScrollStyle2 = "option-scroll-style";
const String kOptionImageQuality = "option-image-quality";
const String kOptionCodecPreference = "option-codec-preference";
const String kOptionFollowRemoteCursor = "option-follow-remote-cursor";
const String kOptionFollowRemoteWindow = "option-follow-remote-window";
const String kOptionZoomCursor = "option-zoom-cursor";
const String kOptionShowRemoteCursor = "option-show-remote-cursor";
const String kOptionShowQualityMonitor = "option-show-quality-monitor";
const String kOptionDisableAudio = "option-disable-audio";
const String kOptionDisableClipboard = "option-disable-clipboard";
const String kOptionPrivacyMode = "option-privacy-mode";
const String kOptionEnableFileTransfer = "option-enable-file-transfer";
const String kOptionEnableFileTransferPaste = "option-enable-file-transfer-paste";
const String kOptionEnableTcpTunneling = "option-enable-tcp-tunneling";
const String kOptionSendAbortOnClipboard = "option-send-abort-on-clipboard";
const String kOptionAllowSwapKey = "option-allow-swap-key";
const String kOptionCursorAnimation = "option-cursor-animation";
const String kOptionLockAfterSessionEnd = "option-lock-after-session-end";
const String kOptionShowStreamsOnStart = "option-show-streams-on-start";
const String kOptionShowDisplaysAsIndividualWindows = "option-show-displays-as-individual-windows";
const String kOptionUseAllMyDisplaysForTheRemoteSession = "option-use-all-my-displays-for-the-remote-session";
const String kOptionCollapseToolbar = "option-collapse-toolbar";
const String kOptionReverseMouseWheel = "option-reverse-mouse-wheel";
const String kOptionMenubarState = "option-menubar-state";
const String kOptionTabBarState = "option-tab-bar-state";
const String kOptionCursorScale = "option-cursor-scale";
const String kOptionSoftwareCursor = "option-software-cursor";
const String kOptionServerShowRemoteCursor = "option-server-show-remote-cursor";
const String kOptionServerShowRemoteCursorName = "option-server-show-remote-cursor-name";
const String kOptionTextureRender = "option-texture-render";
const String kOptionUnitRectRender = "option-unit-rect-render";
const String kOptionFloatingDragTurns = "option-floating-drag-turns";

const String kOptionInputSource = "option-input-source";
const String kInputSourceDefault = "default";
const String kInputSourceKeyboard = "keyboard";
const String kInputSourceMouse = "mouse";

const String kOptionRelayStrategy = "option-relay-strategy";
const String kRelayStrategyAuto = "relay-strategy-auto";
const String kRelayStrategyDirect = "relay-strategy-direct";
const String kRelayStrategyRelay = "relay-strategy-relay";

const String kOptionConnectViaRelay = "option-connect-via-relay";

const String kOptionLang = "option-lang";
const String kOptionTheme = "option-theme";
const String kOptionThemeLight = "light";
const String kOptionThemeDark = "dark";

const String kOptionToggleViewOnly = "option-toggle-view-only";
const String kOptionShowPopupToolbar = "option-show-popup-toolbar";
const String kOptionShowPopupToolbarForSession = "option-show-popup-toolbar-for-session";
const String kOptionDisplaysAsIndividualWindows = "option-displays-as-individual-windows";
const String kOptionShowDisplaysAsIndividualWindowsTip = "option-show-displays-as-individual-windows-tip";

const String kOptionDirectServer = "option-direct-server";
const String kOptionAccessMode = "option-access-mode";
const String kOptionVerificationMethod = "option-verification-method";
const String kOptionApproveMode = "option-approve-mode";

const String kOptionCustomFPS = "option-custom-fps";
const String kOptionCustomQuality = "option-custom-quality";

const String kDefaultAudioDevice = "";
const String kNullAudioDevice = "null";

const double kDesktopScreenRatio = 4.0 / 3;
const double kMobileScreenRatio = 3.0 / 2;

const String kCommaSplit = ",";

// Old options
const String kOldOptionRemoteCursor = "remote-cursor";
const String kOldOptionZoomCursor = "zoom-cursor";
const String kOldOptionShowQualityMonitor = "show-quality-monitor";
const String kOldOptionDisableAudio = "disable-audio";
const String kOldOptionDisableClipboard = "disable-clipboard";
const String kOldOptionPrivacyMode = "privacy-mode";
const String kOldOptionEnableFileTransfer = "enable-file-transfer";
const String kOldOptionLockAfterSessionEnd = "lock-after-session-end";

// Server model peer info
const String kPeerInfoId = "id";
const String kPeerInfoHostname = "hostname";
const String kPeerInfoUsername = "username";
const String kPeerInfoPlatform = "platform";
const String kPeerInfoAliasOrId = "alias_or_id";
const String kPeerInfoForceAlwaysRelay = "forceAlwaysRelay";
const String kPeerInfoRdp = "rdp";

// Clipboard format
const String kClipboardFormatTextPlain = "text/plain";
const String kClipboardFormatTextHtml = "text/html";
const String kClipboardFormatFilePaths = "file-paths";
const String kClipboardFormatImagePng = "image/png";

// For connection page
const double kMinConnWidth = 300;
const double kMinConnHeight = 500;
const double kAbPeerCardWidth = 220;
const double kAbPeerCardHeight = 104;

// For connection animation
const Duration kDefaultAnimDuration = Duration(milliseconds: 120);
const Duration kLongAnimDuration = Duration(milliseconds: 240);
const Duration kExtraLongAnimDuration = Duration(milliseconds: 360);

// For keyboard interactive mode
const String kKeyboardModeLegacy = "legacy";
const String kKeyboardModeMap = "map";
const String kKeyboardModeTranslate = "translate";

// For version comparison
const String kMinVersionForTextureRender = "1.2.0";
const String kMinVersionForTextureRenderInvoker = "1.2.4";
const String kMinVersionForControlDesktopFromApp = "1.2.7";
const String kMinVersionForVirtualDisplay = "1.2.7";

// For settings page
const double kSettingsPageMaxWidth = 700;
const double kSettingsScrollbarPadding = 16;

// For peer card
const double kDesktopPeerCardWidth = 220;
const double kDesktopPeerCardHeight = 100;
const double kMobilePeerCardWidth = 182;
const double kMobilePeerCardHeight = 100;

// For peer menu
const double kPeerContextMenuWidth = 200;

// For virtual display
const String kVirtualDisplayResolution = "virtual_display_resolution";
const String kVirtualDisplayFps = "virtual_display_fps";
const int kVirtualDisplayDefaultFps = 30;
const int kVirtualDisplayMaxFps = 120;
const String kVirtualDisplayDefaultResolution = "1920x1080";

// For file transfer
const int kFileTransferBlockSize = 1024 * 1024 * 8;

// For message box
const int kMsgBoxTimeout = 20;

// For loading logo
const double kLogoAspectRatio = 3.14;

// Window types
enum WindowType {
  Main,
  RemoteDesktop,
  FileTransfer,
  PortForward,
  Unknown,
}

// Session types
enum ConnType {
  defaultConn,
  fileTransfer,
  portForward,
  rdp,
  viewCamera,
  terminal,
}

extension ConnTypeExtension on ConnType {
  bool get isFileTransfer => this == ConnType.fileTransfer;
  bool get isPortForward => this == ConnType.portForward;
  bool get isRDP => this == ConnType.rdp;
  bool get isViewCamera => this == ConnType.viewCamera;
  bool get isTerminal => this == ConnType.terminal;
  bool get isDesktop => this == ConnType.defaultConn;
}

bool isDesktop = false;
bool isWeb = false;
bool isWebDesktop = false;
bool isMobile = false;
bool isAndroid = false;
bool isIOS = false;
bool isWindows = false;
bool isLinux = false;
bool isMacOS = false;

void updatePlatformInfo() {
  try {
    isWindows = Platform.isWindows;
    isLinux = Platform.isLinux;
    isMacOS = Platform.isMacOS;
    isAndroid = Platform.isAndroid;
    isIOS = Platform.isIOS;
    isDesktop = isWindows || isLinux || isMacOS;
    isMobile = isAndroid || isIOS;
  } catch (e) {
    // This should only happen on web
    isWeb = true;
  }
}

// Text input action maps
Map<String, TextInputAction> textInputActionMap = {
  'done': TextInputAction.done,
  'go': TextInputAction.go,
  'next': TextInputAction.next,
  'previous': TextInputAction.previous,
  'search': TextInputAction.search,
  'send': TextInputAction.send,
  'none': TextInputAction.none,
  'unspecified': TextInputAction.unspecified,
};

// Settings tab key
enum SettingsTabKey {
  general,
  safety,
  network,
  display,
  account,
  about,
  plugin,
}

const Map<SettingsTabKey, String> settingsTabKeyToName = {
  SettingsTabKey.general: 'General',
  SettingsTabKey.safety: 'Security',
  SettingsTabKey.network: 'Network',
  SettingsTabKey.display: 'Display',
  SettingsTabKey.account: 'Account',
  SettingsTabKey.about: 'About',
  SettingsTabKey.plugin: 'Plugin',
};
