use serde::{Deserialize, Serialize};

pub enum Data {
	Gift(Gift),
	Facet(Facet),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gift {
	pub name: String,
	pub typek: String,
	pub facets: Vec<Facet>,
}

impl Gift {
	pub fn new(name: String, typek: String) -> Self {
		Self {
			name,
			typek,
			facets: vec![],
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Facet {
	pub name: String,
	pub level: Option<u16>, // Moon Gift
	pub renown: Option<String>, // Wolf/Shadow Gift
	                       // pub cost: String,
	                       // pub pool: String,
	                       // pub action: String,
	                       // pub duration: String,
	                       // pub description: String,
	                       // pub reference: String,
}

impl Facet {
	pub fn new(name: String, level: Option<u16>, renown: Option<String>) -> Self {
		Self {
			name,
			level,
			renown,
			// cost: arr[2].clone(),
			// pool: arr[3].clone(),
			// action: arr[4].clone(),
			// duration: arr[5].clone(),
			// description: arr[6].clone(),
			// reference: arr[7].clone(),
		}
	}
}