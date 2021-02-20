// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Tetsy.

// Tetsy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy.  If not, see <http://www.gnu.org/licenses/>.

//! Path utilities
use std::path::Path;
use std::path::PathBuf;

use home::home_dir;

#[cfg(target_os = "macos")]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Vapory"`, `"Tetsy"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = home_dir().expect("Failed to get home dir");
	home.push("Library");
	home.push(name);
	home
}

#[cfg(windows)]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Vapory"`, `"Tetsy"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = home_dir().expect("Failed to get home dir");
	home.push("AppData");
	home.push("Roaming");
	home.push(name);
	home
}

#[cfg(not(any(target_os = "macos", windows)))]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Vapory"`, `"Tetsy"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = home_dir().expect("Failed to get home dir");
	home.push(format!(".{}", name.to_lowercase()));
	home
}

/// Get the specific folder inside a config path.
pub fn config_path_with(name: &str, then: &str) -> PathBuf {
	let mut path = config_path(name);
	path.push(then);
	path
}

/// Default vapory paths
pub mod vapory {
	use std::path::PathBuf;

	/// Default path for vapory installation on Mac Os
	pub fn default() -> PathBuf { super::config_path("Vapory") }

	/// Default path for vapory installation (testnet)
	pub fn test() -> PathBuf {
		let mut path = default();
		path.push("testnet");
		path
	}

	/// Get the specific folder inside default vapory installation
	pub fn with_default(s: &str) -> PathBuf {
		let mut path = default();
		path.push(s);
		path
	}

	/// Get the specific folder inside default vapory installation configured for testnet
	pub fn with_testnet(s: &str) -> PathBuf {
		let mut path = default();
		path.push("testnet");
		path.push(s);
		path
	}
}

/// Restricts the permissions of given path only to the owner.
#[cfg(unix)]
pub fn restrict_permissions_owner(file_path: &Path, write: bool, executable: bool) -> Result<(), String>  {
	let perms = ::std::os::unix::fs::PermissionsExt::from_mode(0o400 + write as u32 * 0o200 + executable as u32 * 0o100);
	::std::fs::set_permissions(file_path, perms).map_err(|e| format!("{:?}", e))
}

/// Restricts the permissions of given path only to the owner.
#[cfg(not(unix))]
pub fn restrict_permissions_owner(_file_path: &Path, _write: bool, _executable: bool) -> Result<(), String>  {
	//TODO: implement me
	Ok(())
}
