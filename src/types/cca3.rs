use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CCA3([u8; 3]);

#[derive(Debug)]
pub struct InvalidCCA3;

impl fmt::Display for CCA3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = std::str::from_utf8(&self.0).unwrap();
		write!(f, "{}", s)
	}
}
impl fmt::Display for InvalidCCA3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Invalid country code: must be exactly 3 uppercase ASCII letters (ISO 3166-1 alpha-3)"
		)
	}
}

impl<'de> Deserialize<'de> for CCA3 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		CCA3::from_str(&s).map_err(serde::de::Error::custom)
	}
}

impl std::error::Error for InvalidCCA3 {}

impl FromStr for CCA3 {
	type Err = InvalidCCA3;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() == 3 && s.chars().all(|c| c.is_ascii_uppercase()) {
			let mut buf = [0u8; 3];
			buf.copy_from_slice(s.as_bytes());
			Ok(CCA3(buf))
		} else {
			Err(InvalidCCA3)
		}
	}
}

impl CCA3 {
	pub fn as_str(&self) -> &str {
		std::str::from_utf8(&self.0).unwrap()
	}
}