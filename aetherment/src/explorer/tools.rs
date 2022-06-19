use crate::IRONWORKS;

struct NullWriter();
impl ironworks::file::File for NullWriter {
	fn read<'a>(_: impl Into<std::borrow::Cow<'a, [u8]>>) -> noumenon::formats::game::Result<Self> {
		Ok(NullWriter())
	}
}

ffi!(fn explorer_path_valid(path: &str) -> bool {
	// TODO: add way to check if a path is valid to ironworks, this is stupid
	IRONWORKS.file::<NullWriter>(path).is_ok()
});
