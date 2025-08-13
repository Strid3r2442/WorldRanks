use std::collections::HashMap;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(EnumIter, Display, EnumString, Clone)]
pub enum SortBy {
	Name,
	Population,
	Area,
}

#[derive(EnumIter, Display, EnumString, PartialEq, Copy, Clone, serde::Serialize)]
pub enum Region {
	Americas,
	Antarctic,
	Africa,
	Asia,
	Europe,
	Oceania
}

impl<'de> serde::Deserialize<'de> for Region {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		<Region as std::str::FromStr>::from_str(&s).map_err(serde::de::Error::custom)
	}
}

#[derive(EnumIter, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Status {
	#[strum(to_string = "Member of the United Nations")]
	UN,
	Independent
}

#[derive(Clone, Copy)]
pub enum FilterQuery<'a> {
	Text(&'a str),
	Region(&'a [Region]),
	Status(&'a HashMap<Status, bool>),
}
