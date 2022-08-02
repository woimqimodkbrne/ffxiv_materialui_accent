#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::CString;

#[path = "bindings.rs"]
pub mod sys;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Col {
	Text = 0,
	TextDisabled = 1,
	WindowBg = 2,
	ChildBg = 3,
	PopupBg = 4,
	Border = 5,
	BorderShadow = 6,
	FrameBg = 7,
	FrameBgHovered = 8,
	FrameBgActive = 9,
	TitleBg = 10,
	TitleBgActive = 11,
	TitleBgCollapsed = 12,
	MenuBarBg = 13,
	ScrollbarBg = 14,
	ScrollbarGrab = 15,
	ScrollbarGrabHovered = 16,
	ScrollbarGrabActive = 17,
	CheckMark = 18,
	SliderGrab = 19,
	SliderGrabActive = 20,
	Button = 21,
	ButtonHovered = 22,
	ButtonActive = 23,
	Header = 24,
	HeaderHovered = 25,
	HeaderActive = 26,
	Separator = 27,
	SeparatorHovered = 28,
	SeparatorActive = 29,
	ResizeGrip = 30,
	ResizeGripHovered = 31,
	ResizeGripActive = 32,
	Tab = 33,
	TabHovered = 34,
	TabActive = 35,
	TabUnfocused = 36,
	TabUnfocusedActive = 37,
	PlotLines = 38,
	PlotLinesHovered = 39,
	PlotHistogram = 40,
	PlotHistogramHovered = 41,
	TableHeaderBg = 42,
	TableBorderStrong = 43,
	TableBorderLight = 44,
	TableRowBg = 45,
	TableRowBgAlt = 46,
	TextSelectedBg = 47,
	DragDropTarget = 48,
	NavHighlight = 49,
	NavWindowingHighlight = 50,
	NavWindowingDimBg = 51,
	ModalWindowDimBg = 52,
	COUNT = 53,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cond {
	None = 0,
	Always = 1,
	Once = 2,
	FirstUseEver = 4,
	Appearing = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
	Float = 8,
	Double = 9,
	COUNT = 10,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Dir {
	Left = 0,
	Right = 1,
	Up = 2,
	Down = 3,
	COUNT = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
	None = 0,
	Tab = 512,
	LeftArrow = 513,
	RightArrow = 514,
	UpArrow = 515,
	DownArrow = 516,
	PageUp = 517,
	PageDown = 518,
	Home = 519,
	End = 520,
	Insert = 521,
	Delete = 522,
	Backspace = 523,
	Space = 524,
	Enter = 525,
	Escape = 526,
	LeftCtrl = 527,
	LeftShift = 528,
	LeftAlt = 529,
	LeftSuper = 530,
	RightCtrl = 531,
	RightShift = 532,
	RightAlt = 533,
	RightSuper = 534,
	Menu = 535,
	A = 546,
	B = 547,
	C = 548,
	D = 549,
	E = 550,
	F = 551,
	G = 552,
	H = 553,
	I = 554,
	J = 555,
	K = 556,
	L = 557,
	M = 558,
	N = 559,
	O = 560,
	P = 561,
	Q = 562,
	R = 563,
	S = 564,
	T = 565,
	U = 566,
	V = 567,
	W = 568,
	X = 569,
	Y = 570,
	Z = 571,
	Apostrophe = 584,
	Comma = 585,
	Minus = 586,
	Period = 587,
	Slash = 588,
	Semicolon = 589,
	Equal = 590,
	LeftBracket = 591,
	Backslash = 592,
	RightBracket = 593,
	GraveAccent = 594,
	CapsLock = 595,
	ScrollLock = 596,
	NumLock = 597,
	PrintScreen = 598,
	Pause = 599,
	KeypadDecimal = 610,
	KeypadDivide = 611,
	KeypadMultiply = 612,
	KeypadSubtract = 613,
	KeypadAdd = 614,
	KeypadEnter = 615,
	KeypadEqual = 616,
	GamepadStart = 617,
	GamepadBack = 618,
	GamepadFaceUp = 619,
	GamepadFaceDown = 620,
	GamepadFaceLeft = 621,
	GamepadFaceRight = 622,
	GamepadDpadUp = 623,
	GamepadDpadDown = 624,
	GamepadDpadLeft = 625,
	GamepadDpadRight = 626,
	GamepadLStickUp = 633,
	GamepadLStickDown = 634,
	GamepadLStickLeft = 635,
	GamepadLStickRight = 636,
	GamepadRStickUp = 637,
	GamepadRStickDown = 638,
	GamepadRStickLeft = 639,
	GamepadRStickRight = 640,
	ModCtrl = 641,
	ModShift = 642,
	ModAlt = 643,
	ModSuper = 644,
	COUNT = 645,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NavInput {
	Activate = 0,
	Cancel = 1,
	Input = 2,
	Menu = 3,
	DpadLeft = 4,
	DpadRight = 5,
	DpadUp = 6,
	DpadDown = 7,
	LStickLeft = 8,
	LStickRight = 9,
	LStickUp = 10,
	LStickDown = 11,
	FocusPrev = 12,
	FocusNext = 13,
	TweakSlow = 14,
	TweakFast = 15,
	COUNT = 20,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
	Left = 0,
	Right = 1,
	Middle = 2,
	COUNT = 5,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseCursor {
	Arrow = 0,
	TextInput = 1,
	ResizeAll = 2,
	ResizeNS = 3,
	ResizeEW = 4,
	ResizeNESW = 5,
	ResizeNWSE = 6,
	Hand = 7,
	NotAllowed = 8,
	COUNT = 9,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SortDirection {
	None = 0,
	Ascending = 1,
	Descending = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StyleVar {
	Alpha = 0,
	DisabledAlpha = 1,
	WindowPadding = 2,
	WindowRounding = 3,
	WindowBorderSize = 4,
	WindowMinSize = 5,
	WindowTitleAlign = 6,
	ChildRounding = 7,
	ChildBorderSize = 8,
	PopupRounding = 9,
	PopupBorderSize = 10,
	FramePadding = 11,
	FrameRounding = 12,
	FrameBorderSize = 13,
	ItemSpacing = 14,
	ItemInnerSpacing = 15,
	IndentSpacing = 16,
	CellPadding = 17,
	ScrollbarSize = 18,
	ScrollbarRounding = 19,
	GrabMinSize = 20,
	GrabRounding = 21,
	TabRounding = 22,
	ButtonTextAlign = 23,
	SelectableTextAlign = 24,
	COUNT = 25,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TableBgTarget {
	None = 0,
	CellBg = 3,
}

bitflags::bitflags!{pub struct DrawFlags: i32 {
	const None = 0;
	const Closed = 1;
	const RoundCornersTopLeft = 16;
	const RoundCornersTopRight = 32;
	const RoundCornersBottomLeft = 64;
	const RoundCornersBottomRight = 128;
	const RoundCornersNone = 256;
	const RoundCornersTop = 48;
	const RoundCornersBottom = 192;
	const RoundCornersLeft = 80;
	const RoundCornersRight = 160;
	const RoundCornersAll = 240;
}}

bitflags::bitflags!{pub struct ListFlags: i32 {
	const None = 0;
	const AntiAliasedLines = 1;
	const AntiAliasedLinesUseTex = 2;
	const AntiAliasedFill = 4;
	const AllowVtxOffset = 8;
}}

bitflags::bitflags!{pub struct AtlasFlags: i32 {
	const None = 0;
	const NoPowerOfTwoHeight = 1;
	const NoMouseCursors = 2;
	const NoBakedLines = 4;
}}

bitflags::bitflags!{pub struct BackendFlags: i32 {
	const None = 0;
	const HasGamepad = 1;
	const HasMouseCursors = 2;
	const HasSetMousePos = 4;
	const RendererHasVtxOffset = 8;
}}

bitflags::bitflags!{pub struct ButtonFlags: i32 {
	const None = 0;
	const MouseButtonLeft = 1;
	const MouseButtonRight = 2;
	const MouseButtonMiddle = 4;
}}

bitflags::bitflags!{pub struct ColorEditFlags: i32 {
	const None = 0;
	const NoAlpha = 2;
	const NoPicker = 4;
	const NoOptions = 8;
	const NoSmallPreview = 16;
	const NoInputs = 32;
	const NoTooltip = 64;
	const NoLabel = 128;
	const NoSidePreview = 256;
	const NoDragDrop = 512;
	const NoBorder = 1024;
	const AlphaBar = 65536;
	const AlphaPreview = 131072;
	const AlphaPreviewHalf = 262144;
	const HDR = 524288;
	const DisplayRGB = 1048576;
	const DisplayHSV = 2097152;
	const DisplayHex = 4194304;
	const Float = 16777216;
	const PickerHueBar = 33554432;
	const PickerHueWheel = 67108864;
	const InputRGB = 134217728;
	const InputHSV = 268435456;
}}

bitflags::bitflags!{pub struct ConfigFlags: i32 {
	const None = 0;
	const NavEnableKeyboard = 1;
	const NavEnableGamepad = 2;
	const NavEnableSetMousePos = 4;
	const NavNoCaptureKeyboard = 8;
	const NoMouse = 16;
	const NoMouseCursorChange = 32;
	const IsSRGB = 1048576;
	const IsTouchScreen = 2097152;
}}

bitflags::bitflags!{pub struct ComboFlags: i32 {
	const None = 0;
	const PopupAlignLeft = 1;
	const HeightSmall = 2;
	const HeightRegular = 4;
	const HeightLarge = 8;
	const HeightLargest = 16;
	const NoArrowButton = 32;
	const NoPreview = 64;
}}

bitflags::bitflags!{pub struct DragDropFlags: i32 {
	const None = 0;
	const SourceNoPreviewTooltip = 1;
	const SourceNoDisableHover = 2;
	const SourceNoHoldToOpenOthers = 4;
	const SourceAllowNullID = 8;
	const SourceExtern = 16;
	const SourceAutoExpirePayload = 32;
	const AcceptBeforeDelivery = 1024;
	const AcceptNoDrawDefaultRect = 2048;
	const AcceptNoPreviewTooltip = 4096;
	const AcceptPeekOnly = 3072;
}}

bitflags::bitflags!{pub struct FocusedFlags: i32 {
	const None = 0;
	const ChildWindows = 1;
	const RootWindow = 2;
	const AnyWindow = 4;
	const NoPopupHierarchy = 8;
	const RootAndChildWindows = 3;
}}

bitflags::bitflags!{pub struct HoveredFlags: i32 {
	const None = 0;
	const ChildWindows = 1;
	const RootWindow = 2;
	const AnyWindow = 4;
	const NoPopupHierarchy = 8;
	const AllowWhenBlockedByPopup = 32;
	const AllowWhenBlockedByActiveItem = 128;
	const AllowWhenOverlapped = 256;
	const AllowWhenDisabled = 512;
	const NoNavOverride = 1024;
	const RectOnly = 416;
	const RootAndChildWindows = 3;
}}

bitflags::bitflags!{pub struct InputTextFlags: i32 {
	const None = 0;
	const CharsDecimal = 1;
	const CharsHexadecimal = 2;
	const CharsUppercase = 4;
	const CharsNoBlank = 8;
	const AutoSelectAll = 16;
	const EnterReturnsTrue = 32;
	const CallbackCompletion = 64;
	const CallbackHistory = 128;
	const CallbackAlways = 256;
	const CallbackCharFilter = 512;
	const AllowTabInput = 1024;
	const CtrlEnterForNewLine = 2048;
	const NoHorizontalScroll = 4096;
	const AlwaysOverwrite = 8192;
	const ReadOnly = 16384;
	const Password = 32768;
	const NoUndoRedo = 65536;
	const CharsScientific = 131072;
	const CallbackResize = 262144;
	const CallbackEdit = 524288;
}}

bitflags::bitflags!{pub struct ModFlags: i32 {
	const None = 0;
	const Ctrl = 1;
	const Shift = 2;
	const Alt = 4;
	const Super = 8;
}}

bitflags::bitflags!{pub struct PopupFlags: i32 {
	const None = 0;
	const MouseButtonLeft = 0;
	const MouseButtonRight = 1;
	const MouseButtonMiddle = 2;
	const NoOpenOverExistingPopup = 32;
	const NoOpenOverItems = 64;
	const AnyPopupId = 128;
	const AnyPopupLevel = 256;
	const AnyPopup = 384;
}}

bitflags::bitflags!{pub struct SelectableFlags: i32 {
	const None = 0;
	const DontClosePopups = 1;
	const SpanAllColumns = 2;
	const AllowDoubleClick = 4;
	const Disabled = 8;
	const AllowItemOverlap = 16;
}}

bitflags::bitflags!{pub struct SliderFlags: i32 {
	const None = 0;
	const AlwaysClamp = 16;
	const Logarithmic = 32;
	const NoRoundToFormat = 64;
	const NoInput = 128;
}}

bitflags::bitflags!{pub struct TabBarFlags: i32 {
	const None = 0;
	const Reorderable = 1;
	const AutoSelectNewTabs = 2;
	const TabListPopupButton = 4;
	const NoCloseWithMiddleMouseButton = 8;
	const NoTabListScrollingButtons = 16;
	const NoTooltip = 32;
	const FittingPolicyResizeDown = 64;
	const FittingPolicyScroll = 128;
}}

bitflags::bitflags!{pub struct TabItemFlags: i32 {
	const None = 0;
	const UnsavedDocument = 1;
	const SetSelected = 2;
	const NoCloseWithMiddleMouseButton = 4;
	const NoPushId = 8;
	const NoTooltip = 16;
	const NoReorder = 32;
	const Leading = 64;
	const Trailing = 128;
}}

bitflags::bitflags!{pub struct TableFlags: i32 {
	const None = 0;
	const Resizable = 1;
	const Reorderable = 2;
	const Hideable = 4;
	const Sortable = 8;
	const NoSavedSettings = 16;
	const ContextMenuInBody = 32;
	const RowBg = 64;
	const BordersInnerH = 128;
	const BordersOuterH = 256;
	const BordersInnerV = 512;
	const BordersOuterV = 1024;
	const BordersH = 384;
	const BordersV = 1536;
	const BordersInner = 640;
	const BordersOuter = 1280;
	const Borders = 1920;
	const NoBordersInBody = 2048;
	const NoBordersInBodyUntilResize = 4096;
	const SizingFixedFit = 8192;
	const SizingFixedSame = 16384;
	const SizingStretchProp = 24576;
	const SizingStretchSame = 32768;
	const NoHostExtendX = 65536;
	const NoHostExtendY = 131072;
	const NoKeepColumnsVisible = 262144;
	const PreciseWidths = 524288;
	const NoClip = 1048576;
	const PadOuterX = 2097152;
	const NoPadOuterX = 4194304;
	const NoPadInnerX = 8388608;
	const ScrollX = 16777216;
	const ScrollY = 33554432;
	const SortMulti = 67108864;
	const SortTristate = 134217728;
}}

bitflags::bitflags!{pub struct TableColumnFlags: i32 {
	const None = 0;
	const Disabled = 1;
	const DefaultHide = 2;
	const DefaultSort = 4;
	const WidthStretch = 8;
	const WidthFixed = 16;
	const NoResize = 32;
	const NoReorder = 64;
	const NoHide = 128;
	const NoClip = 256;
	const NoSort = 512;
	const NoSortAscending = 1024;
	const NoSortDescending = 2048;
	const NoHeaderLabel = 4096;
	const NoHeaderWidth = 8192;
	const PreferSortAscending = 16384;
	const PreferSortDescending = 32768;
	const IndentEnable = 65536;
	const IndentDisable = 131072;
	const IsEnabled = 16777216;
	const IsVisible = 33554432;
	const IsSorted = 67108864;
	const IsHovered = 134217728;
}}

bitflags::bitflags!{pub struct TableRowFlags: i32 {
	const None = 0;
	const Headers = 1;
}}

bitflags::bitflags!{pub struct TreeNodeFlags: i32 {
	const None = 0;
	const Selected = 1;
	const Framed = 2;
	const AllowItemOverlap = 4;
	const NoTreePushOnOpen = 8;
	const NoAutoOpenOnLog = 16;
	const DefaultOpen = 32;
	const OpenOnDoubleClick = 64;
	const OpenOnArrow = 128;
	const Leaf = 256;
	const Bullet = 512;
	const FramePadding = 1024;
	const SpanAvailWidth = 2048;
	const SpanFullWidth = 4096;
	const NavLeftJumpsBackHere = 8192;
	const CollapsingHeader = 26;
}}

bitflags::bitflags!{pub struct ViewportFlags: i32 {
	const None = 0;
	const IsPlatformWindow = 1;
	const IsPlatformMonitor = 2;
	const OwnedByApp = 4;
}}

bitflags::bitflags!{pub struct WindowFlags: i32 {
	const None = 0;
	const NoTitleBar = 1;
	const NoResize = 2;
	const NoMove = 4;
	const NoScrollbar = 8;
	const NoScrollWithMouse = 16;
	const NoCollapse = 32;
	const AlwaysAutoResize = 64;
	const NoBackground = 128;
	const NoSavedSettings = 256;
	const NoMouseInputs = 512;
	const MenuBar = 1024;
	const HorizontalScrollbar = 2048;
	const NoFocusOnAppearing = 4096;
	const NoBringToFrontOnFocus = 8192;
	const AlwaysVerticalScrollbar = 16384;
	const AlwaysHorizontalScrollbar = 32768;
	const AlwaysUseWindowPadding = 65536;
	const NoNavInputs = 262144;
	const NoNavFocus = 524288;
	const UnsavedDocument = 1048576;
	const NoNav = 786432;
	const NoDecoration = 43;
	const NoInputs = 786944;
	const NavFlattened = 8388608;
	const ChildWindow = 16777216;
	const Tooltip = 33554432;
	const Popup = 67108864;
	const Modal = 134217728;
	const ChildMenu = 268435456;
}}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LayoutType {
	Horizontal = 0,
	Vertical = 1,
}

bitflags::bitflags!{pub struct ActivateFlags: i32 {
	const None = 0;
	const PreferInput = 1;
	const PreferTweak = 2;
	const TryToPreserveState = 4;
}}

bitflags::bitflags!{pub struct DebugLogFlags: i32 {
	const None = 0;
	const EventActiveId = 1;
	const EventFocus = 2;
	const EventPopup = 4;
	const EventNav = 8;
	const EventIO = 16;
	const OutputToTTY = 1024;
}}

bitflags::bitflags!{pub struct ItemFlags: i32 {
	const None = 0;
	const NoTabStop = 1;
	const ButtonRepeat = 2;
	const Disabled = 4;
	const NoNav = 8;
	const NoNavDefaultFocus = 16;
	const SelectableDontClosePopup = 32;
	const MixedValue = 64;
	const ReadOnly = 128;
	const Inputable = 256;
}}

bitflags::bitflags!{pub struct ItemStatusFlags: i32 {
	const None = 0;
	const HoveredRect = 1;
	const HasDisplayRect = 2;
	const Edited = 4;
	const ToggledSelection = 8;
	const ToggledOpen = 16;
	const HasDeactivated = 32;
	const Deactivated = 64;
	const HoveredWindow = 128;
	const FocusedByTabbing = 256;
}}

bitflags::bitflags!{pub struct OldColumnFlags: i32 {
	const None = 0;
	const NoBorder = 1;
	const NoResize = 2;
	const NoPreserveWidths = 4;
	const NoForceWithinWindow = 8;
	const GrowParentContentsSize = 16;
}}

bitflags::bitflags!{pub struct NavHighlightFlags: i32 {
	const None = 0;
	const TypeDefault = 1;
	const TypeThin = 2;
	const AlwaysDraw = 4;
	const NoRounding = 8;
}}

bitflags::bitflags!{pub struct NavDirSourceFlags: i32 {
	const None = 0;
	const RawKeyboard = 1;
	const Keyboard = 2;
	const PadDPad = 4;
	const PadLStick = 8;
}}

bitflags::bitflags!{pub struct NavMoveFlags: i32 {
	const None = 0;
	const LoopX = 1;
	const LoopY = 2;
	const WrapX = 4;
	const WrapY = 8;
	const AllowCurrentNavId = 16;
	const AlsoScoreVisibleSet = 32;
	const ScrollToEdgeY = 64;
	const Forwarded = 128;
	const DebugNoResult = 256;
	const FocusApi = 512;
	const Tabbing = 1024;
	const Activate = 2048;
	const DontSetNavHighlight = 4096;
}}

bitflags::bitflags!{pub struct NextItemDataFlags: i32 {
	const None = 0;
	const HasWidth = 1;
	const HasOpen = 2;
}}

bitflags::bitflags!{pub struct NextWindowDataFlags: i32 {
	const None = 0;
	const HasPos = 1;
	const HasSize = 2;
	const HasContentSize = 4;
	const HasCollapsed = 8;
	const HasSizeConstraint = 16;
	const HasFocus = 32;
	const HasBgAlpha = 64;
	const HasScroll = 128;
}}

bitflags::bitflags!{pub struct ScrollFlags: i32 {
	const None = 0;
	const KeepVisibleEdgeX = 1;
	const KeepVisibleEdgeY = 2;
	const KeepVisibleCenterX = 4;
	const KeepVisibleCenterY = 8;
	const AlwaysCenterX = 16;
	const AlwaysCenterY = 32;
	const NoScrollParent = 64;
}}

bitflags::bitflags!{pub struct SeparatorFlags: i32 {
	const None = 0;
	const Horizontal = 1;
	const Vertical = 2;
	const SpanAllColumns = 4;
}}

bitflags::bitflags!{pub struct TextFlags: i32 {
	const None = 0;
	const NoWidthForLargeClippedText = 1;
}}

bitflags::bitflags!{pub struct TooltipFlags: i32 {
	const None = 0;
	const OverridePreviousTooltip = 1;
}}

bitflags::bitflags!{pub struct InputTextFlagsPrivate: i32 {
	const Multiline = 67108864;
	const NoMarkEdited = 134217728;
	const MergedItem = 268435456;
}}

bitflags::bitflags!{pub struct ButtonFlagsPrivate: i32 {
	const PressedOnClick = 16;
	const PressedOnClickRelease = 32;
	const PressedOnClickReleaseAnywhere = 64;
	const PressedOnRelease = 128;
	const PressedOnDoubleClick = 256;
	const PressedOnDragDropHold = 512;
	const Repeat = 1024;
	const FlattenChildren = 2048;
	const AllowItemOverlap = 4096;
	const DontClosePopups = 8192;
	const AlignTextBaseLine = 32768;
	const NoKeyModifiers = 65536;
	const NoHoldingActiveId = 131072;
	const NoNavFocus = 262144;
	const NoHoveredOnFocus = 524288;
}}

bitflags::bitflags!{pub struct ComboFlagsPrivate: i32 {
	const CustomPreview = 1048576;
}}

bitflags::bitflags!{pub struct SliderFlagsPrivate: i32 {
	const Vertical = 1048576;
	const ReadOnly = 2097152;
}}

bitflags::bitflags!{pub struct SelectableFlagsPrivate: i32 {
	const NoHoldingActiveID = 1048576;
	const SelectOnNav = 2097152;
	const SelectOnClick = 4194304;
	const SelectOnRelease = 8388608;
	const SpanAvailWidth = 16777216;
	const DrawHoveredWhenHeld = 33554432;
	const SetNavIdOnHover = 67108864;
	const NoPadWithHalfSpacing = 134217728;
}}

bitflags::bitflags!{pub struct TreeNodeFlagsPrivate: i32 {
	const ClipLabelForTrailingButton = 1048576;
}}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataTypePrivate {
	String = 11,
	Pointer = 12,
	ID = 13,
}

bitflags::bitflags!{pub struct TabBarFlagsPrivate: i32 {
	const DockNode = 1048576;
	const IsFocused = 2097152;
	const SaveSettings = 4194304;
}}

bitflags::bitflags!{pub struct TabItemFlagsPrivate: i32 {
	const NoCloseButton = 1048576;
	const Button = 2097152;
}}

pub fn create_context(shared_font_atlas: &mut sys::ImFontAtlas) -> *mut sys::ImGuiContext {
	unsafe{sys::igCreateContext(shared_font_atlas)}
}

pub fn destroy_context(ctx: &mut sys::ImGuiContext) {
	unsafe{sys::igDestroyContext(ctx)}
}

pub fn get_current_context() -> *mut sys::ImGuiContext {
	unsafe{sys::igGetCurrentContext()}
}

pub fn set_current_context(ctx: &mut sys::ImGuiContext) {
	unsafe{sys::igSetCurrentContext(ctx)}
}

pub fn get_io() -> *mut sys::ImGuiIO {
	unsafe{sys::igGetIO()}
}

pub fn get_style() -> &'static mut sys::ImGuiStyle {
	unsafe{&mut *sys::igGetStyle()}
}

pub fn new_frame() {
	unsafe{sys::igNewFrame()}
}

pub fn end_frame() {
	unsafe{sys::igEndFrame()}
}

pub fn render() {
	unsafe{sys::igRender()}
}

pub fn get_draw_data() -> *mut sys::ImDrawData {
	unsafe{sys::igGetDrawData()}
}

pub fn show_demo_window(p_open: &mut bool) {
	unsafe{sys::igShowDemoWindow(p_open)}
}

pub fn show_metrics_window(p_open: &mut bool) {
	unsafe{sys::igShowMetricsWindow(p_open)}
}

pub fn show_about_window(p_open: &mut bool) {
	unsafe{sys::igShowAboutWindow(p_open)}
}

pub fn show_style_editor(ref_: &mut sys::ImGuiStyle) {
	unsafe{sys::igShowStyleEditor(ref_)}
}

pub fn show_style_selector(label: &str) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igShowStyleSelector(label_.as_ptr())}
}

pub fn show_font_selector(label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igShowFontSelector(label_.as_ptr())}
}

pub fn show_user_guide() {
	unsafe{sys::igShowUserGuide()}
}

pub fn get_version() -> *const ::std::os::raw::c_char {
	unsafe{sys::igGetVersion()}
}

pub fn style_colors_dark(dst: &mut sys::ImGuiStyle) {
	unsafe{sys::igStyleColorsDark(dst)}
}

pub fn style_colors_light(dst: &mut sys::ImGuiStyle) {
	unsafe{sys::igStyleColorsLight(dst)}
}

pub fn style_colors_classic(dst: &mut sys::ImGuiStyle) {
	unsafe{sys::igStyleColorsClassic(dst)}
}

pub fn begin(name: &str, p_open: &mut bool, flags: WindowFlags) -> bool {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igBegin(name_.as_ptr(), p_open, flags.bits)}
}

pub fn end() {
	unsafe{sys::igEnd()}
}

pub fn begin_child(str_id: &str, size: [f32; 2], border: bool, flags: WindowFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginChild_Str(str_id_.as_ptr(), size, border, flags.bits)}
}

pub fn begin_child2(id: sys::ImGuiID, size: [f32; 2], border: bool, flags: WindowFlags) -> bool {
	unsafe{sys::igBeginChild_ID(id, size, border, flags.bits)}
}

pub fn end_child() {
	unsafe{sys::igEndChild()}
}

pub fn is_window_appearing() -> bool {
	unsafe{sys::igIsWindowAppearing()}
}

pub fn is_window_collapsed() -> bool {
	unsafe{sys::igIsWindowCollapsed()}
}

pub fn is_window_focused(flags: FocusedFlags) -> bool {
	unsafe{sys::igIsWindowFocused(flags.bits)}
}

pub fn is_window_hovered(flags: HoveredFlags) -> bool {
	unsafe{sys::igIsWindowHovered(flags.bits)}
}

pub fn get_window_draw_list() -> DrawList {
	DrawList {
		drawlist: unsafe{sys::igGetWindowDrawList()},
		vtx_offset: 0,
	}
}

pub fn get_window_pos() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetWindowPos(&mut r)}
	r
}

pub fn get_window_size() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetWindowSize(&mut r)}
	r
}

pub fn get_window_width() -> f32 {
	unsafe{sys::igGetWindowWidth()}
}

pub fn get_window_height() -> f32 {
	unsafe{sys::igGetWindowHeight()}
}

pub fn set_next_window_pos(pos: [f32; 2], cond: Cond, pivot: [f32; 2]) {
	unsafe{sys::igSetNextWindowPos(pos, cond as i32, pivot)}
}

pub fn set_next_window_size(size: [f32; 2], cond: Cond) {
	unsafe{sys::igSetNextWindowSize(size, cond as i32)}
}

pub fn set_next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2], custom_callback: sys::ImGuiSizeCallback, custom_callback_data: &mut ::std::os::raw::c_void) {
	unsafe{sys::igSetNextWindowSizeConstraints(size_min, size_max, custom_callback, custom_callback_data)}
}

pub fn set_next_window_content_size(size: [f32; 2]) {
	unsafe{sys::igSetNextWindowContentSize(size)}
}

pub fn set_next_window_collapsed(collapsed: bool, cond: Cond) {
	unsafe{sys::igSetNextWindowCollapsed(collapsed, cond as i32)}
}

pub fn set_next_window_focus() {
	unsafe{sys::igSetNextWindowFocus()}
}

pub fn set_next_window_bg_alpha(alpha: f32) {
	unsafe{sys::igSetNextWindowBgAlpha(alpha)}
}

pub fn set_window_pos__vec2(pos: [f32; 2], cond: Cond) {
	unsafe{sys::igSetWindowPos_Vec2(pos, cond as i32)}
}

pub fn set_window_size__vec2(size: [f32; 2], cond: Cond) {
	unsafe{sys::igSetWindowSize_Vec2(size, cond as i32)}
}

pub fn set_window_collapsed__bool(collapsed: bool, cond: Cond) {
	unsafe{sys::igSetWindowCollapsed_Bool(collapsed, cond as i32)}
}

pub fn set_window_focus__nil() {
	unsafe{sys::igSetWindowFocus_Nil()}
}

pub fn set_window_font_scale(scale: f32) {
	unsafe{sys::igSetWindowFontScale(scale)}
}

pub fn set_window_pos__str(name: &str, pos: [f32; 2], cond: Cond) {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igSetWindowPos_Str(name_.as_ptr(), pos, cond as i32)}
}

pub fn set_window_size__str(name: &str, size: [f32; 2], cond: Cond) {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igSetWindowSize_Str(name_.as_ptr(), size, cond as i32)}
}

pub fn set_window_collapsed__str(name: &str, collapsed: bool, cond: Cond) {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igSetWindowCollapsed_Str(name_.as_ptr(), collapsed, cond as i32)}
}

pub fn set_window_focus__str(name: &str) {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igSetWindowFocus_Str(name_.as_ptr())}
}

pub fn get_content_region_avail() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetContentRegionAvail(&mut r)}
	r
}

pub fn get_content_region_max() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetContentRegionMax(&mut r)}
	r
}

pub fn get_window_content_region_min() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetWindowContentRegionMin(&mut r)}
	r
}

pub fn get_window_content_region_max() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetWindowContentRegionMax(&mut r)}
	r
}

// pub fn get_window_content_region_width() -> f32 {
// 	unsafe{sys::igGetWindowContentRegionWidth()}
// }

pub fn get_scroll_x() -> f32 {
	unsafe{sys::igGetScrollX()}
}

pub fn get_scroll_y() -> f32 {
	unsafe{sys::igGetScrollY()}
}

pub fn set_scroll_x__float(scroll_x: f32) {
	unsafe{sys::igSetScrollX_Float(scroll_x)}
}

pub fn set_scroll_y__float(scroll_y: f32) {
	unsafe{sys::igSetScrollY_Float(scroll_y)}
}

pub fn get_scroll_max_x() -> f32 {
	unsafe{sys::igGetScrollMaxX()}
}

pub fn get_scroll_max_y() -> f32 {
	unsafe{sys::igGetScrollMaxY()}
}

pub fn set_scroll_here_x(center_x_ratio: f32) {
	unsafe{sys::igSetScrollHereX(center_x_ratio)}
}

pub fn set_scroll_here_y(center_y_ratio: f32) {
	unsafe{sys::igSetScrollHereY(center_y_ratio)}
}

pub fn set_scroll_from_pos_x__float(local_x: f32, center_x_ratio: f32) {
	unsafe{sys::igSetScrollFromPosX_Float(local_x, center_x_ratio)}
}

pub fn set_scroll_from_pos_y__float(local_y: f32, center_y_ratio: f32) {
	unsafe{sys::igSetScrollFromPosY_Float(local_y, center_y_ratio)}
}

pub fn push_font(font: &mut sys::ImFont) {
	unsafe{sys::igPushFont(font)}
}

pub fn pop_font() {
	unsafe{sys::igPopFont()}
}

pub fn push_style_color(idx: Col, col: u32) {
	unsafe{sys::igPushStyleColor_U32(idx as i32, col)}
}

pub fn push_style_color2(idx: Col, col: [f32; 4]) {
	unsafe{sys::igPushStyleColor_Vec4(idx as i32, col)}
}

pub fn pop_style_color(count: i32) {
	unsafe{sys::igPopStyleColor(count)}
}

pub fn push_style_var(idx: StyleVar, val: f32) {
	unsafe{sys::igPushStyleVar_Float(idx as i32, val)}
}

pub fn push_style_var2(idx: StyleVar, val: [f32; 2]) {
	unsafe{sys::igPushStyleVar_Vec2(idx as i32, val)}
}

pub fn pop_style_var(count: i32) {
	unsafe{sys::igPopStyleVar(count)}
}

pub fn push_allow_keyboard_focus(allow_keyboard_focus: bool) {
	unsafe{sys::igPushAllowKeyboardFocus(allow_keyboard_focus)}
}

pub fn pop_allow_keyboard_focus() {
	unsafe{sys::igPopAllowKeyboardFocus()}
}

pub fn push_button_repeat(repeat: bool) {
	unsafe{sys::igPushButtonRepeat(repeat)}
}

pub fn pop_button_repeat() {
	unsafe{sys::igPopButtonRepeat()}
}

pub fn push_item_width(item_width: f32) {
	unsafe{sys::igPushItemWidth(item_width)}
}

pub fn pop_item_width() {
	unsafe{sys::igPopItemWidth()}
}

pub fn set_next_item_width(item_width: f32) {
	unsafe{sys::igSetNextItemWidth(item_width)}
}

pub fn calc_item_width() -> f32 {
	unsafe{sys::igCalcItemWidth()}
}

pub fn push_text_wrap_pos(wrap_local_pos_x: f32) {
	unsafe{sys::igPushTextWrapPos(wrap_local_pos_x)}
}

pub fn pop_text_wrap_pos() {
	unsafe{sys::igPopTextWrapPos()}
}

pub fn get_font() -> *mut sys::ImFont {
	unsafe{sys::igGetFont()}
}

pub fn get_font_size() -> f32 {
	unsafe{sys::igGetFontSize()}
}

pub fn get_font_tex_uv_white_pixel(pOut: &mut [f32; 2]) {
	unsafe{sys::igGetFontTexUvWhitePixel(pOut)}
}

pub fn get_color(idx: Col) -> u32 {
	unsafe{sys::igGetColorU32_Col(idx as i32, 1.0)}
}

pub fn get_color_u32__col(idx: Col, alpha_mul: f32) -> u32 {
	unsafe{sys::igGetColorU32_Col(idx as i32, alpha_mul)}
}

pub fn get_color_u32__vec4(col: [f32; 4]) -> u32 {
	unsafe{sys::igGetColorU32_Vec4(col)}
}

pub fn get_color_u32__u32(col: u32) -> u32 {
	unsafe{sys::igGetColorU32_U32(col)}
}

pub fn get_style_color_vec4(idx: Col) -> *const [f32; 4] {
	unsafe{sys::igGetStyleColorVec4(idx as i32)}
}

pub fn separator() {
	unsafe{sys::igSeparator()}
}

pub fn same_line() {
	unsafe{sys::igSameLine(0.0, -1.0)}
}

pub fn same_line2(offset_from_start_x: f32, spacing: f32) {
	unsafe{sys::igSameLine(offset_from_start_x, spacing)}
}

pub fn new_line() {
	unsafe{sys::igNewLine()}
}

pub fn spacing() {
	unsafe{sys::igSpacing()}
}

pub fn dummy(size: [f32; 2]) {
	unsafe{sys::igDummy(size)}
}

pub fn indent(indent_w: f32) {
	unsafe{sys::igIndent(indent_w)}
}

pub fn unindent(indent_w: f32) {
	unsafe{sys::igUnindent(indent_w)}
}

pub fn begin_group() {
	unsafe{sys::igBeginGroup()}
}

pub fn end_group() {
	unsafe{sys::igEndGroup()}
}

pub fn get_cursor_pos() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetCursorPos(&mut r)}
	r
}

pub fn get_cursor_pos_x() -> f32 {
	unsafe{sys::igGetCursorPosX()}
}

pub fn get_cursor_pos_y() -> f32 {
	unsafe{sys::igGetCursorPosY()}
}

pub fn set_cursor_pos(local_pos: [f32; 2]) {
	unsafe{sys::igSetCursorPos(local_pos)}
}

pub fn set_cursor_pos_x(local_x: f32) {
	unsafe{sys::igSetCursorPosX(local_x)}
}

pub fn set_cursor_pos_y(local_y: f32) {
	unsafe{sys::igSetCursorPosY(local_y)}
}

pub fn get_cursor_start_pos() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetCursorStartPos(&mut r)}
	r
}

pub fn get_cursor_screen_pos() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetCursorScreenPos(&mut r)}
	r
}

pub fn set_cursor_screen_pos(pos: [f32; 2]) {
	unsafe{sys::igSetCursorScreenPos(pos)}
}

pub fn align_text_to_frame_padding() {
	unsafe{sys::igAlignTextToFramePadding()}
}

pub fn get_text_line_height() -> f32 {
	unsafe{sys::igGetTextLineHeight()}
}

pub fn get_text_line_height_with_spacing() -> f32 {
	unsafe{sys::igGetTextLineHeightWithSpacing()}
}

pub fn get_frame_height() -> f32 {
	unsafe{sys::igGetFrameHeight()}
}

pub fn get_frame_height_with_spacing() -> f32 {
	unsafe{sys::igGetFrameHeightWithSpacing()}
}

pub fn push_id(str_id: &str) {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igPushID_Str(str_id_.as_ptr())}
}

pub fn push_id__str_str(str_id_begin: &str, str_id_end: &str) {
	let str_id_begin_ = CString::new(str_id_begin).unwrap();
	let str_id_end_ = CString::new(str_id_end).unwrap();
	unsafe{sys::igPushID_StrStr(str_id_begin_.as_ptr(), str_id_end_.as_ptr())}
}

pub fn push_id__ptr(ptr_id: *const ::std::os::raw::c_void) {
	unsafe{sys::igPushID_Ptr(ptr_id)}
}

pub fn push_id_i32(int_id: i32) {
	unsafe{sys::igPushID_Int(int_id)}
}

pub fn pop_id() {
	unsafe{sys::igPopID()}
}

pub fn get_id(str_id: &str) -> sys::ImGuiID {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igGetID_Str(str_id_.as_ptr())}
}

pub fn get_id__str_str(str_id_begin: &str, str_id_end: &str) -> sys::ImGuiID {
	let str_id_begin_ = CString::new(str_id_begin).unwrap();
	let str_id_end_ = CString::new(str_id_end).unwrap();
	unsafe{sys::igGetID_StrStr(str_id_begin_.as_ptr(), str_id_end_.as_ptr())}
}

pub fn get_id__ptr(ptr_id: *const ::std::os::raw::c_void) -> sys::ImGuiID {
	unsafe{sys::igGetID_Ptr(ptr_id)}
}

pub fn text_unformatted(text: &str) {
	let text_ = CString::new(text).unwrap();
	unsafe{sys::igTextUnformatted(text_.as_ptr(), std::ptr::null::<*mut i8>() as *mut _)}
}

pub fn text(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igText(fmt_.as_ptr())}
}

pub fn text_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextV(fmt_.as_ptr(), args)}
}

pub fn text_colored(col: [f32; 4], fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextColored(col, fmt_.as_ptr())}
}

pub fn text_colored_v(col: [f32; 4], fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextColoredV(col, fmt_.as_ptr(), args)}
}

pub fn text_disabled(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextDisabled(fmt_.as_ptr())}
}

pub fn text_disabled_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextDisabledV(fmt_.as_ptr(), args)}
}

pub fn text_wrapped(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextWrapped(fmt_.as_ptr())}
}

pub fn text_wrapped_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTextWrappedV(fmt_.as_ptr(), args)}
}

pub fn label_text(label: &str, fmt: &str) {
	let label_ = CString::new(label).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igLabelText(label_.as_ptr(), fmt_.as_ptr())}
}

pub fn label_text_v(label: &str, fmt: &str, args: sys::va_list) {
	let label_ = CString::new(label).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igLabelTextV(label_.as_ptr(), fmt_.as_ptr(), args)}
}

pub fn bullet_text(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igBulletText(fmt_.as_ptr())}
}

pub fn bullet_text_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igBulletTextV(fmt_.as_ptr(), args)}
}

pub fn button(label: &str, size: [f32; 2]) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igButton(label_.as_ptr(), size)}
}

pub fn small_button(label: &str) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igSmallButton(label_.as_ptr())}
}

pub fn invisible_button(str_id: &str, size: [f32; 2], flags: ButtonFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igInvisibleButton(str_id_.as_ptr(), size, flags.bits)}
}

pub fn arrow_button(str_id: &str, dir: Dir) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igArrowButton(str_id_.as_ptr(), dir as i32)}
}

pub fn image(user_texture_id: usize, size: [f32; 2], uv0: [f32; 2], uv1: [f32; 2], tint_col: [f32; 4], border_col: [f32; 4]) {
	unsafe{sys::igImage(user_texture_id as *mut _, size, uv0, uv1, tint_col, border_col)}
}

pub fn image_button(user_texture_id: usize, size: [f32; 2], uv0: [f32; 2], uv1: [f32; 2], frame_padding: i32, bg_col: [f32; 4], tint_col: [f32; 4]) -> bool {
	unsafe{sys::igImageButton(user_texture_id as *mut _, size, uv0, uv1, frame_padding, bg_col, tint_col)}
}

pub fn checkbox(label: &str, v: &mut bool) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCheckbox(label_.as_ptr(), v)}
}

pub fn checkbox_flags__int_ptr(label: &str, flags: &mut i32, flags_value: i32) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCheckboxFlags_IntPtr(label_.as_ptr(), flags, flags_value)}
}

pub fn checkbox_flags__uint_ptr(label: &str, flags: &mut ::std::os::raw::c_uint, flags_value: ::std::os::raw::c_uint) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCheckboxFlags_UintPtr(label_.as_ptr(), flags, flags_value)}
}

pub fn radio_button__bool(label: &str, active: bool) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igRadioButton_Bool(label_.as_ptr(), active)}
}

pub fn radio_button__int_ptr(label: &str, v: &mut i32, v_button: i32) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igRadioButton_IntPtr(label_.as_ptr(), v, v_button)}
}

pub fn progress_bar(fraction: f32, size_arg: [f32; 2], overlay: &str) {
	let overlay_ = CString::new(overlay).unwrap();
	unsafe{sys::igProgressBar(fraction, size_arg, overlay_.as_ptr())}
}

pub fn bullet() {
	unsafe{sys::igBullet()}
}

pub fn begin_combo(label: &str, preview_value: &str, flags: ComboFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let preview_value_ = CString::new(preview_value).unwrap();
	unsafe{sys::igBeginCombo(label_.as_ptr(), preview_value_.as_ptr(), flags.bits)}
}

pub fn end_combo() {
	unsafe{sys::igEndCombo()}
}

pub fn combo__str_arr(label: &str, current_item: &mut i32, items: *const *const ::std::os::raw::c_char, items_count: i32, popup_max_height_in_items: i32) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCombo_Str_arr(label_.as_ptr(), current_item, items, items_count, popup_max_height_in_items)}
}

pub fn combo__str(label: &str, current_item: &mut i32, items_separated_by_zeros: &str, popup_max_height_in_items: i32) -> bool {
	let label_ = CString::new(label).unwrap();
	let items_separated_by_zeros_ = CString::new(items_separated_by_zeros).unwrap();
	unsafe{sys::igCombo_Str(label_.as_ptr(), current_item, items_separated_by_zeros_.as_ptr(), popup_max_height_in_items)}
}

pub fn drag_float(label: &str, v: &mut f32, v_speed: f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragFloat(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_float2(label: &str, v: &mut f32, v_speed: f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragFloat2(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_float3(label: &str, v: &mut f32, v_speed: f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragFloat3(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_float4(label: &str, v: &mut f32, v_speed: f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragFloat4(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_float_range2(label: &str, v_current_min: &mut f32, v_current_max: &mut f32, v_speed: f32, v_min: f32, v_max: f32, format: &str, format_max: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	let format_max_ = CString::new(format_max).unwrap();
	unsafe{sys::igDragFloatRange2(label_.as_ptr(), v_current_min, v_current_max, v_speed, v_min, v_max, format_.as_ptr(), format_max_.as_ptr(), flags.bits)}
}

pub fn drag_int(label: &str, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragInt(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_int2(label: &str, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragInt2(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_int3(label: &str, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragInt3(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_int4(label: &str, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragInt4(label_.as_ptr(), v, v_speed, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_int_range2(label: &str, v_current_min: &mut i32, v_current_max: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &str, format_max: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	let format_max_ = CString::new(format_max).unwrap();
	unsafe{sys::igDragIntRange2(label_.as_ptr(), v_current_min, v_current_max, v_speed, v_min, v_max, format_.as_ptr(), format_max_.as_ptr(), flags.bits)}
}

pub fn drag_scalar(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, v_speed: f32, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragScalar(label_.as_ptr(), data_type as i32, p_data, v_speed, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn drag_scalar_n(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, components: i32, v_speed: f32, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragScalarN(label_.as_ptr(), data_type as i32, p_data, components, v_speed, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_float(label: &str, v: &mut f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderFloat(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_float2(label: &str, v: &mut f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderFloat2(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_float3(label: &str, v: &mut f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderFloat3(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_float4(label: &str, v: &mut f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderFloat4(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_angle(label: &str, v_rad: &mut f32, v_degrees_min: f32, v_degrees_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderAngle(label_.as_ptr(), v_rad, v_degrees_min, v_degrees_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_int(label: &str, v: &mut i32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderInt(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_int2(label: &str, v: &mut i32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderInt2(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_int3(label: &str, v: &mut i32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderInt3(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_int4(label: &str, v: &mut i32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderInt4(label_.as_ptr(), v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_scalar(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderScalar(label_.as_ptr(), data_type as i32, p_data, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_scalar_n(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, components: i32, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderScalarN(label_.as_ptr(), data_type as i32, p_data, components, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn v_slider_float(label: &str, size: [f32; 2], v: &mut f32, v_min: f32, v_max: f32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igVSliderFloat(label_.as_ptr(), size, v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn v_slider_int(label: &str, size: [f32; 2], v: &mut i32, v_min: i32, v_max: i32, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igVSliderInt(label_.as_ptr(), size, v, v_min, v_max, format_.as_ptr(), flags.bits)}
}

pub fn v_slider_scalar(label: &str, size: [f32; 2], data_type: DataType, p_data: &mut ::std::os::raw::c_void, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igVSliderScalar(label_.as_ptr(), size, data_type as i32, p_data, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn input_text(label: &str, entry: &mut String, flags: InputTextFlags) -> bool {
	entry.push('\0');
	let label_ = CString::new(label).unwrap();
	let r = unsafe{sys::igInputText(label_.as_ptr(), entry.as_mut_ptr() as *mut _, entry.capacity() as u64, flags.bits, None, std::ptr::null::<*mut i8>() as *mut _)};
	if let Some(p) = entry.find('\0') {entry.truncate(p);}
	r
}

pub fn input_text_multiline(label: &str, entry: &mut String, size: [f32; 2], flags: InputTextFlags) -> bool {
	entry.push('\0');
	let label_ = CString::new(label).unwrap();
	let r = unsafe{sys::igInputTextMultiline(label_.as_ptr(), entry.as_mut_ptr() as *mut _, entry.capacity() as u64, size, flags.bits, None, std::ptr::null::<*mut i8>() as *mut _)};
	if let Some(p) = entry.find('\0') {entry.truncate(p);}
	r
}

pub fn input_text_with_hint(label: &str, hint: &str, entry: &mut String, flags: InputTextFlags) -> bool {
	entry.push('\0');
	let label_ = CString::new(label).unwrap();
	let hint_ = CString::new(hint).unwrap();
	let r = unsafe{sys::igInputTextWithHint(label_.as_ptr(), hint_.as_ptr(), entry.as_mut_ptr() as *mut _, entry.capacity() as u64, flags.bits, None, std::ptr::null::<*mut i8>() as *mut _)};
	if let Some(p) = entry.find('\0') {entry.truncate(p);}
	r
}

pub fn input_float(label: &str, v: &mut f32, step: f32, step_fast: f32, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputFloat(label_.as_ptr(), v, step, step_fast, format_.as_ptr(), flags.bits)}
}

pub fn input_float2(label: &str, v: &mut f32, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputFloat2(label_.as_ptr(), v, format_.as_ptr(), flags.bits)}
}

pub fn input_float3(label: &str, v: &mut f32, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputFloat3(label_.as_ptr(), v, format_.as_ptr(), flags.bits)}
}

pub fn input_float4(label: &str, v: &mut f32, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputFloat4(label_.as_ptr(), v, format_.as_ptr(), flags.bits)}
}

pub fn input_int(label: &str, v: &mut i32, step: i32, step_fast: i32, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igInputInt(label_.as_ptr(), v, step, step_fast, flags.bits)}
}

pub fn input_int2(label: &str, v: &mut i32, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igInputInt2(label_.as_ptr(), v, flags.bits)}
}

pub fn input_int3(label: &str, v: &mut i32, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igInputInt3(label_.as_ptr(), v, flags.bits)}
}

pub fn input_int4(label: &str, v: &mut i32, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igInputInt4(label_.as_ptr(), v, flags.bits)}
}

pub fn input_double(label: &str, v: &mut f64, step: f64, step_fast: f64, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputDouble(label_.as_ptr(), v, step, step_fast, format_.as_ptr(), flags.bits)}
}

pub fn input_scalar(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, p_step: *const ::std::os::raw::c_void, p_step_fast: *const ::std::os::raw::c_void, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputScalar(label_.as_ptr(), data_type as i32, p_data, p_step, p_step_fast, format_.as_ptr(), flags.bits)}
}

pub fn input_scalar_n(label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, components: i32, p_step: *const ::std::os::raw::c_void, p_step_fast: *const ::std::os::raw::c_void, format: &str, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igInputScalarN(label_.as_ptr(), data_type as i32, p_data, components, p_step, p_step_fast, format_.as_ptr(), flags.bits)}
}

pub fn color_edit3(label: &str, col: &mut [f32; 3], flags: ColorEditFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igColorEdit3(label_.as_ptr(), col.as_ptr() as *mut _, flags.bits)}
}

pub fn color_edit4(label: &str, col: &mut [f32; 4], flags: ColorEditFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igColorEdit4(label_.as_ptr(), col.as_ptr() as *mut _, flags.bits)}
}

pub fn color_picker3(label: &str, col: &mut [f32; 3], flags: ColorEditFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igColorPicker3(label_.as_ptr(), col.as_ptr() as *mut _, flags.bits)}
}

pub fn color_picker4(label: &str, col: &mut [f32; 4], flags: ColorEditFlags, ref_col: *const f32) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igColorPicker4(label_.as_ptr(), col.as_ptr() as *mut _, flags.bits, ref_col)}
}

pub fn color_button(desc_id: &str, col: [f32; 4], flags: ColorEditFlags, size: [f32; 2]) -> bool {
	let desc_id_ = CString::new(desc_id).unwrap();
	unsafe{sys::igColorButton(desc_id_.as_ptr(), col, flags.bits, size)}
}

pub fn set_color_edit_options(flags: ColorEditFlags) {
	unsafe{sys::igSetColorEditOptions(flags.bits)}
}

pub fn tree_node(label: &str) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTreeNode_Str(label_.as_ptr())}
}

pub fn tree_node__str_str(str_id: &str, fmt: &str) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNode_StrStr(str_id_.as_ptr(), fmt_.as_ptr())}
}

pub fn tree_node__ptr(ptr_id: *const ::std::os::raw::c_void, fmt: &str) -> bool {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNode_Ptr(ptr_id, fmt_.as_ptr())}
}

pub fn tree_node_v__str(str_id: &str, fmt: &str, args: sys::va_list) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeV_Str(str_id_.as_ptr(), fmt_.as_ptr(), args)}
}

pub fn tree_node_v__ptr(ptr_id: *const ::std::os::raw::c_void, fmt: &str, args: sys::va_list) -> bool {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeV_Ptr(ptr_id, fmt_.as_ptr(), args)}
}

pub fn tree_node_ex__str(label: &str, flags: TreeNodeFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTreeNodeEx_Str(label_.as_ptr(), flags.bits)}
}

pub fn tree_node_ex__str_str(str_id: &str, flags: TreeNodeFlags, fmt: &str) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeEx_StrStr(str_id_.as_ptr(), flags.bits, fmt_.as_ptr())}
}

pub fn tree_node_ex__ptr(ptr_id: *const ::std::os::raw::c_void, flags: TreeNodeFlags, fmt: &str) -> bool {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeEx_Ptr(ptr_id, flags.bits, fmt_.as_ptr())}
}

pub fn tree_node_ex_v__str(str_id: &str, flags: TreeNodeFlags, fmt: &str, args: sys::va_list) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeExV_Str(str_id_.as_ptr(), flags.bits, fmt_.as_ptr(), args)}
}

pub fn tree_node_ex_v__ptr(ptr_id: *const ::std::os::raw::c_void, flags: TreeNodeFlags, fmt: &str, args: sys::va_list) -> bool {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igTreeNodeExV_Ptr(ptr_id, flags.bits, fmt_.as_ptr(), args)}
}

pub fn tree_push__str(str_id: &str) {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igTreePush_Str(str_id_.as_ptr())}
}

pub fn tree_push__ptr(ptr_id: *const ::std::os::raw::c_void) {
	unsafe{sys::igTreePush_Ptr(ptr_id)}
}

pub fn tree_pop() {
	unsafe{sys::igTreePop()}
}

pub fn get_tree_node_to_label_spacing() -> f32 {
	unsafe{sys::igGetTreeNodeToLabelSpacing()}
}

pub fn collapsing_header(label: &str, flags: TreeNodeFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCollapsingHeader_TreeNodeFlags(label_.as_ptr(), flags.bits)}
}

pub fn collapsing_header_bool(label: &str, p_visible: &mut bool, flags: TreeNodeFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCollapsingHeader_BoolPtr(label_.as_ptr(), p_visible, flags.bits)}
}

pub fn set_next_item_open(is_open: bool, cond: Cond) {
	unsafe{sys::igSetNextItemOpen(is_open, cond as i32)}
}

pub fn selectable(label: &str, selected: bool, flags: SelectableFlags, size: [f32; 2]) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igSelectable_Bool(label_.as_ptr(), selected, flags.bits, size)}
}

pub fn selectable__bool_ptr(label: &str, p_selected: &mut bool, flags: SelectableFlags, size: [f32; 2]) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igSelectable_BoolPtr(label_.as_ptr(), p_selected, flags.bits, size)}
}

pub fn begin_list_box(label: &str, size: [f32; 2]) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igBeginListBox(label_.as_ptr(), size)}
}

pub fn end_list_box() {
	unsafe{sys::igEndListBox()}
}

pub fn list_box__str_arr(label: &str, current_item: &mut i32, items: *const *const ::std::os::raw::c_char, items_count: i32, height_in_items: i32) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igListBox_Str_arr(label_.as_ptr(), current_item, items, items_count, height_in_items)}
}

pub fn plot_lines__float_ptr(label: &str, values: *const f32, values_count: i32, values_offset: i32, overlay_text: &str, scale_min: f32, scale_max: f32, graph_size: [f32; 2], stride: i32) {
	let label_ = CString::new(label).unwrap();
	let overlay_text_ = CString::new(overlay_text).unwrap();
	unsafe{sys::igPlotLines_FloatPtr(label_.as_ptr(), values, values_count, values_offset, overlay_text_.as_ptr(), scale_min, scale_max, graph_size, stride)}
}

pub fn plot_histogram__float_ptr(label: &str, values: *const f32, values_count: i32, values_offset: i32, overlay_text: &str, scale_min: f32, scale_max: f32, graph_size: [f32; 2], stride: i32) {
	let label_ = CString::new(label).unwrap();
	let overlay_text_ = CString::new(overlay_text).unwrap();
	unsafe{sys::igPlotHistogram_FloatPtr(label_.as_ptr(), values, values_count, values_offset, overlay_text_.as_ptr(), scale_min, scale_max, graph_size, stride)}
}

pub fn value__bool(prefix: &str, b: bool) {
	let prefix_ = CString::new(prefix).unwrap();
	unsafe{sys::igValue_Bool(prefix_.as_ptr(), b)}
}

pub fn value__int(prefix: &str, v: i32) {
	let prefix_ = CString::new(prefix).unwrap();
	unsafe{sys::igValue_Int(prefix_.as_ptr(), v)}
}

pub fn value__uint(prefix: &str, v: ::std::os::raw::c_uint) {
	let prefix_ = CString::new(prefix).unwrap();
	unsafe{sys::igValue_Uint(prefix_.as_ptr(), v)}
}

pub fn value__float(prefix: &str, v: f32, float_format: &str) {
	let prefix_ = CString::new(prefix).unwrap();
	let float_format_ = CString::new(float_format).unwrap();
	unsafe{sys::igValue_Float(prefix_.as_ptr(), v, float_format_.as_ptr())}
}

pub fn begin_menu_bar() -> bool {
	unsafe{sys::igBeginMenuBar()}
}

pub fn end_menu_bar() {
	unsafe{sys::igEndMenuBar()}
}

pub fn begin_main_menu_bar() -> bool {
	unsafe{sys::igBeginMainMenuBar()}
}

pub fn end_main_menu_bar() {
	unsafe{sys::igEndMainMenuBar()}
}

pub fn begin_menu(label: &str, enabled: bool) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igBeginMenu(label_.as_ptr(), enabled)}
}

pub fn end_menu() {
	unsafe{sys::igEndMenu()}
}

pub fn menu_item__bool(label: &str, shortcut: &str, selected: bool, enabled: bool) -> bool {
	let label_ = CString::new(label).unwrap();
	let shortcut_ = CString::new(shortcut).unwrap();
	unsafe{sys::igMenuItem_Bool(label_.as_ptr(), shortcut_.as_ptr(), selected, enabled)}
}

pub fn menu_item__bool_ptr(label: &str, shortcut: &str, p_selected: &mut bool, enabled: bool) -> bool {
	let label_ = CString::new(label).unwrap();
	let shortcut_ = CString::new(shortcut).unwrap();
	unsafe{sys::igMenuItem_BoolPtr(label_.as_ptr(), shortcut_.as_ptr(), p_selected, enabled)}
}

pub fn begin_tooltip() {
	unsafe{sys::igBeginTooltip()}
}

pub fn end_tooltip() {
	unsafe{sys::igEndTooltip()}
}

pub fn set_tooltip(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igSetTooltip(fmt_.as_ptr())}
}

pub fn set_tooltip_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igSetTooltipV(fmt_.as_ptr(), args)}
}

pub fn begin_popup(str_id: &str, flags: WindowFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginPopup(str_id_.as_ptr(), flags.bits)}
}

pub fn begin_popup_modal(name: &str, p_open: &mut bool, flags: WindowFlags) -> bool {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igBeginPopupModal(name_.as_ptr(), p_open, flags.bits)}
}

pub fn end_popup() {
	unsafe{sys::igEndPopup()}
}

pub fn open_popup(str_id: &str, popup_flags: PopupFlags) {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igOpenPopup_Str(str_id_.as_ptr(), popup_flags.bits)}
}

pub fn open_popup_u32(id: sys::ImGuiID, popup_flags: PopupFlags) {
	unsafe{sys::igOpenPopup_ID(id, popup_flags.bits)}
}

pub fn open_popup_on_item_click(str_id: &str, popup_flags: PopupFlags) {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igOpenPopupOnItemClick(str_id_.as_ptr(), popup_flags.bits)}
}

pub fn close_current_popup() {
	unsafe{sys::igCloseCurrentPopup()}
}

pub fn begin_popup_context_item(str_id: &str, popup_flags: PopupFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginPopupContextItem(str_id_.as_ptr(), popup_flags.bits)}
}

pub fn begin_popup_context_window(str_id: &str, popup_flags: PopupFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginPopupContextWindow(str_id_.as_ptr(), popup_flags.bits)}
}

pub fn begin_popup_context_void(str_id: &str, popup_flags: PopupFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginPopupContextVoid(str_id_.as_ptr(), popup_flags.bits)}
}

pub fn is_popup_open__str(str_id: &str, flags: PopupFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igIsPopupOpen_Str(str_id_.as_ptr(), flags.bits)}
}

pub fn begin_table(str_id: &str, column: i32, flags: TableFlags, outer_size: [f32; 2], inner_width: f32) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginTable(str_id_.as_ptr(), column, flags.bits, outer_size, inner_width)}
}

pub fn end_table() {
	unsafe{sys::igEndTable()}
}

pub fn table_next_row(row_flags: TableRowFlags, min_row_height: f32) {
	unsafe{sys::igTableNextRow(row_flags.bits, min_row_height)}
}

pub fn table_next_column() -> bool {
	unsafe{sys::igTableNextColumn()}
}

pub fn table_set_column_index(column_n: i32) -> bool {
	unsafe{sys::igTableSetColumnIndex(column_n)}
}

pub fn table_setup_column(label: &str, flags: TableColumnFlags, init_width_or_weight: f32, user_id: sys::ImGuiID) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTableSetupColumn(label_.as_ptr(), flags.bits, init_width_or_weight, user_id)}
}

pub fn table_setup_scroll_freeze(cols: i32, rows: i32) {
	unsafe{sys::igTableSetupScrollFreeze(cols, rows)}
}

pub fn table_headers_row() {
	unsafe{sys::igTableHeadersRow()}
}

pub fn table_header(label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTableHeader(label_.as_ptr())}
}

pub fn table_get_sort_specs() -> *mut sys::ImGuiTableSortSpecs {
	unsafe{sys::igTableGetSortSpecs()}
}

pub fn table_get_column_count() -> i32 {
	unsafe{sys::igTableGetColumnCount()}
}

pub fn table_get_column_index() -> i32 {
	unsafe{sys::igTableGetColumnIndex()}
}

pub fn table_get_row_index() -> i32 {
	unsafe{sys::igTableGetRowIndex()}
}

pub fn table_get_column_name__int(column_n: i32) -> *const ::std::os::raw::c_char {
	unsafe{sys::igTableGetColumnName_Int(column_n)}
}

pub fn table_get_column_flags(column_n: i32) -> i32 {
	unsafe{sys::igTableGetColumnFlags(column_n)}
}

pub fn table_set_column_enabled(column_n: i32, v: bool) {
	unsafe{sys::igTableSetColumnEnabled(column_n, v)}
}

pub fn table_set_bg_color(target: TableBgTarget, color: u32, column_n: i32) {
	unsafe{sys::igTableSetBgColor(target as i32, color, column_n)}
}

pub fn columns(count: i32, id: &str, border: bool) {
	let id_ = CString::new(id).unwrap();
	unsafe{sys::igColumns(count, id_.as_ptr(), border)}
}

pub fn next_column() {
	unsafe{sys::igNextColumn()}
}

pub fn get_column_index() -> i32 {
	unsafe{sys::igGetColumnIndex()}
}

pub fn get_column_width(column_index: i32) -> f32 {
	unsafe{sys::igGetColumnWidth(column_index)}
}

pub fn set_column_width(column_index: i32, width: f32) {
	unsafe{sys::igSetColumnWidth(column_index, width)}
}

pub fn get_column_offset(column_index: i32) -> f32 {
	unsafe{sys::igGetColumnOffset(column_index)}
}

pub fn set_column_offset(column_index: i32, offset_x: f32) {
	unsafe{sys::igSetColumnOffset(column_index, offset_x)}
}

pub fn get_columns_count() -> i32 {
	unsafe{sys::igGetColumnsCount()}
}

pub fn begin_tab_bar(str_id: &str, flags: TabBarFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginTabBar(str_id_.as_ptr(), flags.bits)}
}

pub fn end_tab_bar() {
	unsafe{sys::igEndTabBar()}
}

pub fn begin_tab_item(label: &str, p_open: &mut bool, flags: TabItemFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igBeginTabItem(label_.as_ptr(), p_open, flags.bits)}
}

pub fn end_tab_item() {
	unsafe{sys::igEndTabItem()}
}

pub fn tab_item_button(label: &str, flags: TabItemFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTabItemButton(label_.as_ptr(), flags.bits)}
}

pub fn set_tab_item_closed(tab_or_docked_window_label: &str) {
	let tab_or_docked_window_label_ = CString::new(tab_or_docked_window_label).unwrap();
	unsafe{sys::igSetTabItemClosed(tab_or_docked_window_label_.as_ptr())}
}

pub fn log_to_tty(auto_open_depth: i32) {
	unsafe{sys::igLogToTTY(auto_open_depth)}
}

pub fn log_to_file(auto_open_depth: i32, filename: &str) {
	let filename_ = CString::new(filename).unwrap();
	unsafe{sys::igLogToFile(auto_open_depth, filename_.as_ptr())}
}

pub fn log_to_clipboard(auto_open_depth: i32) {
	unsafe{sys::igLogToClipboard(auto_open_depth)}
}

pub fn log_finish() {
	unsafe{sys::igLogFinish()}
}

pub fn log_buttons() {
	unsafe{sys::igLogButtons()}
}

pub fn log_text_v(fmt: &str, args: sys::va_list) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igLogTextV(fmt_.as_ptr(), args)}
}

pub fn begin_drag_drop_source(flags: DragDropFlags) -> bool {
	unsafe{sys::igBeginDragDropSource(flags.bits)}
}

pub fn set_drag_drop_payload(type_: &str, data: *const ::std::os::raw::c_void, sz: sys::size_t, cond: Cond) -> bool {
	let type__ = CString::new(type_).unwrap();
	unsafe{sys::igSetDragDropPayload(type__.as_ptr(), data, sz, cond as i32)}
}

pub fn end_drag_drop_source() {
	unsafe{sys::igEndDragDropSource()}
}

pub fn begin_drag_drop_target() -> bool {
	unsafe{sys::igBeginDragDropTarget()}
}

pub fn accept_drag_drop_payload(type_: &str, flags: DragDropFlags) -> *const sys::ImGuiPayload {
	let type__ = CString::new(type_).unwrap();
	unsafe{sys::igAcceptDragDropPayload(type__.as_ptr(), flags.bits)}
}

pub fn end_drag_drop_target() {
	unsafe{sys::igEndDragDropTarget()}
}

pub fn get_drag_drop_payload() -> *const sys::ImGuiPayload {
	unsafe{sys::igGetDragDropPayload()}
}

pub fn begin_disabled(disabled: bool) {
	unsafe{sys::igBeginDisabled(disabled)}
}

pub fn end_disabled() {
	unsafe{sys::igEndDisabled()}
}

pub fn push_clip_rect(clip_rect_min: [f32; 2], clip_rect_max: [f32; 2], intersect_with_current_clip_rect: bool) {
	unsafe{sys::igPushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect)}
}

pub fn pop_clip_rect() {
	unsafe{sys::igPopClipRect()}
}

pub fn set_item_default_focus() {
	unsafe{sys::igSetItemDefaultFocus()}
}

pub fn set_keyboard_focus_here(offset: i32) {
	unsafe{sys::igSetKeyboardFocusHere(offset)}
}

pub fn is_item_hovered() -> bool {
	unsafe{sys::igIsItemHovered(0)}
}

pub fn is_item_hovered2(flags: HoveredFlags) -> bool {
	unsafe{sys::igIsItemHovered(flags.bits)}
}

pub fn is_item_active() -> bool {
	unsafe{sys::igIsItemActive()}
}

pub fn is_item_focused() -> bool {
	unsafe{sys::igIsItemFocused()}
}

pub fn is_item_clicked(mouse_button: MouseButton) -> bool {
	unsafe{sys::igIsItemClicked(mouse_button as i32)}
}

pub fn is_item_visible() -> bool {
	unsafe{sys::igIsItemVisible()}
}

pub fn is_item_edited() -> bool {
	unsafe{sys::igIsItemEdited()}
}

pub fn is_item_activated() -> bool {
	unsafe{sys::igIsItemActivated()}
}

pub fn is_item_deactivated() -> bool {
	unsafe{sys::igIsItemDeactivated()}
}

pub fn is_item_deactivated_after_edit() -> bool {
	unsafe{sys::igIsItemDeactivatedAfterEdit()}
}

pub fn is_item_toggled_open() -> bool {
	unsafe{sys::igIsItemToggledOpen()}
}

pub fn is_any_item_hovered() -> bool {
	unsafe{sys::igIsAnyItemHovered()}
}

pub fn is_any_item_active() -> bool {
	unsafe{sys::igIsAnyItemActive()}
}

pub fn is_any_item_focused() -> bool {
	unsafe{sys::igIsAnyItemFocused()}
}

pub fn get_item_rect_min() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetItemRectMin(&mut r)}
	r
}

pub fn get_item_rect_max() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetItemRectMax(&mut r)}
	r
}

pub fn get_item_rect_size() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetItemRectSize(&mut r)}
	r
}

pub fn set_item_allow_overlap() {
	unsafe{sys::igSetItemAllowOverlap()}
}

pub fn get_main_viewport() -> *mut sys::ImGuiViewport {
	unsafe{sys::igGetMainViewport()}
}

pub fn is_rect_visible__nil(size: [f32; 2]) -> bool {
	unsafe{sys::igIsRectVisible_Nil(size)}
}

pub fn is_rect_visible__vec2(rect_min: [f32; 2], rect_max: [f32; 2]) -> bool {
	unsafe{sys::igIsRectVisible_Vec2(rect_min, rect_max)}
}

pub fn get_time() -> f64 {
	unsafe{sys::igGetTime()}
}

pub fn get_frame_count() -> i32 {
	unsafe{sys::igGetFrameCount()}
}

pub fn get_background_draw_list__nil() -> *mut sys::ImDrawList {
	unsafe{sys::igGetBackgroundDrawList_Nil()}
}

pub fn get_foreground_draw_list__nil() -> *mut sys::ImDrawList {
	unsafe{sys::igGetForegroundDrawList_Nil()}
}

pub fn get_draw_list_shared_data() -> *mut sys::ImDrawListSharedData {
	unsafe{sys::igGetDrawListSharedData()}
}

pub fn get_style_color_name(idx: Col) -> *const ::std::os::raw::c_char {
	unsafe{sys::igGetStyleColorName(idx as i32)}
}

pub fn set_state_storage(storage: &mut sys::ImGuiStorage) {
	unsafe{sys::igSetStateStorage(storage)}
}

// pub fn get_state_storage() -> *mut sys::ImGuiStorage {
// 	unsafe{sys::igGetStateStorage()}
// }

pub fn get_state_storage() -> Storage {
	Storage(unsafe{sys::igGetStateStorage()})
}

pub struct Storage(*mut sys::ImGuiStorage);
impl Storage {
	pub fn bool(&self, key: u32, default: bool) -> &'static mut bool {
		unsafe{&mut *sys::ImGuiStorage_GetBoolRef(self.0, key, default)}
	}
	
	pub fn get_bool(&self, key: u32, default: bool) -> bool {
		unsafe{sys::ImGuiStorage_GetBool(self.0, key, default)}
	}
	
	pub fn set_bool(&self, key: u32, value: bool) {
		unsafe{sys::ImGuiStorage_SetBool(self.0, key, value)}
	}
	
	pub fn f32(&self, key: u32, default: f32) -> &'static mut f32 {
		unsafe{&mut *sys::ImGuiStorage_GetFloatRef(self.0, key, default)}
	}
	
	pub fn get_f32(&self, key: u32, default: f32) -> f32 {
		unsafe{sys::ImGuiStorage_GetFloat(self.0, key, default)}
	}
	
	pub fn set_f32(&self, key: u32, value: f32) {
		unsafe{sys::ImGuiStorage_SetFloat(self.0, key, value)}
	}
	
	pub fn i32(&self, key: u32, default: i32) -> &'static mut i32 {
		unsafe{&mut *sys::ImGuiStorage_GetIntRef(self.0, key, default)}
	}
	
	pub fn get_i32(&self, key: u32, default: i32) -> i32 {
		unsafe{sys::ImGuiStorage_GetInt(self.0, key, default)}
	}
	
	pub fn set_i32(&self, key: u32, value: i32) {
		unsafe{sys::ImGuiStorage_SetInt(self.0, key, value)}
	}
}

// pub fn calc_list_clipping(items_count: i32, items_height: f32, out_items_display_start: &mut i32, out_items_display_end: &mut i32) {
// 	unsafe{sys::igCalcListClipping(items_count, items_height, out_items_display_start, out_items_display_end)}
// }

pub fn begin_child_frame(id: sys::ImGuiID, size: [f32; 2], flags: WindowFlags) -> bool {
	unsafe{sys::igBeginChildFrame(id, size, flags.bits)}
}

pub fn end_child_frame() {
	unsafe{sys::igEndChildFrame()}
}

pub fn calc_text_size(text: &str, hide_text_after_double_hash: bool, wrap_width: f32) -> [f32; 2] {
	let mut r = [0f32; 2];
	let text_ = CString::new(text).unwrap();
	unsafe{sys::igCalcTextSize(&mut r, text_.as_ptr(), (text_.as_ptr() as usize + text.len()) as *const _, hide_text_after_double_hash, wrap_width)}
	r
}

pub fn color_convert_u32_to_float4(in_: u32) -> [f32; 4] {
	let mut r = [0f32; 4];
	unsafe{sys::igColorConvertU32ToFloat4(&mut r, in_)}
	r
}

pub fn color_convert_float4_to_u32(in_: [f32; 4]) -> u32 {
	unsafe{sys::igColorConvertFloat4ToU32(in_)}
}

pub fn color_convert_rgbto_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
	unsafe{sys::igColorConvertRGBtoHSV(r, g, b, &mut x, &mut y, &mut z)}
	(x, y, z)
}

pub fn color_convert_hsvto_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
	let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
	unsafe{sys::igColorConvertHSVtoRGB(h, s, v, &mut x, &mut y, &mut z)}
	(x, y, z)
}

pub fn get_key_index(imgui_key: Key) -> i32 {
	unsafe{sys::igGetKeyIndex(imgui_key as i32)}
}

pub fn is_key_down(user_key_index: i32) -> bool {
	unsafe{sys::igIsKeyDown(user_key_index)}
}

pub fn is_key_pressed(user_key_index: i32, repeat: bool) -> bool {
	unsafe{sys::igIsKeyPressed(user_key_index, repeat)}
}

pub fn is_key_released(user_key_index: i32) -> bool {
	unsafe{sys::igIsKeyReleased(user_key_index)}
}

pub fn get_key_pressed_amount(key_index: i32, repeat_delay: f32, rate: f32) -> i32 {
	unsafe{sys::igGetKeyPressedAmount(key_index, repeat_delay, rate)}
}

// pub fn capture_keyboard_from_app(want_capture_keyboard_value: bool) {
// 	unsafe{sys::igCaptureKeyboardFromApp(want_capture_keyboard_value)}
// }

pub fn is_mouse_down(button: MouseButton) -> bool {
	unsafe{sys::igIsMouseDown(button as i32)}
}

pub fn is_mouse_clicked(button: MouseButton, repeat: bool) -> bool {
	unsafe{sys::igIsMouseClicked(button as i32, repeat)}
}

pub fn is_mouse_released(button: MouseButton) -> bool {
	unsafe{sys::igIsMouseReleased(button as i32)}
}

pub fn is_mouse_double_clicked(button: MouseButton) -> bool {
	unsafe{sys::igIsMouseDoubleClicked(button as i32)}
}

pub fn is_mouse_hovering_rect(r_min: [f32; 2], r_max: [f32; 2], clip: bool) -> bool {
	unsafe{sys::igIsMouseHoveringRect(r_min, r_max, clip)}
}

pub fn is_mouse_pos_valid(mouse_pos: *const [f32; 2]) -> bool {
	unsafe{sys::igIsMousePosValid(mouse_pos)}
}

pub fn is_any_mouse_down() -> bool {
	unsafe{sys::igIsAnyMouseDown()}
}

pub fn get_mouse_pos() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetMousePos(&mut r)}
	r
}

pub fn get_mouse_pos_on_opening_current_popup() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetMousePosOnOpeningCurrentPopup(&mut r)}
	r
}

pub fn is_mouse_dragging(button: MouseButton, lock_threshold: f32) -> bool {
	unsafe{sys::igIsMouseDragging(button as i32, lock_threshold)}
}

pub fn get_mouse_drag_delta(button: MouseButton, lock_threshold: f32) -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetMouseDragDelta(&mut r, button as i32, lock_threshold)}
	r
}

pub fn reset_mouse_drag_delta(button: MouseButton) {
	unsafe{sys::igResetMouseDragDelta(button as i32)}
}

pub fn get_mouse_cursor() -> i32 {
	unsafe{sys::igGetMouseCursor()}
}

pub fn set_mouse_cursor(cursor_type: MouseCursor) {
	unsafe{sys::igSetMouseCursor(cursor_type as i32)}
}

// pub fn capture_mouse_from_app(want_capture_mouse_value: bool) {
// 	unsafe{sys::igCaptureMouseFromApp(want_capture_mouse_value)}
// }

pub fn get_clipboard_text() -> *const ::std::os::raw::c_char {
	unsafe{sys::igGetClipboardText()}
}

pub fn set_clipboard_text(text: &str) {
	let text_ = CString::new(text).unwrap();
	unsafe{sys::igSetClipboardText(text_.as_ptr())}
}

pub fn load_ini_settings_from_disk(ini_filename: &str) {
	let ini_filename_ = CString::new(ini_filename).unwrap();
	unsafe{sys::igLoadIniSettingsFromDisk(ini_filename_.as_ptr())}
}

pub fn load_ini_settings_from_memory(ini_data: &str, ini_size: sys::size_t) {
	let ini_data_ = CString::new(ini_data).unwrap();
	unsafe{sys::igLoadIniSettingsFromMemory(ini_data_.as_ptr(), ini_size)}
}

pub fn save_ini_settings_to_disk(ini_filename: &str) {
	let ini_filename_ = CString::new(ini_filename).unwrap();
	unsafe{sys::igSaveIniSettingsToDisk(ini_filename_.as_ptr())}
}

pub fn save_ini_settings_to_memory(out_ini_size: &mut sys::size_t) -> *const ::std::os::raw::c_char {
	unsafe{sys::igSaveIniSettingsToMemory(out_ini_size)}
}

pub fn debug_check_version_and_data_layout(version_str: &str, sz_io: sys::size_t, sz_style: sys::size_t, sz_vec2: sys::size_t, sz_vec4: sys::size_t, sz_drawvert: sys::size_t, sz_drawidx: sys::size_t) -> bool {
	let version_str_ = CString::new(version_str).unwrap();
	unsafe{sys::igDebugCheckVersionAndDataLayout(version_str_.as_ptr(), sz_io, sz_style, sz_vec2, sz_vec4, sz_drawvert, sz_drawidx)}
}

pub fn set_allocator_functions(alloc_func: sys::ImGuiMemAllocFunc, free_func: sys::ImGuiMemFreeFunc, user_data: &mut ::std::os::raw::c_void) {
	unsafe{sys::igSetAllocatorFunctions(alloc_func, free_func, user_data)}
}

pub fn get_allocator_functions(p_alloc_func: &mut sys::ImGuiMemAllocFunc, p_free_func: &mut sys::ImGuiMemFreeFunc, p_user_data: *mut *mut ::std::os::raw::c_void) {
	unsafe{sys::igGetAllocatorFunctions(p_alloc_func, p_free_func, p_user_data)}
}

pub fn mem_alloc(size: sys::size_t) -> *mut ::std::os::raw::c_void {
	unsafe{sys::igMemAlloc(size)}
}

pub fn mem_free(ptr: &mut ::std::os::raw::c_void) {
	unsafe{sys::igMemFree(ptr)}
}

pub fn im_draw_list_splitter__im_draw_list_splitter() -> *mut sys::ImDrawListSplitter {
	unsafe{sys::ImDrawListSplitter_ImDrawListSplitter()}
}

pub fn im_draw_list_splitter_destroy(self_: &mut sys::ImDrawListSplitter) {
	unsafe{sys::ImDrawListSplitter_destroy(self_)}
}

pub fn im_draw_list_splitter__clear(self_: &mut sys::ImDrawListSplitter) {
	unsafe{sys::ImDrawListSplitter_Clear(self_)}
}

pub fn im_draw_list_splitter__clear_free_memory(self_: &mut sys::ImDrawListSplitter) {
	unsafe{sys::ImDrawListSplitter_ClearFreeMemory(self_)}
}

pub fn im_draw_list_splitter__split(self_: &mut sys::ImDrawListSplitter, draw_list: &mut sys::ImDrawList, count: i32) {
	unsafe{sys::ImDrawListSplitter_Split(self_, draw_list, count)}
}

pub fn im_draw_list_splitter__merge(self_: &mut sys::ImDrawListSplitter, draw_list: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawListSplitter_Merge(self_, draw_list)}
}

pub fn im_draw_list_splitter__set_current_channel(self_: &mut sys::ImDrawListSplitter, draw_list: &mut sys::ImDrawList, channel_idx: i32) {
	unsafe{sys::ImDrawListSplitter_SetCurrentChannel(self_, draw_list, channel_idx)}
}

pub fn im_draw_list__im_draw_list(shared_data: *const sys::ImDrawListSharedData) -> *mut sys::ImDrawList {
	unsafe{sys::ImDrawList_ImDrawList(shared_data)}
}

pub fn im_draw_list_destroy(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList_destroy(self_)}
}

pub struct DrawList {
	pub drawlist: *mut sys::ImDrawList,
	vtx_offset: u16,
}

impl DrawList {
	pub fn push_clip_rect(&self, clip_rect_min: [f32; 2], clip_rect_max: [f32; 2], intersect_with_current_clip_rect: bool) {
		unsafe{sys::ImDrawList_PushClipRect(self.drawlist, clip_rect_min, clip_rect_max, intersect_with_current_clip_rect)}
	}
	
	pub fn push_clip_rect_full_screen(&self) {
		unsafe{sys::ImDrawList_PushClipRectFullScreen(self.drawlist)}
	}
	
	pub fn pop_clip_rect(&self) {
		unsafe{sys::ImDrawList_PopClipRect(self.drawlist)}
	}

	pub fn push_texture_id(&self, texture_id: usize) {
		unsafe{sys::ImDrawList_PushTextureID(self.drawlist, texture_id as *mut _)}
	}
	
	pub fn pop_texture_id(&self) {
		unsafe{sys::ImDrawList_PopTextureID(self.drawlist)}
	}
	
	pub fn get_clip_rect_min(&self) -> [f32; 2] {
		let mut r = [0f32; 2];
		unsafe{sys::ImDrawList_GetClipRectMin(&mut r, self.drawlist)}
		r
	}
	
	pub fn get_clip_rect_max(&self) -> [f32; 2] {
		let mut r = [0f32; 2];
		unsafe{sys::ImDrawList_GetClipRectMax(&mut r, self.drawlist)}
		r
	}
	
	pub fn add_line(&self, p1: [f32; 2], p2: [f32; 2], col: u32, thickness: f32) {
		unsafe{sys::ImDrawList_AddLine(self.drawlist, p1, p2, col, thickness)}
	}
	
	pub fn add_rect(&self, p_min: [f32; 2], p_max: [f32; 2], col: u32, rounding: f32, flags: DrawFlags, thickness: f32) {
		unsafe{sys::ImDrawList_AddRect(self.drawlist, p_min, p_max, col, rounding, flags.bits, thickness)}
	}
	
	pub fn add_rect_filled(&self, p_min: [f32; 2], p_max: [f32; 2], col: u32, rounding: f32, flags: DrawFlags) {
		unsafe{sys::ImDrawList_AddRectFilled(self.drawlist, p_min, p_max, col, rounding, flags.bits)}
	}
	
	pub fn add_rect_filled_multi_color(&self, p_min: [f32; 2], p_max: [f32; 2], col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {
		unsafe{sys::ImDrawList_AddRectFilledMultiColor(self.drawlist, p_min, p_max, col_upr_left, col_upr_right, col_bot_right, col_bot_left)}
	}
	
	pub fn add_quad(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], col: u32, thickness: f32) {
		unsafe{sys::ImDrawList_AddQuad(self.drawlist, p1, p2, p3, p4, col, thickness)}
	}
	
	pub fn add_quad_filled(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_AddQuadFilled(self.drawlist, p1, p2, p3, p4, col)}
	}
	
	pub fn add_triangle(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], col: u32, thickness: f32) {
		unsafe{sys::ImDrawList_AddTriangle(self.drawlist, p1, p2, p3, col, thickness)}
	}
	
	pub fn add_triangle_filled(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_AddTriangleFilled(self.drawlist, p1, p2, p3, col)}
	}
	
	pub fn add_circle(&self, center: [f32; 2], radius: f32, col: u32, num_segments: i32, thickness: f32) {
		unsafe{sys::ImDrawList_AddCircle(self.drawlist, center, radius, col, num_segments, thickness)}
	}
	
	pub fn add_circle_filled(&self, center: [f32; 2], radius: f32, col: u32, num_segments: i32) {
		unsafe{sys::ImDrawList_AddCircleFilled(self.drawlist, center, radius, col, num_segments)}
	}
	
	pub fn add_ngon(&self, center: [f32; 2], radius: f32, col: u32, num_segments: i32, thickness: f32) {
		unsafe{sys::ImDrawList_AddNgon(self.drawlist, center, radius, col, num_segments, thickness)}
	}
	
	pub fn add_ngon_filled(&self, center: [f32; 2], radius: f32, col: u32, num_segments: i32) {
		unsafe{sys::ImDrawList_AddNgonFilled(self.drawlist, center, radius, col, num_segments)}
	}
	
	pub fn add_text(&self, pos: [f32; 2], col: u32, text_begin: &str) {
		let text_begin_ = CString::new(text_begin).unwrap();
		unsafe{sys::ImDrawList_AddText_Vec2(self.drawlist, pos, col, text_begin_.as_ptr(), std::ptr::null())}
	}
	
	pub fn add_text__vec2(&self, pos: [f32; 2], col: u32, text_begin: &str, text_end: &str) {
		let text_begin_ = CString::new(text_begin).unwrap();
		let text_end_ = CString::new(text_end).unwrap();
		unsafe{sys::ImDrawList_AddText_Vec2(self.drawlist, pos, col, text_begin_.as_ptr(), text_end_.as_ptr())}
	}
	
	pub fn add_text__font_ptr(&self, font: *const sys::ImFont, font_size: f32, pos: [f32; 2], col: u32, text_begin: &str, text_end: &str, wrap_width: f32, cpu_fine_clip_rect: *const [f32; 4]) {
		let text_begin_ = CString::new(text_begin).unwrap();
		let text_end_ = CString::new(text_end).unwrap();
		unsafe{sys::ImDrawList_AddText_FontPtr(self.drawlist, font, font_size, pos, col, text_begin_.as_ptr(), text_end_.as_ptr(), wrap_width, cpu_fine_clip_rect)}
	}
	
	pub fn add_polyline(&self, points: *const [f32; 2], num_points: i32, col: u32, flags: DrawFlags, thickness: f32) {
		unsafe{sys::ImDrawList_AddPolyline(self.drawlist, points, num_points, col, flags.bits, thickness)}
	}
	
	// pub fn add_convex_poly_filled(&self, points: *const [f32; 2], num_points: i32, col: u32) {
	pub fn add_convex_poly_filled(&self, points: &[[f32; 2]], col: u32) {
		let len = points.len() as i32;
		unsafe{sys::ImDrawList_AddConvexPolyFilled(self.drawlist, points.as_ptr(), len, col)}
	}
	
	pub fn add_bezier_cubic(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], col: u32, thickness: f32, num_segments: i32) {
		unsafe{sys::ImDrawList_AddBezierCubic(self.drawlist, p1, p2, p3, p4, col, thickness, num_segments)}
	}
	
	pub fn add_bezier_quadratic(&self, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], col: u32, thickness: f32, num_segments: i32) {
		unsafe{sys::ImDrawList_AddBezierQuadratic(self.drawlist, p1, p2, p3, col, thickness, num_segments)}
	}
	
	pub fn add_image(&self, user_texture_id: usize, p_min: [f32; 2], p_max: [f32; 2], uv_min: [f32; 2], uv_max: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_AddImage(self.drawlist, user_texture_id as *mut _, p_min, p_max, uv_min, uv_max, col)}
	}
	
	pub fn add_image_quad(&self, user_texture_id: usize, p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], uv1: [f32; 2], uv2: [f32; 2], uv3: [f32; 2], uv4: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_AddImageQuad(self.drawlist, user_texture_id as *mut _, p1, p2, p3, p4, uv1, uv2, uv3, uv4, col)}
	}
	
	pub fn add_image_rounded(&self, user_texture_id: usize, p_min: [f32; 2], p_max: [f32; 2], uv_min: [f32; 2], uv_max: [f32; 2], col: u32, rounding: f32, flags: DrawFlags) {
		unsafe{sys::ImDrawList_AddImageRounded(self.drawlist, user_texture_id as *mut _, p_min, p_max, uv_min, uv_max, col, rounding, flags.bits)}
	}
	
	pub fn path_clear(&self) {
		unsafe{sys::ImDrawList_PathClear(self.drawlist)}
	}
	
	pub fn path_line_to(&self, pos: [f32; 2]) {
		unsafe{sys::ImDrawList_PathLineTo(self.drawlist, pos)}
	}
	
	pub fn path_line_to_merge_duplicate(&self, pos: [f32; 2]) {
		unsafe{sys::ImDrawList_PathLineToMergeDuplicate(self.drawlist, pos)}
	}
	
	pub fn path_fill_convex(&self, col: u32) {
		unsafe{sys::ImDrawList_PathFillConvex(self.drawlist, col)}
	}
	
	pub fn path_stroke(&self, col: u32, flags: DrawFlags, thickness: f32) {
		unsafe{sys::ImDrawList_PathStroke(self.drawlist, col, flags.bits, thickness)}
	}
	
	pub fn path_arc_to(&self, center: [f32; 2], radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
		unsafe{sys::ImDrawList_PathArcTo(self.drawlist, center, radius, a_min, a_max, num_segments)}
	}
	
	pub fn path_arc_to_fast(&self, center: [f32; 2], radius: f32, a_min_of_12: i32, a_max_of_12: i32) {
		unsafe{sys::ImDrawList_PathArcToFast(self.drawlist, center, radius, a_min_of_12, a_max_of_12)}
	}
	
	pub fn path_bezier_cubic_curve_to(&self, p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], num_segments: i32) {
		unsafe{sys::ImDrawList_PathBezierCubicCurveTo(self.drawlist, p2, p3, p4, num_segments)}
	}
	
	pub fn path_bezier_quadratic_curve_to(&self, p2: [f32; 2], p3: [f32; 2], num_segments: i32) {
		unsafe{sys::ImDrawList_PathBezierQuadraticCurveTo(self.drawlist, p2, p3, num_segments)}
	}
	
	pub fn path_rect(&self, rect_min: [f32; 2], rect_max: [f32; 2], rounding: f32, flags: DrawFlags) {
		unsafe{sys::ImDrawList_PathRect(self.drawlist, rect_min, rect_max, rounding, flags.bits)}
	}
	
	pub fn add_callback(&self, callback: sys::ImDrawCallback, callback_data: &mut ::std::os::raw::c_void) {
		unsafe{sys::ImDrawList_AddCallback(self.drawlist, callback, callback_data)}
	}
	
	pub fn add_draw_cmd(&self) {
		unsafe{sys::ImDrawList_AddDrawCmd(self.drawlist)}
	}
	
	pub fn clone_output(&self) -> *mut sys::ImDrawList {
		unsafe{sys::ImDrawList_CloneOutput(self.drawlist)}
	}
	
	pub fn channels_split(&self, count: i32) {
		unsafe{sys::ImDrawList_ChannelsSplit(self.drawlist, count)}
	}
	
	pub fn channels_merge(&self) {
		unsafe{sys::ImDrawList_ChannelsMerge(self.drawlist)}
	}
	
	pub fn channels_set_current(&self, n: i32) {
		unsafe{sys::ImDrawList_ChannelsSetCurrent(self.drawlist, n)}
	}
	
	pub fn prim_reserve(&mut self, idx_count: i32, vtx_count: i32) {
		unsafe{sys::ImDrawList_PrimReserve(self.drawlist, idx_count, vtx_count)}
		self.vtx_offset = unsafe{(*self.drawlist)._VtxCurrentIdx as u16}
	}
	
	pub fn prim_unreserve(&self, idx_count: i32, vtx_count: i32) {
		unsafe{sys::ImDrawList_PrimUnreserve(self.drawlist, idx_count, vtx_count)}
	}
	
	pub fn prim_rect(&self, a: [f32; 2], b: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_PrimRect(self.drawlist, a, b, col)}
	}
	
	pub fn prim_rect_uv(&self, a: [f32; 2], b: [f32; 2], uv_a: [f32; 2], uv_b: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_PrimRectUV(self.drawlist, a, b, uv_a, uv_b, col)}
	}
	
	pub fn prim_quad_uv(&self, a: [f32; 2], b: [f32; 2], c: [f32; 2], d: [f32; 2], uv_a: [f32; 2], uv_b: [f32; 2], uv_c: [f32; 2], uv_d: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_PrimQuadUV(self.drawlist, a, b, c, d, uv_a, uv_b, uv_c, uv_d, col)}
	}
	
	pub fn prim_write_vtx(&self, pos: [f32; 2], uv: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_PrimWriteVtx(self.drawlist, pos, uv, col)}
	}
	
	pub fn prim_write_idx(&self, idx: u16) {
		unsafe{sys::ImDrawList_PrimWriteIdx(self.drawlist, self.vtx_offset + idx)}
	}
	
	pub fn prim_vtx(&self, pos: [f32; 2], uv: [f32; 2], col: u32) {
		unsafe{sys::ImDrawList_PrimVtx(self.drawlist, pos, uv, col)}
	}
}

pub fn im_draw_list___reset_for_new_frame(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__ResetForNewFrame(self_)}
}

pub fn im_draw_list___clear_free_memory(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__ClearFreeMemory(self_)}
}

pub fn im_draw_list___pop_unused_draw_cmd(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__PopUnusedDrawCmd(self_)}
}

pub fn im_draw_list___try_merge_draw_cmds(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__TryMergeDrawCmds(self_)}
}

pub fn im_draw_list___on_changed_clip_rect(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__OnChangedClipRect(self_)}
}

pub fn im_draw_list___on_changed_texture_id(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__OnChangedTextureID(self_)}
}

pub fn im_draw_list___on_changed_vtx_offset(self_: &mut sys::ImDrawList) {
	unsafe{sys::ImDrawList__OnChangedVtxOffset(self_)}
}

pub fn im_draw_list___calc_circle_auto_segment_count(self_: &mut sys::ImDrawList, radius: f32) -> i32 {
	unsafe{sys::ImDrawList__CalcCircleAutoSegmentCount(self_, radius)}
}

pub fn im_draw_list___path_arc_to_fast_ex(self_: &mut sys::ImDrawList, center: [f32; 2], radius: f32, a_min_sample: i32, a_max_sample: i32, a_step: i32) {
	unsafe{sys::ImDrawList__PathArcToFastEx(self_, center, radius, a_min_sample, a_max_sample, a_step)}
}

pub fn im_draw_list___path_arc_to_n(self_: &mut sys::ImDrawList, center: [f32; 2], radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
	unsafe{sys::ImDrawList__PathArcToN(self_, center, radius, a_min, a_max, num_segments)}
}

pub fn im_hash_data(data: *const ::std::os::raw::c_void, data_size: sys::size_t, seed: u32) -> sys::ImGuiID {
	unsafe{sys::igImHashData(data, data_size, seed)}
}

pub fn im_hash_str(data: &str, data_size: sys::size_t, seed: u32) -> sys::ImGuiID {
	let data_ = CString::new(data).unwrap();
	unsafe{sys::igImHashStr(data_.as_ptr(), data_size, seed)}
}

pub fn im_alpha_blend_colors(col_a: u32, col_b: u32) -> u32 {
	unsafe{sys::igImAlphaBlendColors(col_a, col_b)}
}

pub fn im_is_power_of_two__int(v: i32) -> bool {
	unsafe{sys::igImIsPowerOfTwo_Int(v)}
}

pub fn im_is_power_of_two__u64(v: sys::ImU64) -> bool {
	unsafe{sys::igImIsPowerOfTwo_U64(v)}
}

pub fn im_upper_power_of_two(v: i32) -> i32 {
	unsafe{sys::igImUpperPowerOfTwo(v)}
}

pub fn im_stricmp(str1: &str, str2: &str) -> i32 {
	let str1_ = CString::new(str1).unwrap();
	let str2_ = CString::new(str2).unwrap();
	unsafe{sys::igImStricmp(str1_.as_ptr(), str2_.as_ptr())}
}

pub fn im_strnicmp(str1: &str, str2: &str, count: sys::size_t) -> i32 {
	let str1_ = CString::new(str1).unwrap();
	let str2_ = CString::new(str2).unwrap();
	unsafe{sys::igImStrnicmp(str1_.as_ptr(), str2_.as_ptr(), count)}
}

pub fn im_strncpy(dst: &mut ::std::os::raw::c_char, src: &str, count: sys::size_t) {
	let src_ = CString::new(src).unwrap();
	unsafe{sys::igImStrncpy(dst, src_.as_ptr(), count)}
}

pub fn im_strdup(str_: &str) -> *mut ::std::os::raw::c_char {
	let str__ = CString::new(str_).unwrap();
	unsafe{sys::igImStrdup(str__.as_ptr())}
}

pub fn im_strdupcpy(dst: &mut ::std::os::raw::c_char, p_dst_size: &mut sys::size_t, str_: &str) -> *mut ::std::os::raw::c_char {
	let str__ = CString::new(str_).unwrap();
	unsafe{sys::igImStrdupcpy(dst, p_dst_size, str__.as_ptr())}
}

pub fn im_strchr_range(str_begin: &str, str_end: &str, c: ::std::os::raw::c_char) -> *const ::std::os::raw::c_char {
	let str_begin_ = CString::new(str_begin).unwrap();
	let str_end_ = CString::new(str_end).unwrap();
	unsafe{sys::igImStrchrRange(str_begin_.as_ptr(), str_end_.as_ptr(), c)}
}

pub fn im_strlen_w(str_: *const sys::ImWchar) -> i32 {
	unsafe{sys::igImStrlenW(str_)}
}

pub fn im_streol_range(str_: &str, str_end: &str) -> *const ::std::os::raw::c_char {
	let str__ = CString::new(str_).unwrap();
	let str_end_ = CString::new(str_end).unwrap();
	unsafe{sys::igImStreolRange(str__.as_ptr(), str_end_.as_ptr())}
}

pub fn im_strbol_w(buf_mid_line: *const sys::ImWchar, buf_begin: *const sys::ImWchar) -> *const sys::ImWchar {
	unsafe{sys::igImStrbolW(buf_mid_line, buf_begin)}
}

pub fn im_stristr(haystack: &str, haystack_end: &str, needle: &str, needle_end: &str) -> *const ::std::os::raw::c_char {
	let haystack_ = CString::new(haystack).unwrap();
	let haystack_end_ = CString::new(haystack_end).unwrap();
	let needle_ = CString::new(needle).unwrap();
	let needle_end_ = CString::new(needle_end).unwrap();
	unsafe{sys::igImStristr(haystack_.as_ptr(), haystack_end_.as_ptr(), needle_.as_ptr(), needle_end_.as_ptr())}
}

pub fn im_str_trim_blanks(str_: &mut ::std::os::raw::c_char) {
	unsafe{sys::igImStrTrimBlanks(str_)}
}

pub fn im_str_skip_blank(str_: &str) -> *const ::std::os::raw::c_char {
	let str__ = CString::new(str_).unwrap();
	unsafe{sys::igImStrSkipBlank(str__.as_ptr())}
}

pub fn im_format_string(buf: &mut ::std::os::raw::c_char, buf_size: sys::size_t, fmt: &str) -> i32 {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igImFormatString(buf, buf_size, fmt_.as_ptr())}
}

pub fn im_format_string_v(buf: &mut ::std::os::raw::c_char, buf_size: sys::size_t, fmt: &str, args: sys::va_list) -> i32 {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igImFormatStringV(buf, buf_size, fmt_.as_ptr(), args)}
}

pub fn im_parse_format_find_start(format: &str) -> *const ::std::os::raw::c_char {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igImParseFormatFindStart(format_.as_ptr())}
}

pub fn im_parse_format_find_end(format: &str) -> *const ::std::os::raw::c_char {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igImParseFormatFindEnd(format_.as_ptr())}
}

pub fn im_parse_format_trim_decorations(format: &str, buf: &mut ::std::os::raw::c_char, buf_size: sys::size_t) -> *const ::std::os::raw::c_char {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igImParseFormatTrimDecorations(format_.as_ptr(), buf, buf_size)}
}

pub fn im_parse_format_precision(format: &str, default_value: i32) -> i32 {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igImParseFormatPrecision(format_.as_ptr(), default_value)}
}

pub fn im_char_is_blank_a(c: ::std::os::raw::c_char) -> bool {
	unsafe{sys::igImCharIsBlankA(c)}
}

pub fn im_char_is_blank_w(c: ::std::os::raw::c_uint) -> bool {
	unsafe{sys::igImCharIsBlankW(c)}
}

pub fn im_text_char_to_utf8(out_buf: &mut ::std::os::raw::c_char, c: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char {
	unsafe{sys::igImTextCharToUtf8(out_buf, c)}
}

pub fn im_text_str_to_utf8(out_buf: &mut ::std::os::raw::c_char, out_buf_size: i32, in_text: *const sys::ImWchar, in_text_end: *const sys::ImWchar) -> i32 {
	unsafe{sys::igImTextStrToUtf8(out_buf, out_buf_size, in_text, in_text_end)}
}

pub fn im_text_char_from_utf8(out_char: &mut ::std::os::raw::c_uint, in_text: &str, in_text_end: &str) -> i32 {
	let in_text_ = CString::new(in_text).unwrap();
	let in_text_end_ = CString::new(in_text_end).unwrap();
	unsafe{sys::igImTextCharFromUtf8(out_char, in_text_.as_ptr(), in_text_end_.as_ptr())}
}

pub fn im_text_str_from_utf8(out_buf: &mut sys::ImWchar, out_buf_size: i32, in_text: &str, in_text_end: &str, in_remaining: &mut *const ::std::os::raw::c_char) -> i32 {
	let in_text_ = CString::new(in_text).unwrap();
	let in_text_end_ = CString::new(in_text_end).unwrap();
	unsafe{sys::igImTextStrFromUtf8(out_buf, out_buf_size, in_text_.as_ptr(), in_text_end_.as_ptr(), in_remaining)}
}

pub fn im_text_count_chars_from_utf8(in_text: &str, in_text_end: &str) -> i32 {
	let in_text_ = CString::new(in_text).unwrap();
	let in_text_end_ = CString::new(in_text_end).unwrap();
	unsafe{sys::igImTextCountCharsFromUtf8(in_text_.as_ptr(), in_text_end_.as_ptr())}
}

pub fn im_text_count_utf8_bytes_from_char(in_text: &str, in_text_end: &str) -> i32 {
	let in_text_ = CString::new(in_text).unwrap();
	let in_text_end_ = CString::new(in_text_end).unwrap();
	unsafe{sys::igImTextCountUtf8BytesFromChar(in_text_.as_ptr(), in_text_end_.as_ptr())}
}

pub fn im_text_count_utf8_bytes_from_str(in_text: *const sys::ImWchar, in_text_end: *const sys::ImWchar) -> i32 {
	unsafe{sys::igImTextCountUtf8BytesFromStr(in_text, in_text_end)}
}

pub fn im_file_open(filename: &str, mode: &str) -> sys::ImFileHandle {
	let filename_ = CString::new(filename).unwrap();
	let mode_ = CString::new(mode).unwrap();
	unsafe{sys::igImFileOpen(filename_.as_ptr(), mode_.as_ptr())}
}

pub fn im_file_close(file: sys::ImFileHandle) -> bool {
	unsafe{sys::igImFileClose(file)}
}

pub fn im_file_get_size(file: sys::ImFileHandle) -> sys::ImU64 {
	unsafe{sys::igImFileGetSize(file)}
}

pub fn im_file_read(data: &mut ::std::os::raw::c_void, size: sys::ImU64, count: sys::ImU64, file: sys::ImFileHandle) -> sys::ImU64 {
	unsafe{sys::igImFileRead(data, size, count, file)}
}

pub fn im_file_write(data: *const ::std::os::raw::c_void, size: sys::ImU64, count: sys::ImU64, file: sys::ImFileHandle) -> sys::ImU64 {
	unsafe{sys::igImFileWrite(data, size, count, file)}
}

pub fn im_file_load_to_memory(filename: &str, mode: &str, out_file_size: &mut sys::size_t, padding_bytes: i32) -> *mut ::std::os::raw::c_void {
	let filename_ = CString::new(filename).unwrap();
	let mode_ = CString::new(mode).unwrap();
	unsafe{sys::igImFileLoadToMemory(filename_.as_ptr(), mode_.as_ptr(), out_file_size, padding_bytes)}
}

pub fn im_pow__float(x: f32, y: f32) -> f32 {
	unsafe{sys::igImPow_Float(x, y)}
}

pub fn im_pow_double(x: f64, y: f64) -> f64 {
	unsafe{sys::igImPow_double(x, y)}
}

pub fn im_log__float(x: f32) -> f32 {
	unsafe{sys::igImLog_Float(x)}
}

pub fn im_log_double(x: f64) -> f64 {
	unsafe{sys::igImLog_double(x)}
}

pub fn im_abs__int(x: i32) -> i32 {
	unsafe{sys::igImAbs_Int(x)}
}

pub fn im_abs__float(x: f32) -> f32 {
	unsafe{sys::igImAbs_Float(x)}
}

pub fn im_abs_double(x: f64) -> f64 {
	unsafe{sys::igImAbs_double(x)}
}

pub fn im_sign__float(x: f32) -> f32 {
	unsafe{sys::igImSign_Float(x)}
}

pub fn im_sign_double(x: f64) -> f64 {
	unsafe{sys::igImSign_double(x)}
}

pub fn im_rsqrt__float(x: f32) -> f32 {
	unsafe{sys::igImRsqrt_Float(x)}
}

pub fn im_rsqrt_double(x: f64) -> f64 {
	unsafe{sys::igImRsqrt_double(x)}
}

pub fn im_min(pOut: &mut [f32; 2], lhs: [f32; 2], rhs: [f32; 2]) {
	unsafe{sys::igImMin(pOut, lhs, rhs)}
}

pub fn im_max(pOut: &mut [f32; 2], lhs: [f32; 2], rhs: [f32; 2]) {
	unsafe{sys::igImMax(pOut, lhs, rhs)}
}

pub fn im_clamp(pOut: &mut [f32; 2], v: [f32; 2], mn: [f32; 2], mx: [f32; 2]) {
	unsafe{sys::igImClamp(pOut, v, mn, mx)}
}

pub fn im_lerp__vec2_float(pOut: &mut [f32; 2], a: [f32; 2], b: [f32; 2], t: f32) {
	unsafe{sys::igImLerp_Vec2Float(pOut, a, b, t)}
}

pub fn im_lerp__vec2_vec2(pOut: &mut [f32; 2], a: [f32; 2], b: [f32; 2], t: [f32; 2]) {
	unsafe{sys::igImLerp_Vec2Vec2(pOut, a, b, t)}
}

pub fn im_lerp__vec4(pOut: &mut [f32; 4], a: [f32; 4], b: [f32; 4], t: f32) {
	unsafe{sys::igImLerp_Vec4(pOut, a, b, t)}
}

pub fn im_saturate(f: f32) -> f32 {
	unsafe{sys::igImSaturate(f)}
}

pub fn im_length_sqr__vec2(lhs: [f32; 2]) -> f32 {
	unsafe{sys::igImLengthSqr_Vec2(lhs)}
}

pub fn im_length_sqr__vec4(lhs: [f32; 4]) -> f32 {
	unsafe{sys::igImLengthSqr_Vec4(lhs)}
}

pub fn im_inv_length(lhs: [f32; 2], fail_value: f32) -> f32 {
	unsafe{sys::igImInvLength(lhs, fail_value)}
}

pub fn im_floor__float(f: f32) -> f32 {
	unsafe{sys::igImFloor_Float(f)}
}

// pub fn im_floor_signed(f: f32) -> f32 {
// 	unsafe{sys::igImFloorSigned(f)}
// }

pub fn im_floor__vec2(pOut: &mut [f32; 2], v: [f32; 2]) {
	unsafe{sys::igImFloor_Vec2(pOut, v)}
}

pub fn im_mod_positive(a: i32, b: i32) -> i32 {
	unsafe{sys::igImModPositive(a, b)}
}

pub fn im_dot(a: [f32; 2], b: [f32; 2]) -> f32 {
	unsafe{sys::igImDot(a, b)}
}

pub fn im_rotate(pOut: &mut [f32; 2], v: [f32; 2], cos_a: f32, sin_a: f32) {
	unsafe{sys::igImRotate(pOut, v, cos_a, sin_a)}
}

pub fn im_linear_sweep(current: f32, target: f32, speed: f32) -> f32 {
	unsafe{sys::igImLinearSweep(current, target, speed)}
}

pub fn im_mul(pOut: &mut [f32; 2], lhs: [f32; 2], rhs: [f32; 2]) {
	unsafe{sys::igImMul(pOut, lhs, rhs)}
}

pub fn im_bezier_cubic_calc(pOut: &mut [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], t: f32) {
	unsafe{sys::igImBezierCubicCalc(pOut, p1, p2, p3, p4, t)}
}

pub fn im_bezier_cubic_closest_point(pOut: &mut [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], p: [f32; 2], num_segments: i32) {
	unsafe{sys::igImBezierCubicClosestPoint(pOut, p1, p2, p3, p4, p, num_segments)}
}

pub fn im_bezier_cubic_closest_point_casteljau(pOut: &mut [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], p4: [f32; 2], p: [f32; 2], tess_tol: f32) {
	unsafe{sys::igImBezierCubicClosestPointCasteljau(pOut, p1, p2, p3, p4, p, tess_tol)}
}

pub fn im_bezier_quadratic_calc(pOut: &mut [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], t: f32) {
	unsafe{sys::igImBezierQuadraticCalc(pOut, p1, p2, p3, t)}
}

pub fn im_line_closest_point(pOut: &mut [f32; 2], a: [f32; 2], b: [f32; 2], p: [f32; 2]) {
	unsafe{sys::igImLineClosestPoint(pOut, a, b, p)}
}

pub fn im_triangle_contains_point(a: [f32; 2], b: [f32; 2], c: [f32; 2], p: [f32; 2]) -> bool {
	unsafe{sys::igImTriangleContainsPoint(a, b, c, p)}
}

pub fn im_triangle_closest_point(pOut: &mut [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2], p: [f32; 2]) {
	unsafe{sys::igImTriangleClosestPoint(pOut, a, b, c, p)}
}

pub fn im_triangle_barycentric_coords(a: [f32; 2], b: [f32; 2], c: [f32; 2], p: [f32; 2], out_u: &mut f32, out_v: &mut f32, out_w: &mut f32) {
	unsafe{sys::igImTriangleBarycentricCoords(a, b, c, p, out_u, out_v, out_w)}
}

pub fn im_triangle_area(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
	unsafe{sys::igImTriangleArea(a, b, c)}
}

pub fn im_get_dir_quadrant_from_delta(dx: f32, dy: f32) -> i32 {
	unsafe{sys::igImGetDirQuadrantFromDelta(dx, dy)}
}

pub fn im_bit_array_test_bit(arr: *const u32, n: i32) -> bool {
	unsafe{sys::igImBitArrayTestBit(arr, n)}
}

pub fn im_bit_array_clear_bit(arr: &mut u32, n: i32) {
	unsafe{sys::igImBitArrayClearBit(arr, n)}
}

pub fn im_bit_array_set_bit(arr: &mut u32, n: i32) {
	unsafe{sys::igImBitArraySetBit(arr, n)}
}

pub fn im_bit_array_set_bit_range(arr: &mut u32, n: i32, n2: i32) {
	unsafe{sys::igImBitArraySetBitRange(arr, n, n2)}
}

pub fn im_draw_list_shared_data__im_draw_list_shared_data() -> *mut sys::ImDrawListSharedData {
	unsafe{sys::ImDrawListSharedData_ImDrawListSharedData()}
}

pub fn im_draw_list_shared_data_destroy(self_: &mut sys::ImDrawListSharedData) {
	unsafe{sys::ImDrawListSharedData_destroy(self_)}
}

pub fn im_draw_list_shared_data__set_circle_tessellation_max_error(self_: &mut sys::ImDrawListSharedData, max_error: f32) {
	unsafe{sys::ImDrawListSharedData_SetCircleTessellationMaxError(self_, max_error)}
}

pub fn get_current_window_read() -> &'static mut sys::ImGuiWindow {
	unsafe{&mut *sys::igGetCurrentWindowRead()}
}

pub fn get_current_window() -> &'static mut sys::ImGuiWindow {
	unsafe{&mut *sys::igGetCurrentWindow()}
}

pub fn find_window_by_id(id: sys::ImGuiID) -> *mut sys::ImGuiWindow {
	unsafe{sys::igFindWindowByID(id)}
}

pub fn find_window_by_name(name: &str) -> *mut sys::ImGuiWindow {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igFindWindowByName(name_.as_ptr())}
}

pub fn update_window_parent_and_root_links(window: &mut sys::ImGuiWindow, flags: WindowFlags, parent_window: &mut sys::ImGuiWindow) {
	unsafe{sys::igUpdateWindowParentAndRootLinks(window, flags.bits, parent_window)}
}

pub fn calc_window_next_auto_fit_size(pOut: &mut [f32; 2], window: &mut sys::ImGuiWindow) {
	unsafe{sys::igCalcWindowNextAutoFitSize(pOut, window)}
}

// pub fn is_window_child_of(window: &mut sys::ImGuiWindow, potential_parent: &mut sys::ImGuiWindow) -> bool {
// 	unsafe{sys::igIsWindowChildOf(window, potential_parent)}
// }

pub fn is_window_above(potential_above: &mut sys::ImGuiWindow, potential_below: &mut sys::ImGuiWindow) -> bool {
	unsafe{sys::igIsWindowAbove(potential_above, potential_below)}
}

pub fn is_window_nav_focusable(window: &mut sys::ImGuiWindow) -> bool {
	unsafe{sys::igIsWindowNavFocusable(window)}
}

pub fn set_window_pos__window_ptr(window: &mut sys::ImGuiWindow, pos: [f32; 2], cond: Cond) {
	unsafe{sys::igSetWindowPos_WindowPtr(window, pos, cond as i32)}
}

pub fn set_window_size__window_ptr(window: &mut sys::ImGuiWindow, size: [f32; 2], cond: Cond) {
	unsafe{sys::igSetWindowSize_WindowPtr(window, size, cond as i32)}
}

pub fn set_window_collapsed__window_ptr(window: &mut sys::ImGuiWindow, collapsed: bool, cond: Cond) {
	unsafe{sys::igSetWindowCollapsed_WindowPtr(window, collapsed, cond as i32)}
}

pub fn set_window_hit_test_hole(window: &mut sys::ImGuiWindow, pos: [f32; 2], size: [f32; 2]) {
	unsafe{sys::igSetWindowHitTestHole(window, pos, size)}
}

pub fn focus_window(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igFocusWindow(window)}
}

pub fn focus_top_most_window_under_one(under_this_window: &mut sys::ImGuiWindow, ignore_window: &mut sys::ImGuiWindow) {
	unsafe{sys::igFocusTopMostWindowUnderOne(under_this_window, ignore_window)}
}

pub fn bring_window_to_focus_front(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igBringWindowToFocusFront(window)}
}

pub fn bring_window_to_display_front(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igBringWindowToDisplayFront(window)}
}

pub fn bring_window_to_display_back(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igBringWindowToDisplayBack(window)}
}

pub fn set_current_font(font: &mut sys::ImFont) {
	unsafe{sys::igSetCurrentFont(font)}
}

pub fn get_default_font() -> *mut sys::ImFont {
	unsafe{sys::igGetDefaultFont()}
}

pub fn get_foreground_draw_list__window_ptr(window: &mut sys::ImGuiWindow) -> *mut sys::ImDrawList {
	unsafe{sys::igGetForegroundDrawList_WindowPtr(window)}
}

pub fn get_background_draw_list__viewport_ptr(viewport: &mut sys::ImGuiViewport) -> *mut sys::ImDrawList {
	unsafe{sys::igGetBackgroundDrawList_ViewportPtr(viewport)}
}

pub fn get_foreground_draw_list__viewport_ptr(viewport: &mut sys::ImGuiViewport) -> *mut sys::ImDrawList {
	unsafe{sys::igGetForegroundDrawList_ViewportPtr(viewport)}
}

// pub fn initialize(context: &mut sys::ImGuiContext) {
// 	unsafe{sys::igInitialize(context)}
// }

// pub fn shutdown(context: &mut sys::ImGuiContext) {
// 	unsafe{sys::igShutdown(context)}
// }

pub fn update_hovered_window_and_capture_flags() {
	unsafe{sys::igUpdateHoveredWindowAndCaptureFlags()}
}

pub fn start_mouse_moving_window(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igStartMouseMovingWindow(window)}
}

pub fn update_mouse_moving_window_new_frame() {
	unsafe{sys::igUpdateMouseMovingWindowNewFrame()}
}

pub fn update_mouse_moving_window_end_frame() {
	unsafe{sys::igUpdateMouseMovingWindowEndFrame()}
}

pub fn add_context_hook(context: &mut sys::ImGuiContext, hook: *const sys::ImGuiContextHook) -> sys::ImGuiID {
	unsafe{sys::igAddContextHook(context, hook)}
}

pub fn remove_context_hook(context: &mut sys::ImGuiContext, hook_to_remove: sys::ImGuiID) {
	unsafe{sys::igRemoveContextHook(context, hook_to_remove)}
}

pub fn call_context_hooks(context: &mut sys::ImGuiContext, type_: sys::ImGuiContextHookType) {
	unsafe{sys::igCallContextHooks(context, type_)}
}

pub fn mark_ini_settings_dirty__nil() {
	unsafe{sys::igMarkIniSettingsDirty_Nil()}
}

pub fn mark_ini_settings_dirty__window_ptr(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igMarkIniSettingsDirty_WindowPtr(window)}
}

pub fn clear_ini_settings() {
	unsafe{sys::igClearIniSettings()}
}

pub fn create_new_window_settings(name: &str) -> *mut sys::ImGuiWindowSettings {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igCreateNewWindowSettings(name_.as_ptr())}
}

pub fn find_window_settings(id: sys::ImGuiID) -> *mut sys::ImGuiWindowSettings {
	unsafe{sys::igFindWindowSettings(id)}
}

pub fn find_or_create_window_settings(name: &str) -> *mut sys::ImGuiWindowSettings {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igFindOrCreateWindowSettings(name_.as_ptr())}
}

pub fn find_settings_handler(type_name: &str) -> *mut sys::ImGuiSettingsHandler {
	let type_name_ = CString::new(type_name).unwrap();
	unsafe{sys::igFindSettingsHandler(type_name_.as_ptr())}
}

pub fn set_next_window_scroll(scroll: [f32; 2]) {
	unsafe{sys::igSetNextWindowScroll(scroll)}
}

pub fn set_scroll_x__window_ptr(window: &mut sys::ImGuiWindow, scroll_x: f32) {
	unsafe{sys::igSetScrollX_WindowPtr(window, scroll_x)}
}

pub fn set_scroll_y__window_ptr(window: &mut sys::ImGuiWindow, scroll_y: f32) {
	unsafe{sys::igSetScrollY_WindowPtr(window, scroll_y)}
}

pub fn set_scroll_from_pos_x__window_ptr(window: &mut sys::ImGuiWindow, local_x: f32, center_x_ratio: f32) {
	unsafe{sys::igSetScrollFromPosX_WindowPtr(window, local_x, center_x_ratio)}
}

pub fn set_scroll_from_pos_y__window_ptr(window: &mut sys::ImGuiWindow, local_y: f32, center_y_ratio: f32) {
	unsafe{sys::igSetScrollFromPosY_WindowPtr(window, local_y, center_y_ratio)}
}

// pub fn scroll_to_bring_rect_into_view(pOut: &mut [f32; 2], window: &mut sys::ImGuiWindow, item_rect: sys::ImRect) {
// 	unsafe{sys::igScrollToBringRectIntoView(pOut, window, item_rect)}
// }

pub fn get_item_id() -> sys::ImGuiID {
	unsafe{sys::igGetItemID()}
}

pub fn get_item_status_flags() -> i32 {
	unsafe{sys::igGetItemStatusFlags()}
}

pub fn get_item_flags() -> i32 {
	unsafe{sys::igGetItemFlags()}
}

pub fn get_active_id() -> sys::ImGuiID {
	unsafe{sys::igGetActiveID()}
}

pub fn get_focus_id() -> sys::ImGuiID {
	unsafe{sys::igGetFocusID()}
}

pub fn set_active_id(id: sys::ImGuiID, window: &mut sys::ImGuiWindow) {
	unsafe{sys::igSetActiveID(id, window)}
}

pub fn set_focus_id(id: sys::ImGuiID, window: &mut sys::ImGuiWindow) {
	unsafe{sys::igSetFocusID(id, window)}
}

pub fn clear_active_id() {
	unsafe{sys::igClearActiveID()}
}

pub fn get_hovered_id() -> sys::ImGuiID {
	unsafe{sys::igGetHoveredID()}
}

pub fn set_hovered_id(id: sys::ImGuiID) {
	unsafe{sys::igSetHoveredID(id)}
}

pub fn keep_alive_id(id: sys::ImGuiID) {
	unsafe{sys::igKeepAliveID(id)}
}

pub fn mark_item_edited(id: sys::ImGuiID) {
	unsafe{sys::igMarkItemEdited(id)}
}

pub fn push_override_id(id: sys::ImGuiID) {
	unsafe{sys::igPushOverrideID(id)}
}

pub fn get_idwith_seed(str_id_begin: &str, str_id_end: &str, seed: sys::ImGuiID) -> sys::ImGuiID {
	let str_id_begin_ = CString::new(str_id_begin).unwrap();
	let str_id_end_ = CString::new(str_id_end).unwrap();
	unsafe{sys::igGetIDWithSeed(str_id_begin_.as_ptr(), str_id_end_.as_ptr(), seed)}
}

pub fn item_size__vec2(size: [f32; 2], text_baseline_y: f32) {
	unsafe{sys::igItemSize_Vec2(size, text_baseline_y)}
}

pub fn item_size__rect(bb: sys::ImRect, text_baseline_y: f32) {
	unsafe{sys::igItemSize_Rect(bb, text_baseline_y)}
}

pub fn item_add(bb: sys::ImRect, id: sys::ImGuiID, nav_bb: *const sys::ImRect, flags: ItemFlags) -> bool {
	unsafe{sys::igItemAdd(bb, id, nav_bb, flags.bits)}
}

pub fn item_hoverable(bb: sys::ImRect, id: sys::ImGuiID) -> bool {
	unsafe{sys::igItemHoverable(bb, id)}
}

// pub fn item_focusable(window: &mut sys::ImGuiWindow, id: sys::ImGuiID) {
// 	unsafe{sys::igItemFocusable(window, id)}
// }

// pub fn is_clipped_ex(bb: sys::ImRect, id: sys::ImGuiID, clip_even_when_logged: bool) -> bool {
// 	unsafe{sys::igIsClippedEx(bb, id, clip_even_when_logged)}
// }

pub fn calc_item_size(size: [f32; 2], default_w: f32, default_h: f32) -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igCalcItemSize(&mut r, size, default_w, default_h)}
	r
}

pub fn calc_wrap_width_for_pos(pos: [f32; 2], wrap_pos_x: f32) -> f32 {
	unsafe{sys::igCalcWrapWidthForPos(pos, wrap_pos_x)}
}

pub fn push_multi_items_widths(components: i32, width_full: f32) {
	unsafe{sys::igPushMultiItemsWidths(components, width_full)}
}

pub fn is_item_toggled_selection() -> bool {
	unsafe{sys::igIsItemToggledSelection()}
}

pub fn get_content_region_max_abs() -> [f32; 2] {
	let mut r = [0f32; 2];
	unsafe{sys::igGetContentRegionMaxAbs(&mut r)}
	r
}

pub fn shrink_widths(items: &mut sys::ImGuiShrinkWidthItem, count: i32, width_excess: f32) {
	unsafe{sys::igShrinkWidths(items, count, width_excess)}
}

pub fn push_item_flag(option: ItemFlags, enabled: bool) {
	unsafe{sys::igPushItemFlag(option.bits, enabled)}
}

pub fn pop_item_flag() {
	unsafe{sys::igPopItemFlag()}
}

pub fn log_begin(type_: sys::ImGuiLogType, auto_open_depth: i32) {
	unsafe{sys::igLogBegin(type_, auto_open_depth)}
}

pub fn log_to_buffer(auto_open_depth: i32) {
	unsafe{sys::igLogToBuffer(auto_open_depth)}
}

pub fn log_rendered_text(ref_pos: *const [f32; 2], text: &str, text_end: &str) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igLogRenderedText(ref_pos, text_.as_ptr(), text_end_.as_ptr())}
}

pub fn log_set_next_text_decoration(prefix: &str, suffix: &str) {
	let prefix_ = CString::new(prefix).unwrap();
	let suffix_ = CString::new(suffix).unwrap();
	unsafe{sys::igLogSetNextTextDecoration(prefix_.as_ptr(), suffix_.as_ptr())}
}

pub fn begin_child_ex(name: &str, id: sys::ImGuiID, size_arg: [f32; 2], border: bool, flags: WindowFlags) -> bool {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igBeginChildEx(name_.as_ptr(), id, size_arg, border, flags.bits)}
}

pub fn open_popup_ex(id: sys::ImGuiID, popup_flags: PopupFlags) {
	unsafe{sys::igOpenPopupEx(id, popup_flags.bits)}
}

pub fn close_popup_to_level(remaining: i32, restore_focus_to_window_under_popup: bool) {
	unsafe{sys::igClosePopupToLevel(remaining, restore_focus_to_window_under_popup)}
}

pub fn close_popups_over_window(ref_window: &mut sys::ImGuiWindow, restore_focus_to_window_under_popup: bool) {
	unsafe{sys::igClosePopupsOverWindow(ref_window, restore_focus_to_window_under_popup)}
}

pub fn is_popup_open__id(id: sys::ImGuiID, popup_flags: PopupFlags) -> bool {
	unsafe{sys::igIsPopupOpen_ID(id, popup_flags.bits)}
}

pub fn begin_popup_ex(id: sys::ImGuiID, extra_flags: WindowFlags) -> bool {
	unsafe{sys::igBeginPopupEx(id, extra_flags.bits)}
}

pub fn begin_tooltip_ex(extra_flags: WindowFlags, tooltip_flags: TooltipFlags) {
	unsafe{sys::igBeginTooltipEx(extra_flags.bits, tooltip_flags.bits)}
}

pub fn get_popup_allowed_extent_rect(pOut: &mut sys::ImRect, window: &mut sys::ImGuiWindow) {
	unsafe{sys::igGetPopupAllowedExtentRect(pOut, window)}
}

pub fn get_top_most_popup_modal() -> *mut sys::ImGuiWindow {
	unsafe{sys::igGetTopMostPopupModal()}
}

pub fn find_best_window_pos_for_popup(pOut: &mut [f32; 2], window: &mut sys::ImGuiWindow) {
	unsafe{sys::igFindBestWindowPosForPopup(pOut, window)}
}

pub fn find_best_window_pos_for_popup_ex(pOut: &mut [f32; 2], ref_pos: [f32; 2], size: [f32; 2], last_dir: *mut Dir, r_outer: sys::ImRect, r_avoid: sys::ImRect, policy: sys::ImGuiPopupPositionPolicy) {
	unsafe{sys::igFindBestWindowPosForPopupEx(pOut, ref_pos, size, last_dir as *mut i32, r_outer, r_avoid, policy)}
}

pub fn begin_viewport_side_bar(name: &str, viewport: &mut sys::ImGuiViewport, dir: Dir, size: f32, window_flags: WindowFlags) -> bool {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igBeginViewportSideBar(name_.as_ptr(), viewport, dir as i32, size, window_flags.bits)}
}

pub fn menu_item_ex(label: &str, icon: &str, shortcut: &str, selected: bool, enabled: bool) -> bool {
	let label_ = CString::new(label).unwrap();
	let icon_ = CString::new(icon).unwrap();
	let shortcut_ = CString::new(shortcut).unwrap();
	unsafe{sys::igMenuItemEx(label_.as_ptr(), icon_.as_ptr(), shortcut_.as_ptr(), selected, enabled)}
}

pub fn begin_combo_popup(popup_id: sys::ImGuiID, bb: sys::ImRect, flags: ComboFlags) -> bool {
	unsafe{sys::igBeginComboPopup(popup_id, bb, flags.bits)}
}

pub fn begin_combo_preview() -> bool {
	unsafe{sys::igBeginComboPreview()}
}

pub fn end_combo_preview() {
	unsafe{sys::igEndComboPreview()}
}

pub fn nav_init_window(window: &mut sys::ImGuiWindow, force_reinit: bool) {
	unsafe{sys::igNavInitWindow(window, force_reinit)}
}

pub fn nav_move_request_but_no_result_yet() -> bool {
	unsafe{sys::igNavMoveRequestButNoResultYet()}
}

pub fn nav_move_request_cancel() {
	unsafe{sys::igNavMoveRequestCancel()}
}

// pub fn nav_move_request_forward(move_dir: Dir, clip_dir: Dir, bb_rel: sys::ImRect, move_flags: NavMoveFlags) {
// 	unsafe{sys::igNavMoveRequestForward(move_dir as i32, clip_dir as i32, bb_rel, move_flags.bits)}
// }

pub fn nav_move_request_try_wrapping(window: &mut sys::ImGuiWindow, move_flags: NavMoveFlags) {
	unsafe{sys::igNavMoveRequestTryWrapping(window, move_flags.bits)}
}

// pub fn get_nav_input_amount(n: NavInput, mode: sys::ImGuiInputReadMode) -> f32 {
// 	unsafe{sys::igGetNavInputAmount(n as i32, mode)}
// }

// pub fn get_nav_input_amount2d(pOut: &mut [f32; 2], dir_sources: NavDirSourceFlags, mode: sys::ImGuiInputReadMode, slow_factor: f32, fast_factor: f32) {
// 	unsafe{sys::igGetNavInputAmount2d(pOut, dir_sources as i32, mode, slow_factor, fast_factor)}
// }

pub fn calc_typematic_repeat_amount(t0: f32, t1: f32, repeat_delay: f32, repeat_rate: f32) -> i32 {
	unsafe{sys::igCalcTypematicRepeatAmount(t0, t1, repeat_delay, repeat_rate)}
}

pub fn activate_item(id: sys::ImGuiID) {
	unsafe{sys::igActivateItem(id)}
}

pub fn set_nav_id(id: sys::ImGuiID, nav_layer: sys::ImGuiNavLayer, focus_scope_id: sys::ImGuiID, rect_rel: sys::ImRect) {
	unsafe{sys::igSetNavID(id, nav_layer, focus_scope_id, rect_rel)}
}

pub fn push_focus_scope(id: sys::ImGuiID) {
	unsafe{sys::igPushFocusScope(id)}
}

pub fn pop_focus_scope() {
	unsafe{sys::igPopFocusScope()}
}

pub fn get_focused_focus_scope() -> sys::ImGuiID {
	unsafe{sys::igGetFocusedFocusScope()}
}

pub fn get_focus_scope() -> sys::ImGuiID {
	unsafe{sys::igGetFocusScope()}
}

pub fn set_item_using_mouse_wheel() {
	unsafe{sys::igSetItemUsingMouseWheel()}
}

pub fn set_active_id_using_nav_and_keys() {
	unsafe{sys::igSetActiveIdUsingNavAndKeys()}
}

pub fn is_active_id_using_nav_dir(dir: Dir) -> bool {
	unsafe{sys::igIsActiveIdUsingNavDir(dir as i32)}
}

pub fn is_active_id_using_nav_input(input: NavInput) -> bool {
	unsafe{sys::igIsActiveIdUsingNavInput(input as i32)}
}

pub fn is_active_id_using_key(key: Key) -> bool {
	unsafe{sys::igIsActiveIdUsingKey(key as i32)}
}

pub fn is_mouse_drag_past_threshold(button: MouseButton, lock_threshold: f32) -> bool {
	unsafe{sys::igIsMouseDragPastThreshold(button as i32, lock_threshold)}
}

pub fn is_key_pressed_map(key: Key, repeat: bool) -> bool {
	unsafe{sys::igIsKeyPressedMap(key as i32, repeat)}
}

pub fn is_nav_input_down(n: NavInput) -> bool {
	unsafe{sys::igIsNavInputDown(n as i32)}
}

// pub fn is_nav_input_test(n: NavInput, rm: sys::ImGuiInputReadMode) -> bool {
// 	unsafe{sys::igIsNavInputTest(n as i32, rm)}
// }

// pub fn get_merged_key_mod_flags() -> i32 {
// 	unsafe{sys::igGetMergedKeyModFlags()}
// }

pub fn begin_drag_drop_target_custom(bb: sys::ImRect, id: sys::ImGuiID) -> bool {
	unsafe{sys::igBeginDragDropTargetCustom(bb, id)}
}

pub fn clear_drag_drop() {
	unsafe{sys::igClearDragDrop()}
}

pub fn is_drag_drop_payload_being_accepted() -> bool {
	unsafe{sys::igIsDragDropPayloadBeingAccepted()}
}

pub fn set_window_clip_rect_before_set_channel(window: &mut sys::ImGuiWindow, clip_rect: sys::ImRect) {
	unsafe{sys::igSetWindowClipRectBeforeSetChannel(window, clip_rect)}
}

pub fn begin_columns(str_id: &str, count: i32, flags: OldColumnFlags) {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igBeginColumns(str_id_.as_ptr(), count, flags.bits)}
}

pub fn end_columns() {
	unsafe{sys::igEndColumns()}
}

pub fn push_column_clip_rect(column_index: i32) {
	unsafe{sys::igPushColumnClipRect(column_index)}
}

pub fn push_columns_background() {
	unsafe{sys::igPushColumnsBackground()}
}

pub fn pop_columns_background() {
	unsafe{sys::igPopColumnsBackground()}
}

pub fn get_columns_id(str_id: &str, count: i32) -> sys::ImGuiID {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igGetColumnsID(str_id_.as_ptr(), count)}
}

pub fn find_or_create_columns(window: &mut sys::ImGuiWindow, id: sys::ImGuiID) -> *mut sys::ImGuiOldColumns {
	unsafe{sys::igFindOrCreateColumns(window, id)}
}

pub fn get_column_offset_from_norm(columns: *const sys::ImGuiOldColumns, offset_norm: f32) -> f32 {
	unsafe{sys::igGetColumnOffsetFromNorm(columns, offset_norm)}
}

pub fn get_column_norm_from_offset(columns: *const sys::ImGuiOldColumns, offset: f32) -> f32 {
	unsafe{sys::igGetColumnNormFromOffset(columns, offset)}
}

pub fn table_open_context_menu(column_n: i32) {
	unsafe{sys::igTableOpenContextMenu(column_n)}
}

pub fn table_set_column_width(column_n: i32, width: f32) {
	unsafe{sys::igTableSetColumnWidth(column_n, width)}
}

pub fn table_set_column_sort_direction(column_n: i32, sort_direction: SortDirection, append_to_sort_specs: bool) {
	unsafe{sys::igTableSetColumnSortDirection(column_n, sort_direction as i32, append_to_sort_specs)}
}

pub fn table_get_hovered_column() -> i32 {
	unsafe{sys::igTableGetHoveredColumn()}
}

pub fn table_get_header_row_height() -> f32 {
	unsafe{sys::igTableGetHeaderRowHeight()}
}

pub fn table_push_background_channel() {
	unsafe{sys::igTablePushBackgroundChannel()}
}

pub fn table_pop_background_channel() {
	unsafe{sys::igTablePopBackgroundChannel()}
}

pub fn get_current_table() -> *mut sys::ImGuiTable {
	unsafe{sys::igGetCurrentTable()}
}

pub fn table_find_by_id(id: sys::ImGuiID) -> *mut sys::ImGuiTable {
	unsafe{sys::igTableFindByID(id)}
}

pub fn begin_table_ex(name: &str, id: sys::ImGuiID, columns_count: i32, flags: TableFlags, outer_size: [f32; 2], inner_width: f32) -> bool {
	let name_ = CString::new(name).unwrap();
	unsafe{sys::igBeginTableEx(name_.as_ptr(), id, columns_count, flags.bits, outer_size, inner_width)}
}

pub fn table_begin_init_memory(table: &mut sys::ImGuiTable, columns_count: i32) {
	unsafe{sys::igTableBeginInitMemory(table, columns_count)}
}

pub fn table_begin_apply_requests(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableBeginApplyRequests(table)}
}

pub fn table_setup_draw_channels(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableSetupDrawChannels(table)}
}

pub fn table_update_layout(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableUpdateLayout(table)}
}

pub fn table_update_borders(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableUpdateBorders(table)}
}

pub fn table_update_columns_weight_from_width(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableUpdateColumnsWeightFromWidth(table)}
}

pub fn table_draw_borders(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableDrawBorders(table)}
}

pub fn table_draw_context_menu(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableDrawContextMenu(table)}
}

pub fn table_merge_draw_channels(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableMergeDrawChannels(table)}
}

pub fn table_sort_specs_sanitize(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableSortSpecsSanitize(table)}
}

pub fn table_sort_specs_build(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableSortSpecsBuild(table)}
}

pub fn table_get_column_next_sort_direction(column: &mut sys::ImGuiTableColumn) -> i32 {
	unsafe{sys::igTableGetColumnNextSortDirection(column)}
}

pub fn table_fix_column_sort_direction(table: &mut sys::ImGuiTable, column: &mut sys::ImGuiTableColumn) {
	unsafe{sys::igTableFixColumnSortDirection(table, column)}
}

pub fn table_get_column_width_auto(table: &mut sys::ImGuiTable, column: &mut sys::ImGuiTableColumn) -> f32 {
	unsafe{sys::igTableGetColumnWidthAuto(table, column)}
}

pub fn table_begin_row(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableBeginRow(table)}
}

pub fn table_end_row(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableEndRow(table)}
}

pub fn table_begin_cell(table: &mut sys::ImGuiTable, column_n: i32) {
	unsafe{sys::igTableBeginCell(table, column_n)}
}

pub fn table_end_cell(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableEndCell(table)}
}

pub fn table_get_cell_bg_rect(pOut: &mut sys::ImRect, table: *const sys::ImGuiTable, column_n: i32) {
	unsafe{sys::igTableGetCellBgRect(pOut, table, column_n)}
}

pub fn table_get_column_name__table_ptr(table: *const sys::ImGuiTable, column_n: i32) -> *const ::std::os::raw::c_char {
	unsafe{sys::igTableGetColumnName_TablePtr(table, column_n)}
}

pub fn table_get_column_resize_id(table: *const sys::ImGuiTable, column_n: i32, instance_no: i32) -> sys::ImGuiID {
	unsafe{sys::igTableGetColumnResizeID(table, column_n, instance_no)}
}

pub fn table_get_max_column_width(table: *const sys::ImGuiTable, column_n: i32) -> f32 {
	unsafe{sys::igTableGetMaxColumnWidth(table, column_n)}
}

pub fn table_set_column_width_auto_single(table: &mut sys::ImGuiTable, column_n: i32) {
	unsafe{sys::igTableSetColumnWidthAutoSingle(table, column_n)}
}

pub fn table_set_column_width_auto_all(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableSetColumnWidthAutoAll(table)}
}

pub fn table_remove(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableRemove(table)}
}

pub fn table_gc_compact_transient_buffers__table_ptr(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableGcCompactTransientBuffers_TablePtr(table)}
}

pub fn table_gc_compact_transient_buffers__table_temp_data_ptr(table: &mut sys::ImGuiTableTempData) {
	unsafe{sys::igTableGcCompactTransientBuffers_TableTempDataPtr(table)}
}

pub fn table_gc_compact_settings() {
	unsafe{sys::igTableGcCompactSettings()}
}

pub fn table_load_settings(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableLoadSettings(table)}
}

pub fn table_save_settings(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableSaveSettings(table)}
}

pub fn table_reset_settings(table: &mut sys::ImGuiTable) {
	unsafe{sys::igTableResetSettings(table)}
}

pub fn table_get_bound_settings(table: &mut sys::ImGuiTable) -> *mut sys::ImGuiTableSettings {
	unsafe{sys::igTableGetBoundSettings(table)}
}

// pub fn table_settings_install_handler(context: &mut sys::ImGuiContext) {
// 	unsafe{sys::igTableSettingsInstallHandler(context)}
// }

pub fn table_settings_create(id: sys::ImGuiID, columns_count: i32) -> *mut sys::ImGuiTableSettings {
	unsafe{sys::igTableSettingsCreate(id, columns_count)}
}

pub fn table_settings_find_by_id(id: sys::ImGuiID) -> *mut sys::ImGuiTableSettings {
	unsafe{sys::igTableSettingsFindByID(id)}
}

pub fn begin_tab_bar_ex(tab_bar: &mut sys::ImGuiTabBar, bb: sys::ImRect, flags: TabBarFlags) -> bool {
	unsafe{sys::igBeginTabBarEx(tab_bar, bb, flags.bits)}
}

pub fn tab_bar_find_tab_by_id(tab_bar: &mut sys::ImGuiTabBar, tab_id: sys::ImGuiID) -> *mut sys::ImGuiTabItem {
	unsafe{sys::igTabBarFindTabByID(tab_bar, tab_id)}
}

pub fn tab_bar_remove_tab(tab_bar: &mut sys::ImGuiTabBar, tab_id: sys::ImGuiID) {
	unsafe{sys::igTabBarRemoveTab(tab_bar, tab_id)}
}

pub fn tab_bar_close_tab(tab_bar: &mut sys::ImGuiTabBar, tab: &mut sys::ImGuiTabItem) {
	unsafe{sys::igTabBarCloseTab(tab_bar, tab)}
}

pub fn tab_bar_queue_reorder(tab_bar: &mut sys::ImGuiTabBar, tab: *const sys::ImGuiTabItem, offset: i32) {
	unsafe{sys::igTabBarQueueReorder(tab_bar, tab, offset)}
}

pub fn tab_bar_queue_reorder_from_mouse_pos(tab_bar: &mut sys::ImGuiTabBar, tab: *const sys::ImGuiTabItem, mouse_pos: [f32; 2]) {
	unsafe{sys::igTabBarQueueReorderFromMousePos(tab_bar, tab, mouse_pos)}
}

pub fn tab_bar_process_reorder(tab_bar: &mut sys::ImGuiTabBar) -> bool {
	unsafe{sys::igTabBarProcessReorder(tab_bar)}
}

pub fn tab_item_ex(tab_bar: &mut sys::ImGuiTabBar, label: &str, p_open: &mut bool, flags: TabItemFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTabItemEx(tab_bar, label_.as_ptr(), p_open, flags.bits)}
}

pub fn tab_item_calc_size(pOut: &mut [f32; 2], label: &str, has_close_button: bool) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTabItemCalcSize(pOut, label_.as_ptr(), has_close_button)}
}

pub fn tab_item_background(draw_list: &mut sys::ImDrawList, bb: sys::ImRect, flags: TabItemFlags, col: u32) {
	unsafe{sys::igTabItemBackground(draw_list, bb, flags.bits, col)}
}

pub fn tab_item_label_and_close_button(draw_list: &mut sys::ImDrawList, bb: sys::ImRect, flags: TabItemFlags, frame_padding: [f32; 2], label: &str, tab_id: sys::ImGuiID, close_button_id: sys::ImGuiID, is_contents_visible: bool, out_just_closed: &mut bool, out_text_clipped: &mut bool) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTabItemLabelAndCloseButton(draw_list, bb, flags.bits, frame_padding, label_.as_ptr(), tab_id, close_button_id, is_contents_visible, out_just_closed, out_text_clipped)}
}

pub fn render_text(pos: [f32; 2], text: &str, text_end: &str, hide_text_after_hash: bool) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igRenderText(pos, text_.as_ptr(), text_end_.as_ptr(), hide_text_after_hash)}
}

pub fn render_text_wrapped(pos: [f32; 2], text: &str, text_end: &str, wrap_width: f32) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igRenderTextWrapped(pos, text_.as_ptr(), text_end_.as_ptr(), wrap_width)}
}

pub fn render_text_clipped(pos_min: [f32; 2], pos_max: [f32; 2], text: &str, text_end: &str, text_size_if_known: *const [f32; 2], align: [f32; 2], clip_rect: *const sys::ImRect) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igRenderTextClipped(pos_min, pos_max, text_.as_ptr(), text_end_.as_ptr(), text_size_if_known, align, clip_rect)}
}

pub fn render_text_clipped_ex(draw_list: &mut sys::ImDrawList, pos_min: [f32; 2], pos_max: [f32; 2], text: &str, text_end: &str, text_size_if_known: *const [f32; 2], align: [f32; 2], clip_rect: *const sys::ImRect) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igRenderTextClippedEx(draw_list, pos_min, pos_max, text_.as_ptr(), text_end_.as_ptr(), text_size_if_known, align, clip_rect)}
}

pub fn render_text_ellipsis(draw_list: &mut sys::ImDrawList, pos_min: [f32; 2], pos_max: [f32; 2], clip_max_x: f32, ellipsis_max_x: f32, text: &str, text_end: &str, text_size_if_known: *const [f32; 2]) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igRenderTextEllipsis(draw_list, pos_min, pos_max, clip_max_x, ellipsis_max_x, text_.as_ptr(), text_end_.as_ptr(), text_size_if_known)}
}

pub fn render_frame(p_min: [f32; 2], p_max: [f32; 2], fill_col: u32, border: bool, rounding: f32) {
	unsafe{sys::igRenderFrame(p_min, p_max, fill_col, border, rounding)}
}

pub fn render_frame_border(p_min: [f32; 2], p_max: [f32; 2], rounding: f32) {
	unsafe{sys::igRenderFrameBorder(p_min, p_max, rounding)}
}

pub fn render_color_rect_with_alpha_checkerboard(draw_list: &mut sys::ImDrawList, p_min: [f32; 2], p_max: [f32; 2], fill_col: u32, grid_step: f32, grid_off: [f32; 2], rounding: f32, flags: DrawFlags) {
	unsafe{sys::igRenderColorRectWithAlphaCheckerboard(draw_list, p_min, p_max, fill_col, grid_step, grid_off, rounding, flags.bits)}
}

pub fn render_nav_highlight(bb: sys::ImRect, id: sys::ImGuiID, flags: NavHighlightFlags) {
	unsafe{sys::igRenderNavHighlight(bb, id, flags.bits)}
}

pub fn find_rendered_text_end(text: &str, text_end: &str) -> *const ::std::os::raw::c_char {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igFindRenderedTextEnd(text_.as_ptr(), text_end_.as_ptr())}
}

pub fn render_arrow(draw_list: &mut sys::ImDrawList, pos: [f32; 2], col: u32, dir: Dir, scale: f32) {
	unsafe{sys::igRenderArrow(draw_list, pos, col, dir as i32, scale)}
}

pub fn render_bullet(draw_list: &mut sys::ImDrawList, pos: [f32; 2], col: u32) {
	unsafe{sys::igRenderBullet(draw_list, pos, col)}
}

pub fn render_check_mark(draw_list: &mut sys::ImDrawList, pos: [f32; 2], col: u32, sz: f32) {
	unsafe{sys::igRenderCheckMark(draw_list, pos, col, sz)}
}

// pub fn render_mouse_cursor(draw_list: &mut sys::ImDrawList, pos: [f32; 2], scale: f32, mouse_cursor: MouseCursor, col_fill: u32, col_border: u32, col_shadow: u32) {
// 	unsafe{sys::igRenderMouseCursor(draw_list, pos, scale, mouse_cursor as i32, col_fill, col_border, col_shadow)}
// }

pub fn render_arrow_pointing_at(draw_list: &mut sys::ImDrawList, pos: [f32; 2], half_sz: [f32; 2], direction: Dir, col: u32) {
	unsafe{sys::igRenderArrowPointingAt(draw_list, pos, half_sz, direction as i32, col)}
}

pub fn render_rect_filled_range_h(draw_list: &mut sys::ImDrawList, rect: sys::ImRect, col: u32, x_start_norm: f32, x_end_norm: f32, rounding: f32) {
	unsafe{sys::igRenderRectFilledRangeH(draw_list, rect, col, x_start_norm, x_end_norm, rounding)}
}

pub fn render_rect_filled_with_hole(draw_list: &mut sys::ImDrawList, outer: sys::ImRect, inner: sys::ImRect, col: u32, rounding: f32) {
	unsafe{sys::igRenderRectFilledWithHole(draw_list, outer, inner, col, rounding)}
}

pub fn text_ex(text: &str, text_end: &str, flags: TextFlags) {
	let text_ = CString::new(text).unwrap();
	let text_end_ = CString::new(text_end).unwrap();
	unsafe{sys::igTextEx(text_.as_ptr(), text_end_.as_ptr(), flags.bits)}
}

pub fn button_ex(label: &str, size_arg: [f32; 2], flags: ButtonFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igButtonEx(label_.as_ptr(), size_arg, flags.bits)}
}

pub fn close_button(id: sys::ImGuiID, pos: [f32; 2]) -> bool {
	unsafe{sys::igCloseButton(id, pos)}
}

pub fn collapse_button(id: sys::ImGuiID, pos: [f32; 2]) -> bool {
	unsafe{sys::igCollapseButton(id, pos)}
}

pub fn arrow_button_ex(str_id: &str, dir: Dir, size_arg: [f32; 2], flags: ButtonFlags) -> bool {
	let str_id_ = CString::new(str_id).unwrap();
	unsafe{sys::igArrowButtonEx(str_id_.as_ptr(), dir as i32, size_arg, flags.bits)}
}

pub fn scrollbar(axis: sys::ImGuiAxis) {
	unsafe{sys::igScrollbar(axis)}
}

// pub fn scrollbar_ex(bb: sys::ImRect, id: sys::ImGuiID, axis: sys::ImGuiAxis, p_scroll_v: &mut f32, avail_v: f32, contents_v: f32, flags: DrawFlags) -> bool {
// 	unsafe{sys::igScrollbarEx(bb, id, axis, p_scroll_v, avail_v, contents_v, flags.bits)}
// }

pub fn image_button_ex(id: sys::ImGuiID, texture_id: usize, size: [f32; 2], uv0: [f32; 2], uv1: [f32; 2], padding: [f32; 2], bg_col: [f32; 4], tint_col: [f32; 4]) -> bool {
	unsafe{sys::igImageButtonEx(id, texture_id as *mut _, size, uv0, uv1, padding, bg_col, tint_col)}
}

pub fn get_window_scrollbar_rect(pOut: &mut sys::ImRect, window: &mut sys::ImGuiWindow, axis: sys::ImGuiAxis) {
	unsafe{sys::igGetWindowScrollbarRect(pOut, window, axis)}
}

pub fn get_window_scrollbar_id(window: &mut sys::ImGuiWindow, axis: sys::ImGuiAxis) -> sys::ImGuiID {
	unsafe{sys::igGetWindowScrollbarID(window, axis)}
}

pub fn get_window_resize_corner_id(window: &mut sys::ImGuiWindow, n: i32) -> sys::ImGuiID {
	unsafe{sys::igGetWindowResizeCornerID(window, n)}
}

pub fn get_window_resize_border_id(window: &mut sys::ImGuiWindow, dir: Dir) -> sys::ImGuiID {
	unsafe{sys::igGetWindowResizeBorderID(window, dir as i32)}
}

pub fn separator_ex(flags: SeparatorFlags) {
	unsafe{sys::igSeparatorEx(flags.bits)}
}

pub fn checkbox_flags__s64_ptr(label: &str, flags: &mut sys::ImS64, flags_value: sys::ImS64) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCheckboxFlags_S64Ptr(label_.as_ptr(), flags, flags_value)}
}

pub fn checkbox_flags__u64_ptr(label: &str, flags: &mut sys::ImU64, flags_value: sys::ImU64) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igCheckboxFlags_U64Ptr(label_.as_ptr(), flags, flags_value)}
}

pub fn button_behavior(bb: sys::ImRect, id: sys::ImGuiID, out_hovered: &mut bool, out_held: &mut bool, flags: ButtonFlags) -> bool {
	unsafe{sys::igButtonBehavior(bb, id, out_hovered, out_held, flags.bits)}
}

pub fn drag_behavior(id: sys::ImGuiID, data_type: DataType, p_v: &mut ::std::os::raw::c_void, v_speed: f32, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags) -> bool {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDragBehavior(id, data_type as i32, p_v, v_speed, p_min, p_max, format_.as_ptr(), flags.bits)}
}

pub fn slider_behavior(bb: sys::ImRect, id: sys::ImGuiID, data_type: DataType, p_v: &mut ::std::os::raw::c_void, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void, format: &str, flags: SliderFlags, out_grab_bb: &mut sys::ImRect) -> bool {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igSliderBehavior(bb, id, data_type as i32, p_v, p_min, p_max, format_.as_ptr(), flags.bits, out_grab_bb)}
}

pub fn splitter_behavior(bb: sys::ImRect, id: sys::ImGuiID, axis: sys::ImGuiAxis, size1: &mut f32, size2: &mut f32, min_size1: f32, min_size2: f32, hover_extend: f32, hover_visibility_delay: f32) -> bool {
	unsafe{sys::igSplitterBehavior(bb, id, axis, size1, size2, min_size1, min_size2, hover_extend, hover_visibility_delay)}
}

pub fn tree_node_behavior(id: sys::ImGuiID, flags: TreeNodeFlags, label: &str, label_end: &str) -> bool {
	let label_ = CString::new(label).unwrap();
	let label_end_ = CString::new(label_end).unwrap();
	unsafe{sys::igTreeNodeBehavior(id, flags.bits, label_.as_ptr(), label_end_.as_ptr())}
}

pub fn tree_node_behavior_is_open(id: sys::ImGuiID, flags: TreeNodeFlags) -> bool {
	unsafe{sys::igTreeNodeBehaviorIsOpen(id, flags.bits)}
}

pub fn tree_push_override_id(id: sys::ImGuiID) {
	unsafe{sys::igTreePushOverrideID(id)}
}

pub fn data_type_get_info(data_type: DataType) -> *const sys::ImGuiDataTypeInfo {
	unsafe{sys::igDataTypeGetInfo(data_type as i32)}
}

pub fn data_type_format_string(buf: &mut ::std::os::raw::c_char, buf_size: i32, data_type: DataType, p_data: *const ::std::os::raw::c_void, format: &str) -> i32 {
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igDataTypeFormatString(buf, buf_size, data_type as i32, p_data, format_.as_ptr())}
}

pub fn data_type_apply_op(data_type: DataType, op: i32, output: &mut ::std::os::raw::c_void, arg_1: *const ::std::os::raw::c_void, arg_2: *const ::std::os::raw::c_void) {
	unsafe{sys::igDataTypeApplyOp(data_type as i32, op, output, arg_1, arg_2)}
}

// pub fn data_type_apply_op_from_text(buf: &str, initial_value_buf: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, format: &str) -> bool {
// 	let buf_ = CString::new(buf).unwrap();
// 	let initial_value_buf_ = CString::new(initial_value_buf).unwrap();
// 	let format_ = CString::new(format).unwrap();
// 	unsafe{sys::igDataTypeApplyOpFromText(buf_.as_ptr(), initial_value_buf_.as_ptr(), data_type as i32, p_data, format_.as_ptr())}
// }

pub fn data_type_compare(data_type: DataType, arg_1: *const ::std::os::raw::c_void, arg_2: *const ::std::os::raw::c_void) -> i32 {
	unsafe{sys::igDataTypeCompare(data_type as i32, arg_1, arg_2)}
}

pub fn data_type_clamp(data_type: DataType, p_data: &mut ::std::os::raw::c_void, p_min: *const ::std::os::raw::c_void, p_max: *const ::std::os::raw::c_void) -> bool {
	unsafe{sys::igDataTypeClamp(data_type as i32, p_data, p_min, p_max)}
}

pub fn input_text_ex(label: &str, hint: &str, buf: &mut ::std::os::raw::c_char, buf_size: i32, size_arg: [f32; 2], flags: InputTextFlags, callback: sys::ImGuiInputTextCallback, user_data: &mut ::std::os::raw::c_void) -> bool {
	let label_ = CString::new(label).unwrap();
	let hint_ = CString::new(hint).unwrap();
	unsafe{sys::igInputTextEx(label_.as_ptr(), hint_.as_ptr(), buf, buf_size, size_arg, flags.bits, callback, user_data)}
}

pub fn temp_input_text(bb: sys::ImRect, id: sys::ImGuiID, label: &str, buf: &mut ::std::os::raw::c_char, buf_size: i32, flags: InputTextFlags) -> bool {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igTempInputText(bb, id, label_.as_ptr(), buf, buf_size, flags.bits)}
}

pub fn temp_input_scalar(bb: sys::ImRect, id: sys::ImGuiID, label: &str, data_type: DataType, p_data: &mut ::std::os::raw::c_void, format: &str, p_clamp_min: *const ::std::os::raw::c_void, p_clamp_max: *const ::std::os::raw::c_void) -> bool {
	let label_ = CString::new(label).unwrap();
	let format_ = CString::new(format).unwrap();
	unsafe{sys::igTempInputScalar(bb, id, label_.as_ptr(), data_type as i32, p_data, format_.as_ptr(), p_clamp_min, p_clamp_max)}
}

pub fn temp_input_is_active(id: sys::ImGuiID) -> bool {
	unsafe{sys::igTempInputIsActive(id)}
}

pub fn get_input_text_state(id: sys::ImGuiID) -> *mut sys::ImGuiInputTextState {
	unsafe{sys::igGetInputTextState(id)}
}

pub fn color_tooltip(text: &str, col: *const f32, flags: ColorEditFlags) {
	let text_ = CString::new(text).unwrap();
	unsafe{sys::igColorTooltip(text_.as_ptr(), col, flags.bits)}
}

pub fn color_edit_options_popup(col: *const f32, flags: ColorEditFlags) {
	unsafe{sys::igColorEditOptionsPopup(col, flags.bits)}
}

pub fn color_picker_options_popup(ref_col: *const f32, flags: ColorEditFlags) {
	unsafe{sys::igColorPickerOptionsPopup(ref_col, flags.bits)}
}

pub fn shade_verts_linear_color_gradient_keep_alpha(draw_list: &mut sys::ImDrawList, vert_start_idx: i32, vert_end_idx: i32, gradient_p0: [f32; 2], gradient_p1: [f32; 2], col0: u32, col1: u32) {
	unsafe{sys::igShadeVertsLinearColorGradientKeepAlpha(draw_list, vert_start_idx, vert_end_idx, gradient_p0, gradient_p1, col0, col1)}
}

pub fn shade_verts_linear_uv(draw_list: &mut sys::ImDrawList, vert_start_idx: i32, vert_end_idx: i32, a: [f32; 2], b: [f32; 2], uv_a: [f32; 2], uv_b: [f32; 2], clamp: bool) {
	unsafe{sys::igShadeVertsLinearUV(draw_list, vert_start_idx, vert_end_idx, a, b, uv_a, uv_b, clamp)}
}

pub fn gc_compact_transient_misc_buffers() {
	unsafe{sys::igGcCompactTransientMiscBuffers()}
}

pub fn gc_compact_transient_window_buffers(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igGcCompactTransientWindowBuffers(window)}
}

pub fn gc_awake_transient_window_buffers(window: &mut sys::ImGuiWindow) {
	unsafe{sys::igGcAwakeTransientWindowBuffers(window)}
}

pub fn error_check_end_frame_recover(log_callback: sys::ImGuiErrorLogCallback, user_data: &mut ::std::os::raw::c_void) {
	unsafe{sys::igErrorCheckEndFrameRecover(log_callback, user_data)}
}

pub fn debug_draw_item_rect(col: u32) {
	unsafe{sys::igDebugDrawItemRect(col)}
}

pub fn debug_start_item_picker() {
	unsafe{sys::igDebugStartItemPicker()}
}

pub fn show_font_atlas(atlas: &mut sys::ImFontAtlas) {
	unsafe{sys::igShowFontAtlas(atlas)}
}

pub fn debug_node_columns(columns: &mut sys::ImGuiOldColumns) {
	unsafe{sys::igDebugNodeColumns(columns)}
}

pub fn debug_node_draw_list(window: &mut sys::ImGuiWindow, draw_list: *const sys::ImDrawList, label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igDebugNodeDrawList(window, draw_list, label_.as_ptr())}
}

pub fn debug_node_draw_cmd_show_mesh_and_bounding_box(out_draw_list: &mut sys::ImDrawList, draw_list: *const sys::ImDrawList, draw_cmd: *const sys::ImDrawCmd, show_mesh: bool, show_aabb: bool) {
	unsafe{sys::igDebugNodeDrawCmdShowMeshAndBoundingBox(out_draw_list, draw_list, draw_cmd, show_mesh, show_aabb)}
}

pub fn debug_node_font(font: &mut sys::ImFont) {
	unsafe{sys::igDebugNodeFont(font)}
}

pub fn debug_node_storage(storage: &mut sys::ImGuiStorage, label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igDebugNodeStorage(storage, label_.as_ptr())}
}

pub fn debug_node_tab_bar(tab_bar: &mut sys::ImGuiTabBar, label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igDebugNodeTabBar(tab_bar, label_.as_ptr())}
}

pub fn debug_node_table(table: &mut sys::ImGuiTable) {
	unsafe{sys::igDebugNodeTable(table)}
}

pub fn debug_node_table_settings(settings: &mut sys::ImGuiTableSettings) {
	unsafe{sys::igDebugNodeTableSettings(settings)}
}

pub fn debug_node_window(window: &mut sys::ImGuiWindow, label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igDebugNodeWindow(window, label_.as_ptr())}
}

pub fn debug_node_window_settings(settings: &mut sys::ImGuiWindowSettings) {
	unsafe{sys::igDebugNodeWindowSettings(settings)}
}

pub fn debug_node_windows_list(windows: &mut sys::ImVector_ImGuiWindowPtr, label: &str) {
	let label_ = CString::new(label).unwrap();
	unsafe{sys::igDebugNodeWindowsList(windows, label_.as_ptr())}
}

pub fn debug_node_viewport(viewport: &mut sys::ImGuiViewportP) {
	unsafe{sys::igDebugNodeViewport(viewport)}
}

pub fn debug_render_viewport_thumbnail(draw_list: &mut sys::ImDrawList, viewport: &mut sys::ImGuiViewportP, bb: sys::ImRect) {
	unsafe{sys::igDebugRenderViewportThumbnail(draw_list, viewport, bb)}
}

pub fn im_font_atlas_get_builder_for_stb_truetype() -> *const sys::ImFontBuilderIO {
	unsafe{sys::igImFontAtlasGetBuilderForStbTruetype()}
}

pub fn im_font_atlas_build_init(atlas: &mut sys::ImFontAtlas) {
	unsafe{sys::igImFontAtlasBuildInit(atlas)}
}

pub fn im_font_atlas_build_setup_font(atlas: &mut sys::ImFontAtlas, font: &mut sys::ImFont, font_config: &mut sys::ImFontConfig, ascent: f32, descent: f32) {
	unsafe{sys::igImFontAtlasBuildSetupFont(atlas, font, font_config, ascent, descent)}
}

pub fn im_font_atlas_build_pack_custom_rects(atlas: &mut sys::ImFontAtlas, stbrp_context_opaque: &mut ::std::os::raw::c_void) {
	unsafe{sys::igImFontAtlasBuildPackCustomRects(atlas, stbrp_context_opaque)}
}

pub fn im_font_atlas_build_finish(atlas: &mut sys::ImFontAtlas) {
	unsafe{sys::igImFontAtlasBuildFinish(atlas)}
}

pub fn im_font_atlas_build_render8bpp_rect_from_string(atlas: &mut sys::ImFontAtlas, x: i32, y: i32, w: i32, h: i32, in_str: &str, in_marker_char: ::std::os::raw::c_char, in_marker_pixel_value: ::std::os::raw::c_uchar) {
	let in_str_ = CString::new(in_str).unwrap();
	unsafe{sys::igImFontAtlasBuildRender8bppRectFromString(atlas, x, y, w, h, in_str_.as_ptr(), in_marker_char, in_marker_pixel_value)}
}

pub fn im_font_atlas_build_render32bpp_rect_from_string(atlas: &mut sys::ImFontAtlas, x: i32, y: i32, w: i32, h: i32, in_str: &str, in_marker_char: ::std::os::raw::c_char, in_marker_pixel_value: ::std::os::raw::c_uint) {
	let in_str_ = CString::new(in_str).unwrap();
	unsafe{sys::igImFontAtlasBuildRender32bppRectFromString(atlas, x, y, w, h, in_str_.as_ptr(), in_marker_char, in_marker_pixel_value)}
}

pub fn im_font_atlas_build_multiply_calc_lookup_table(out_table: &mut ::std::os::raw::c_uchar, in_multiply_factor: f32) {
	unsafe{sys::igImFontAtlasBuildMultiplyCalcLookupTable(out_table, in_multiply_factor)}
}

pub fn im_font_atlas_build_multiply_rect_alpha8(table: *const ::std::os::raw::c_uchar, pixels: &mut ::std::os::raw::c_uchar, x: i32, y: i32, w: i32, h: i32, stride: i32) {
	unsafe{sys::igImFontAtlasBuildMultiplyRectAlpha8(table, pixels, x, y, w, h, stride)}
}

pub fn log_text(fmt: &str) {
	let fmt_ = CString::new(fmt).unwrap();
	unsafe{sys::igLogText(fmt_.as_ptr())}
}

pub fn g_et__flt__max() -> f32 {
	unsafe{sys::igGET_FLT_MAX()}
}

pub fn g_et__flt__min() -> f32 {
	unsafe{sys::igGET_FLT_MIN()}
}
