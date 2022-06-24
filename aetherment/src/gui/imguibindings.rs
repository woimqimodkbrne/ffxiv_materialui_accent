#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[path = "bindings.rs"]
mod sys;

#[repr(i32)]
pub enum ImGuiWindowFlags {
	None = 0,
	NoTitleBar = 1,
	NoResize = 2,
	NoMove = 4,
	NoScrollbar = 8,
	NoScrollWithMouse = 16,
	NoCollapse = 32,
	AlwaysAutoResize = 64,
	NoBackground = 128,
	NoSavedSettings = 256,
	NoMouseInputs = 512,
	MenuBar = 1024,
	HorizontalScrollbar = 2048,
	NoFocusOnAppearing = 4096,
	NoBringToFrontOnFocus = 8192,
	AlwaysVerticalScrollbar = 16384,
	AlwaysHorizontalScrollbar = 32768,
	AlwaysUseWindowPadding = 65536,
	NoNavInputs = 262144,
	NoNavFocus = 524288,
	UnsavedDocument = 1048576,
	NoNav = 786432,
	NoDecoration = 43,
	NoInputs = 786944,
	NavFlattened = 8388608,
	ChildWindow = 16777216,
	Tooltip = 33554432,
	Popup = 67108864,
	Modal = 134217728,
	ChildMenu = 268435456,
}

#[repr(i32)]
pub enum ImGuiInputTextFlags {
	None = 0,
	CharsDecimal = 1,
	CharsHexadecimal = 2,
	CharsUppercase = 4,
	CharsNoBlank = 8,
	AutoSelectAll = 16,
	EnterReturnsTrue = 32,
	CallbackCompletion = 64,
	CallbackHistory = 128,
	CallbackAlways = 256,
	CallbackCharFilter = 512,
	AllowTabInput = 1024,
	CtrlEnterForNewLine = 2048,
	NoHorizontalScroll = 4096,
	AlwaysOverwrite = 8192,
	ReadOnly = 16384,
	Password = 32768,
	NoUndoRedo = 65536,
	CharsScientific = 131072,
	CallbackResize = 262144,
	CallbackEdit = 524288,
}

#[repr(i32)]
pub enum ImGuiTreeNodeFlags {
	None = 0,
	Selected = 1,
	Framed = 2,
	AllowItemOverlap = 4,
	NoTreePushOnOpen = 8,
	NoAutoOpenOnLog = 16,
	DefaultOpen = 32,
	OpenOnDoubleClick = 64,
	OpenOnArrow = 128,
	Leaf = 256,
	Bullet = 512,
	FramePadding = 1024,
	SpanAvailWidth = 2048,
	SpanFullWidth = 4096,
	NavLeftJumpsBackHere = 8192,
	CollapsingHeader = 26,
}

#[repr(i32)]
pub enum ImGuiPopupFlags {
	None = 0,
	MouseButtonLeft = 0,
	MouseButtonRight = 1,
	MouseButtonMiddle = 2,
	NoOpenOverExistingPopup = 32,
	NoOpenOverItems = 64,
	AnyPopupId = 128,
	AnyPopupLevel = 256,
	AnyPopup = 384,
}

#[repr(i32)]
pub enum ImGuiSelectableFlags {
	None = 0,
	DontClosePopups = 1,
	SpanAllColumns = 2,
	AllowDoubleClick = 4,
	Disabled = 8,
	AllowItemOverlap = 16,
}

#[repr(i32)]
pub enum ImGuiComboFlags {
	None = 0,
	PopupAlignLeft = 1,
	HeightSmall = 2,
	HeightRegular = 4,
	HeightLarge = 8,
	HeightLargest = 16,
	NoArrowButton = 32,
	NoPreview = 64,
}

#[repr(i32)]
pub enum ImGuiTabBarFlags {
	None = 0,
	Reorderable = 1,
	AutoSelectNewTabs = 2,
	TabListPopupButton = 4,
	NoCloseWithMiddleMouseButton = 8,
	NoTabListScrollingButtons = 16,
	NoTooltip = 32,
	FittingPolicyResizeDown = 64,
	FittingPolicyScroll = 128,
}

#[repr(i32)]
pub enum ImGuiTabItemFlags {
	None = 0,
	UnsavedDocument = 1,
	SetSelected = 2,
	NoCloseWithMiddleMouseButton = 4,
	NoPushId = 8,
	NoTooltip = 16,
	NoReorder = 32,
	Leading = 64,
	Trailing = 128,
}

#[repr(i32)]
pub enum ImGuiTableFlags {
	None = 0,
	Resizable = 1,
	Reorderable = 2,
	Hideable = 4,
	Sortable = 8,
	NoSavedSettings = 16,
	ContextMenuInBody = 32,
	RowBg = 64,
	BordersInnerH = 128,
	BordersOuterH = 256,
	BordersInnerV = 512,
	BordersOuterV = 1024,
	BordersH = 384,
	BordersV = 1536,
	BordersInner = 640,
	BordersOuter = 1280,
	Borders = 1920,
	NoBordersInBody = 2048,
	NoBordersInBodyUntilResize = 4096,
	SizingFixedFit = 8192,
	SizingFixedSame = 16384,
	SizingStretchProp = 24576,
	SizingStretchSame = 32768,
	NoHostExtendX = 65536,
	NoHostExtendY = 131072,
	NoKeepColumnsVisible = 262144,
	PreciseWidths = 524288,
	NoClip = 1048576,
	PadOuterX = 2097152,
	NoPadOuterX = 4194304,
	NoPadInnerX = 8388608,
	ScrollX = 16777216,
	ScrollY = 33554432,
	SortMulti = 67108864,
	SortTristate = 134217728,
}

#[repr(i32)]
pub enum ImGuiTableColumnFlags {
	None = 0,
	Disabled = 1,
	DefaultHide = 2,
	DefaultSort = 4,
	WidthStretch = 8,
	WidthFixed = 16,
	NoResize = 32,
	NoReorder = 64,
	NoHide = 128,
	NoClip = 256,
	NoSort = 512,
	NoSortAscending = 1024,
	NoSortDescending = 2048,
	NoHeaderLabel = 4096,
	NoHeaderWidth = 8192,
	PreferSortAscending = 16384,
	PreferSortDescending = 32768,
	IndentEnable = 65536,
	IndentDisable = 131072,
	IsEnabled = 16777216,
	IsVisible = 33554432,
	IsSorted = 67108864,
	IsHovered = 134217728,
}

#[repr(i32)]
pub enum ImGuiTableRowFlags {
	None = 0,
	Headers = 1,
}

#[repr(i32)]
pub enum ImGuiTableBgTarget {
	None = 0,
	CellBg = 3,
}

#[repr(i32)]
pub enum ImGuiFocusedFlags {
	None = 0,
	ChildWindows = 1,
	RootWindow = 2,
	AnyWindow = 4,
	RootAndChildWindows = 3,
}

#[repr(i32)]
pub enum ImGuiHoveredFlags {
	None = 0,
	ChildWindows = 1,
	RootWindow = 2,
	AnyWindow = 4,
	AllowWhenBlockedByPopup = 8,
	AllowWhenBlockedByActiveItem = 32,
	AllowWhenOverlapped = 64,
	AllowWhenDisabled = 128,
	RectOnly = 104,
	RootAndChildWindows = 3,
}

#[repr(i32)]
pub enum ImGuiDragDropFlags {
	None = 0,
	SourceNoPreviewTooltip = 1,
	SourceNoDisableHover = 2,
	SourceNoHoldToOpenOthers = 4,
	SourceAllowNullID = 8,
	SourceExtern = 16,
	SourceAutoExpirePayload = 32,
	AcceptBeforeDelivery = 1024,
	AcceptNoDrawDefaultRect = 2048,
	AcceptNoPreviewTooltip = 4096,
	AcceptPeekOnly = 3072,
}

#[repr(i32)]
pub enum ImGuiDataType {
	Float = 8,
	Double = 9,
	COUNT = 10,
}

#[repr(i32)]
pub enum ImGuiDir {
	Left = 0,
	Right = 1,
	Up = 2,
	Down = 3,
	COUNT = 4,
}

#[repr(i32)]
pub enum ImGuiSortDirection {
	None = 0,
	Ascending = 1,
	Descending = 2,
}

#[repr(i32)]
pub enum ImGuiKey {
	Tab = 0,
	LeftArrow = 1,
	RightArrow = 2,
	UpArrow = 3,
	DownArrow = 4,
	PageUp = 5,
	PageDown = 6,
	Home = 7,
	End = 8,
	Insert = 9,
	Delete = 10,
	Backspace = 11,
	Space = 12,
	Enter = 13,
	Escape = 14,
	KeyPadEnter = 15,
	A = 16,
	C = 17,
	V = 18,
	X = 19,
	Y = 20,
	Z = 21,
	COUNT = 22,
}

#[repr(i32)]
pub enum ImGuiKeyModFlags {
	None = 0,
	Ctrl = 1,
	Shift = 2,
	Alt = 4,
	Super = 8,
}

#[repr(i32)]
pub enum ImGuiNavInput {
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
pub enum ImGuiConfigFlags {
	None = 0,
	NavEnableKeyboard = 1,
	NavEnableGamepad = 2,
	NavEnableSetMousePos = 4,
	NavNoCaptureKeyboard = 8,
	NoMouse = 16,
	NoMouseCursorChange = 32,
	IsSRGB = 1048576,
	IsTouchScreen = 2097152,
}

#[repr(i32)]
pub enum ImGuiBackendFlags {
	None = 0,
	HasGamepad = 1,
	HasMouseCursors = 2,
	HasSetMousePos = 4,
	RendererHasVtxOffset = 8,
}

#[repr(i32)]
pub enum ImGuiCol {
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
pub enum ImGuiStyleVar {
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
pub enum ImGuiButtonFlags {
	None = 0,
	MouseButtonLeft = 1,
	MouseButtonRight = 2,
	MouseButtonMiddle = 4,
}

#[repr(i32)]
pub enum ImGuiColorEditFlags {
	None = 0,
	NoAlpha = 2,
	NoPicker = 4,
	NoOptions = 8,
	NoSmallPreview = 16,
	NoInputs = 32,
	NoTooltip = 64,
	NoLabel = 128,
	NoSidePreview = 256,
	NoDragDrop = 512,
	NoBorder = 1024,
	AlphaBar = 65536,
	AlphaPreview = 131072,
	AlphaPreviewHalf = 262144,
	HDR = 524288,
	DisplayRGB = 1048576,
	DisplayHSV = 2097152,
	DisplayHex = 4194304,
	Float = 16777216,
	PickerHueBar = 33554432,
	PickerHueWheel = 67108864,
	InputRGB = 134217728,
	InputHSV = 268435456,
}

#[repr(i32)]
pub enum ImGuiSliderFlags {
	None = 0,
	AlwaysClamp = 16,
	Logarithmic = 32,
	NoRoundToFormat = 64,
	NoInput = 128,
}

#[repr(i32)]
pub enum ImGuiMouseButton {
	Left = 0,
	Right = 1,
	Middle = 2,
	COUNT = 5,
}

#[repr(i32)]
pub enum ImGuiMouseCursor {
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
pub enum ImGuiCond {
	None = 0,
	Always = 1,
	Once = 2,
	FirstUseEver = 4,
	Appearing = 8,
}

#[repr(i32)]
pub enum ImDrawFlags {
	None = 0,
	Closed = 1,
	RoundCornersTopLeft = 16,
	RoundCornersTopRight = 32,
	RoundCornersBottomLeft = 64,
	RoundCornersBottomRight = 128,
	RoundCornersNone = 256,
	RoundCornersTop = 48,
	RoundCornersBottom = 192,
	RoundCornersLeft = 80,
	RoundCornersRight = 160,
	RoundCornersAll = 240,
}

#[repr(i32)]
pub enum ImDrawListFlags {
	None = 0,
	AntiAliasedLines = 1,
	AntiAliasedLinesUseTex = 2,
	AntiAliasedFill = 4,
	AllowVtxOffset = 8,
}

#[repr(i32)]
pub enum ImFontAtlasFlags {
	None = 0,
	NoPowerOfTwoHeight = 1,
	NoMouseCursors = 2,
	NoBakedLines = 4,
}

#[repr(i32)]
pub enum ImGuiViewportFlags {
	None = 0,
	IsPlatformWindow = 1,
	IsPlatformMonitor = 2,
	OwnedByApp = 4,
}

#[repr(i32)]
pub enum ImGuiItemFlags {
	None = 0,
	NoTabStop = 1,
	ButtonRepeat = 2,
	Disabled = 4,
	NoNav = 8,
	NoNavDefaultFocus = 16,
	SelectableDontClosePopup = 32,
	MixedValue = 64,
	ReadOnly = 128,
}

#[repr(i32)]
pub enum ImGuiItemAddFlags {
	None = 0,
	Focusable = 1,
}

#[repr(i32)]
pub enum ImGuiItemStatusFlags {
	None = 0,
	HoveredRect = 1,
	HasDisplayRect = 2,
	Edited = 4,
	ToggledSelection = 8,
	ToggledOpen = 16,
	HasDeactivated = 32,
	Deactivated = 64,
	HoveredWindow = 128,
	FocusedByCode = 256,
	FocusedByTabbing = 512,
	Focused = 768,
}

#[repr(i32)]
pub enum ImGuiInputTextFlagsPrivate {
	Multiline = 67108864,
	NoMarkEdited = 134217728,
	MergedItem = 268435456,
}

#[repr(i32)]
pub enum ImGuiButtonFlagsPrivate {
	PressedOnClick = 16,
	PressedOnClickRelease = 32,
	PressedOnClickReleaseAnywhere = 64,
	PressedOnRelease = 128,
	PressedOnDoubleClick = 256,
	PressedOnDragDropHold = 512,
	Repeat = 1024,
	FlattenChildren = 2048,
	AllowItemOverlap = 4096,
	DontClosePopups = 8192,
	AlignTextBaseLine = 32768,
	NoKeyModifiers = 65536,
	NoHoldingActiveId = 131072,
	NoNavFocus = 262144,
	NoHoveredOnFocus = 524288,
}

#[repr(i32)]
pub enum ImGuiComboFlagsPrivate {
	CustomPreview = 1048576,
}

#[repr(i32)]
pub enum ImGuiSliderFlagsPrivate {
	Vertical = 1048576,
	ReadOnly = 2097152,
}

#[repr(i32)]
pub enum ImGuiSelectableFlagsPrivate {
	NoHoldingActiveID = 1048576,
	SelectOnNav = 2097152,
	SelectOnClick = 4194304,
	SelectOnRelease = 8388608,
	SpanAvailWidth = 16777216,
	DrawHoveredWhenHeld = 33554432,
	SetNavIdOnHover = 67108864,
	NoPadWithHalfSpacing = 134217728,
}

#[repr(i32)]
pub enum ImGuiTreeNodeFlagsPrivate {
	ClipLabelForTrailingButton = 1048576,
}

#[repr(i32)]
pub enum ImGuiSeparatorFlags {
	None = 0,
	Horizontal = 1,
	Vertical = 2,
	SpanAllColumns = 4,
}

#[repr(i32)]
pub enum ImGuiTextFlags {
	None = 0,
	NoWidthForLargeClippedText = 1,
}

#[repr(i32)]
pub enum ImGuiTooltipFlags {
	None = 0,
	OverridePreviousTooltip = 1,
}

#[repr(i32)]
pub enum ImGuiLayoutType {
	Horizontal = 0,
	Vertical = 1,
}

#[repr(i32)]
pub enum ImGuiLogTyp {
	None = 0,
	TTY = 1,
	File = 2,
	Buffer = 3,
	Clipboard = 4,
}

#[repr(i32)]
pub enum ImGuiAxi {
	X = 0,
	Y = 1,
}

#[repr(i32)]
pub enum ImGuiPlotTyp {
	Lines = 0,
	Histogram = 1,
}

#[repr(i32)]
pub enum ImGuiInputSourc {
	None = 0,
	Mouse = 1,
	Keyboard = 2,
	Gamepad = 3,
	Nav = 4,
	Clipboard = 5,
	COUNT = 6,
}

#[repr(i32)]
pub enum ImGuiInputReadMod {
	Down = 0,
	Pressed = 1,
	Released = 2,
	Repeat = 3,
	RepeatSlow = 4,
	RepeatFast = 5,
}

#[repr(i32)]
pub enum ImGuiNavHighlightFlags {
	None = 0,
	TypeDefault = 1,
	TypeThin = 2,
	AlwaysDraw = 4,
	NoRounding = 8,
}

#[repr(i32)]
pub enum ImGuiNavDirSourceFlags {
	None = 0,
	Keyboard = 1,
	PadDPad = 2,
	PadLStick = 4,
}

#[repr(i32)]
pub enum ImGuiNavMoveFlags {
	None = 0,
	LoopX = 1,
	LoopY = 2,
	WrapX = 4,
	WrapY = 8,
	AllowCurrentNavId = 16,
	AlsoScoreVisibleSet = 32,
	ScrollToEdge = 64,
}

#[repr(i32)]
pub enum ImGuiNavForwar {
	None = 0,
	ForwardQueued = 1,
	ForwardActive = 2,
}

#[repr(i32)]
pub enum ImGuiNavLaye {
	Main = 0,
	Menu = 1,
	COUNT = 2,
}

#[repr(i32)]
pub enum ImGuiPopupPositionPolic {
	Default = 0,
	ComboBox = 1,
	Tooltip = 2,
}

#[repr(i32)]
pub enum ImGuiDataTypePrivate {
	String = 11,
	Pointer = 12,
	ID = 13,
}

#[repr(i32)]
pub enum ImGuiNextWindowDataFlags {
	None = 0,
	HasPos = 1,
	HasSize = 2,
	HasContentSize = 4,
	HasCollapsed = 8,
	HasSizeConstraint = 16,
	HasFocus = 32,
	HasBgAlpha = 64,
	HasScroll = 128,
}

#[repr(i32)]
pub enum ImGuiNextItemDataFlags {
	None = 0,
	HasWidth = 1,
	HasOpen = 2,
}

#[repr(i32)]
pub enum ImGuiOldColumnFlags {
	None = 0,
	NoBorder = 1,
	NoResize = 2,
	NoPreserveWidths = 4,
	NoForceWithinWindow = 8,
	GrowParentContentsSize = 16,
}

#[repr(i32)]
pub enum ImGuiContextHookTyp {
	NewFramePre = 0,
	NewFramePost = 1,
	EndFramePre = 2,
	EndFramePost = 3,
	RenderPre = 4,
	RenderPost = 5,
	Shutdown = 6,
}

#[repr(i32)]
pub enum ImGuiTabBarFlagsPrivate {
	DockNode = 1048576,
	IsFocused = 2097152,
	SaveSettings = 4194304,
}

#[repr(i32)]
pub enum ImGuiTabItemFlagsPrivate {
	NoCloseButton = 1048576,
	Button = 2097152,
}
