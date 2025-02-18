// SPDX-License-Identifier: Zlib
use argh::FromArgs;
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use serde::Serialize;
use std::{
	/*
	 * btreemap/set over hashmap/set bc performance doesn't really matter too much here,
	 * and having everything sorted pleases my kbity brain
	 */
	collections::{BTreeMap, BTreeSet},
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(FromArgs)]
/// doohickey to automatically pre-calculate an initial cache for
/// /proc/icon_exists in SS13.
struct Args {
	/// whether to output prettified json or not
	#[argh(switch, short = 'p')]
	pretty: bool,

	/// whether to use associative ("state" = TRUE) lists or not
	#[argh(switch, short = 'a')]
	assoc: bool,

	/// the input directory to search for dmi files in
	#[argh(option, short = 'i')]
	input: PathBuf,

	/// the JSON file to output the resulting JSON to
	#[argh(option, short = 'o')]
	output: PathBuf,
}

#[derive(Serialize)]
#[serde(untagged)]
enum IconStates {
	Array(BTreeSet<String>),
	Assoc(BTreeMap<String, bool>),
}

#[derive(Serialize)]
struct Output {
	revision: Option<String>,
	icons: BTreeMap<String, IconStates>,
}

impl IconStates {
	pub fn new(states: Vec<String>, assoc: bool) -> Self {
		if assoc {
			Self::Assoc(states.into_iter().map(|state| (state, true)).collect())
		} else {
			Self::Array(states.into_iter().collect())
		}
	}

	pub fn len(&self) -> usize {
		match self {
			Self::Array(states) => states.len(),
			Self::Assoc(states) => states.len(),
		}
	}
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = argh::from_env::<Args>();

	let base_dir = &args.input;
	assert!(base_dir.is_dir(), "input arg must be a directory");
	let out_json = &args.output;

	let mut all_icons = BTreeMap::<String, IconStates>::new();
	let mut file_amt = 0_usize;
	let mut state_amt = 0_usize;
	for entry in WalkDir::new(base_dir).into_iter() {
		let entry = entry?;
		let path = entry.path();
		if !path.is_file() || path.extension().unwrap_or_default().to_str() != Some("dmi") {
			continue;
		}
		let relative_path = path
			.strip_prefix(base_dir)
			.wrap_err_with(|| {
				format!(
					"failed to strip base dir prefix ({}) from {}",
					base_dir.display(),
					path.display()
				)
			})?
			.to_str()
			.wrap_err("non-UTF-8 path")?
			.replace('\\', "/");

		let states = get_dmi_states(path)
			.map(|states| IconStates::new(states, args.assoc))
			.wrap_err_with(|| format!("failed to read dmi file at {}", path.display()))?;

		file_amt += 1;
		state_amt += states.len();
		assert!(
			all_icons.insert(relative_path, states).is_none(),
			"somehow we have duplicate icon files for {}",
			path.display()
		);
	}

	let revision = gix::open(base_dir).ok().and_then(|repo| {
		repo.rev_parse_single("HEAD")
			.ok()
			.map(|rev| rev.to_string())
	});
	let output = Output {
		revision,
		icons: all_icons,
	};

	let state_json = if args.pretty {
		serde_json::to_string_pretty(&output)
	} else {
		serde_json::to_string(&output)
	}
	.wrap_err("failed to serialize to json")?;
	std::fs::write(out_json, state_json)
		.wrap_err_with(|| format!("failed to write output to {}", out_json.display()))?;

	println!("found a total of {state_amt} icon states across {file_amt} dmi files");
	println!("wrote to {}", out_json.display());

	Ok(())
}

fn get_dmi_states(path: &Path) -> Result<Vec<String>> {
	let file = File::open(path)
		.map(BufReader::new)
		.wrap_err("failed to open file for reading")?;
	let decoder = png::Decoder::new(file);
	let reader = decoder.read_info().wrap_err("failed to read png info")?;
	let info = reader.info();
	let mut states = Vec::<String>::new();
	for ztxt in &info.compressed_latin1_text {
		let text = ztxt.get_text().wrap_err("failed to read ztxt data")?;
		text.lines()
			.take_while(|line| !line.contains("# END DMI"))
			.filter_map(|line| {
				line.trim()
					.strip_prefix("state = \"")
					.and_then(|line| line.strip_suffix('"'))
			})
			.for_each(|state| {
				states.push(state.to_owned());
			});
	}
	Ok(states)
}
