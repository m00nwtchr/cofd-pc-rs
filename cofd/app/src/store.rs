#[cfg(not(target_arch = "wasm32"))]
use directories::ProjectDirs;
#[cfg(not(target_arch = "wasm32"))]
use std::{
	fs::File,
	io::{Read, Write},
	path::PathBuf,
};

use serde::{Deserialize, Serialize};

use anyhow::anyhow;
use cfg_if::cfg_if;

pub struct Store {
	#[cfg(target_arch = "wasm32")]
	local_storage: web_sys::Storage,
	#[cfg(not(target_arch = "wasm32"))]
	path: PathBuf,
}

impl Store {
	pub fn new() -> Option<Store> {
		let mut store;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				let window = web_sys::window()?;
				if let Ok(Some(local_storage)) = window.local_storage() {
					store = Some(Self {
						local_storage,
					});
				} else {
					store = None;
				}
			} else {
				let path = Store::save_path();

				let dir = path.parent();
				if dir.is_some() && !dir.unwrap().exists() {
					std::fs::create_dir_all(dir.unwrap()).ok()?;
				}
				// let file = File::create(path).ok()?;

				store = Some(Self {
					path
				});
			}
		}

		store
	}

	pub fn get<T: for<'a> Deserialize<'a>>(&self, name: &str) -> anyhow::Result<Option<T>> {
		let mut str;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				str = self.local_storage.get_item(name).map_err(|err| anyhow!("{:?}", err))?
			} else {
				str = Some(std::fs::read_to_string(self.path.clone())?);
			}
		}

		if let Some(str) = str {
			Ok(Some(ron::de::from_str(&str)?))
		} else {
			Ok(None)
		}
	}

	pub fn set<T: Serialize>(&self, name: &str, value: &T) -> anyhow::Result<()> {
		let str = ron::ser::to_string(value)?;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				self.local_storage.set_item(name, &str).map_err(|err| anyhow!("{:?}", err))?;
			} else {
				std::fs::write(self.path.clone(), str)?;
			}
		}

		Ok(())
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn save_path() -> PathBuf {
		ProjectDirs::from("", "", "cofd-pc")
			.unwrap()
			.data_dir()
			.join("characters.ron")
	}
}
