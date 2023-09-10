use noumenon::format::game::uld;
use crate::render_helper::EnumTools;

// uld.rs
impl EnumTools for uld::AlignmentType {
	type Iterator = std::array::IntoIter<Self, 9>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::TopLeft => "Top Left",
			Self::Top => "Top",
			Self::TopRight => "Top Right",
			Self::Left => "Left",
			Self::Center => "Center",
			Self::Right => "Right",
			Self::BottomLeft => "Bottom Left",
			Self::Bottom => "Bottom",
			Self::BottomRight => "Bottom Right",
			Self::Unk(_) => "Unknown",
		}
	}
	
	fn to_string(&self) -> String {
		match self {
			Self::Unk(v) => format!("Unknown ({v})"),
			_ => self.to_str().to_owned(),
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::TopLeft,
			Self::Top,
			Self::TopRight,
			Self::Left,
			Self::Center,
			Self::Right,
			Self::BottomLeft,
			Self::Bottom,
			Self::BottomRight,
		].into_iter()
	}
}

// component.rs
impl EnumTools for uld::ComponentType {
	type Iterator = std::array::IntoIter<Self, 25>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Custom => "Custom",
			Self::Button => "Button",
			Self::Window => "Window",
			Self::CheckBox => "CheckBox",
			Self::RadioButton => "RadioButton",
			Self::Gauge => "Gauge",
			Self::Slider => "Slider",
			Self::TextInput => "TextInput",
			Self::NumericInput => "NumericInput",
			Self::List => "List",
			Self::DropDown => "DropDown",
			Self::Tabbed => "Tabbed",
			Self::TreeList => "TreeList",
			Self::ScrollBar => "ScrollBar",
			Self::ListItem => "ListItem",
			Self::Icon => "Icon",
			Self::IconWithText => "IconWithText",
			Self::DragDrop => "DragDrop",
			Self::LeveCard => "LeveCard",
			Self::NineGridText => "NineGridText",
			Self::Journal => "Journal",
			Self::Multipurpose => "Multipurpose",
			Self::Map => "Map",
			Self::Preview => "Preview",
			Self::Unknown25 => "Unknown25", 
		}
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::Custom,
			Self::Button,
			Self::Window,
			Self::CheckBox,
			Self::RadioButton,
			Self::Gauge,
			Self::Slider,
			Self::TextInput,
			Self::NumericInput,
			Self::List,
			Self::DropDown,
			Self::Tabbed,
			Self::TreeList,
			Self::ScrollBar,
			Self::ListItem,
			Self::Icon,
			Self::IconWithText,
			Self::DragDrop,
			Self::LeveCard,
			Self::NineGridText,
			Self::Journal,
			Self::Multipurpose,
			Self::Map,
			Self::Preview,
			Self::Unknown25,
		].into_iter()
	}
}

impl EnumTools for uld::Component {
	type Iterator = std::array::IntoIter<Self, 25>;
	
	fn to_str(&self) -> &'static str {
		self.get_type().to_str()
	}
	
	fn iter() -> Self::Iterator {
		[
			Self::Custom(Default::default()),
			Self::Button(Default::default()),
			Self::Window(Default::default()),
			Self::CheckBox(Default::default()),
			Self::RadioButton(Default::default()),
			Self::Gauge(Default::default()),
			Self::Slider(Default::default()),
			Self::TextInput(Default::default()),
			Self::NumericInput(Default::default()),
			Self::List(Default::default()),
			Self::DropDown(Default::default()),
			Self::Tabbed(Default::default()),
			Self::TreeList(Default::default()),
			Self::ScrollBar(Default::default()),
			Self::ListItem(Default::default()),
			Self::Icon(Default::default()),
			Self::IconWithText(Default::default()),
			Self::DragDrop(Default::default()),
			Self::LeveCard(Default::default()),
			Self::NineGridText(Default::default()),
			Self::Journal(Default::default()),
			Self::Multipurpose(Default::default()),
			Self::Map(Default::default()),
			Self::Preview(Default::default()),
			Self::Unknown25(Default::default()),
		].into_iter()
	}
}

// node.rs
impl EnumTools for uld::FontType {
	type Iterator = std::array::IntoIter<Self, 6>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Axis => "Axis",
			Self::MiedingerMed => "MiedingerMed",
			Self::Miedinger => "Miedinger",
			Self::TrumpGothic => "TrumpGothic",
			Self::Jupiter => "Jupiter",
			Self::JupiterLarge => "JupiterLarge",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Axis, Self::MiedingerMed, Self::Miedinger, Self::TrumpGothic, Self::Jupiter, Self::JupiterLarge].into_iter()
	}
}

impl EnumTools for uld::CollisionType {
	type Iterator = std::array::IntoIter<Self, 3>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Hit => "Hit",
			Self::Focus => "Focus",
			Self::Move => "Move",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Hit, Self::Focus, Self::Move].into_iter()
	}
}

impl EnumTools for uld::GridPartsType {
	type Iterator = std::array::IntoIter<Self, 2>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Divide => "Divide",
			Self::Compose => "Compose",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Divide, Self::Compose].into_iter()
	}
}

impl EnumTools for uld::GridRenderType {
	type Iterator = std::array::IntoIter<Self, 2>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Scale => "Scale",
			Self::Tile => "Tile",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Scale, Self::Tile].into_iter()
	}
}

impl EnumTools for uld::SheetType {
	type Iterator = std::array::IntoIter<Self, 2>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Addon => "Addon",
			Self::Lobby => "Lobby",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Addon, Self::Lobby].into_iter()
	}
}

impl EnumTools for uld::Node {
	type Iterator = std::array::IntoIter<Self, 8>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Image(_) => "Image",
			Self::Text(_) => "Text",
			Self::NineGrid(_) => "Nine Grid",
			Self::Counter(_) => "Counter",
			Self::Collision(_) => "Collision",
			Self::Component(_) => "Component",
			Self::Unknown(_) => "Unknown",
			Self::Other(_) => "Other",
		}
	}
	
	fn to_string(&self) -> String {
		match self {
			Self::Component(v) => format!("Component ({}) {}", v.component_id, v.component_node_data.get_type().to_str()),
			Self::Other(v) => format!("Other ({v})"),
			_ => self.to_str().to_string(),
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Image(Default::default()), Self::Text(Default::default()), Self::NineGrid(Default::default()), Self::Counter(Default::default()), Self::Collision(Default::default()), Self::Component(Default::default()), Self::Unknown(Default::default()), Self::Other(Default::default())].into_iter()
	}
}

// timeline.rs
impl EnumTools for uld::KeyUsage {
	type Iterator = std::array::IntoIter<Self, 8>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Position => "Position",
			Self::Rotation => "Rotation",
			Self::Scale => "Scale",
			Self::Alpha => "Alpha",
			Self::NodeColor => "Node Color",
			Self::TextColor => "Text Color",
			Self::EdgeColor => "Edge Color",
			Self::Number => "Number",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Position, Self::Rotation, Self::Scale, Self::Alpha, Self::NodeColor, Self::TextColor, Self::EdgeColor, Self::Number].into_iter()
	}
}

impl EnumTools for uld::KeyGroupType {
	type Iterator = std::array::IntoIter<Self, 26>;
	
	fn to_str(&self) -> &'static str {
		match self {
			Self::Float1 => "Float1",
			Self::Float2 => "Float2",
			Self::Float3 => "Float3",
			Self::SByte1 => "SByte1",
			Self::SByte2 => "SByte2",
			Self::SByte3 => "SByte3",
			Self::Byte1 => "Byte1",
			Self::Byte2 => "Byte2",
			Self::Byte3 => "Byte3",
			Self::Short1 => "Short1",
			Self::Short2 => "Short2",
			Self::Short3 => "Short3",
			Self::UShort1 => "UShort1",
			Self::UShort2 => "UShort2",
			Self::UShort3 => "UShort3",
			Self::Int1 => "Int1",
			Self::Int2 => "Int2",
			Self::Int3 => "Int3",
			Self::UInt1 => "UInt1",
			Self::UInt2 => "UInt2",
			Self::UInt3 => "UInt3",
			Self::Bool1 => "Bool1",
			Self::Bool2 => "Bool2",
			Self::Bool3 => "Bool3",
			Self::Color => "Color",
			Self::Label => "Label",
		}
	}

	fn iter() -> Self::Iterator {
		[Self::Float1, Self::Float2, Self::Float3, Self::SByte1, Self::SByte2, Self::SByte3, Self::Byte1, Self::Byte2, Self::Byte3, Self::Short1, Self::Short2, Self::Short3, Self::UShort1, Self::UShort2, Self::UShort3, Self::Int1, Self::Int2, Self::Int3, Self::UInt1, Self::UInt2, Self::UInt3, Self::Bool1, Self::Bool2, Self::Bool3, Self::Color, Self::Label].into_iter()
	}
}

impl EnumTools for uld::Keyframes {
	type Iterator = std::array::IntoIter<Self, 26>;
	
	fn to_str(&self) -> &'static str {
		self.get_type().to_str()
	}
	
	fn iter() -> Self::Iterator {
		[Self::Float1(Default::default()), Self::Float2(Default::default()), Self::Float3(Default::default()), Self::SByte1(Default::default()), Self::SByte2(Default::default()), Self::SByte3(Default::default()), Self::Byte1(Default::default()), Self::Byte2(Default::default()), Self::Byte3(Default::default()), Self::Short1(Default::default()), Self::Short2(Default::default()), Self::Short3(Default::default()), Self::UShort1(Default::default()), Self::UShort2(Default::default()), Self::UShort3(Default::default()), Self::Int1(Default::default()), Self::Int2(Default::default()), Self::Int3(Default::default()), Self::UInt1(Default::default()), Self::UInt2(Default::default()), Self::UInt3(Default::default()), Self::Bool1(Default::default()), Self::Bool2(Default::default()), Self::Bool3(Default::default()), Self::Color(Default::default()), Self::Label(Default::default())].into_iter()
	}
}