#[repr(u8)]
pub enum LogType {
	Log = 0,
	Error = 1,
	Fatal = 255,
}

pub(crate) static mut LOG: fn(LogType, String) = |_, _| {};
#[macro_export]
macro_rules! log {
	(ftl, $($e:tt)*) => {unsafe{crate::log::LOG(crate::log::LogType::Fatal, format!($($e)*))}};
	(log, $($e:tt)*) => {unsafe{crate::log::LOG(crate::log::LogType::Log, format!($($e)*))}};
	(err, $($e:tt)*) => {unsafe{crate::log::LOG(crate::log::LogType::Error, format!($($e)*))}};
	($($e:tt)*) => {unsafe{crate::log::LOG(crate::log::LogType::Log, format!($($e)*))}};
}