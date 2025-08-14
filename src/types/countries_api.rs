use serde::{Deserialize};
use std::collections::HashMap;
use crate::types::{CCA3, Region};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Name {
	pub common: String,
	pub official: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Currency {
	pub symbol: String,
	pub name: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Flags {
	pub png: String,
	pub svg: String,
	pub alt: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Country {
	pub name: Name,
	pub currencies: Option<HashMap<String, Currency>>,
	pub capital: Option<Vec<String>>,
	pub region: String,
	#[serde(rename = "subregion")]
	pub sub_region: Option<String>,
	pub languages: Option<HashMap<String, String>>,
	pub borders: Option<Vec<CCA3>>,
	pub area: f32,
	pub population: u32,
	pub flags: Flags,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct CountryOverview {
	pub name: Name,
	pub cca3: CCA3,
	pub independent: bool,
	#[serde(rename = "unMember")]
	pub un_member: bool,
	pub region: Region,
	#[serde(rename = "subregion")]
	pub sub_region: String,
	pub area: f32,
	pub population: u32,
	pub flags: Flags,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct NeighbouringCountry {
	pub name: Name,
	pub flags: Flags,
	pub cca3: CCA3,
}
