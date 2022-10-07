use super::reduced_pallet::*;
use super::reduced_runtime::*;
use crate::differs::reduced::calls::prelude::Index;
use comparable::{Changed, MapChange};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// #[derive(Deserialize)]
// #[serde(remote = "ReducedRuntimeChange")]
// struct ReducedRuntimeChangeDef {}

// #[derive(Serialize)]
// #[serde(remote = "Changed<ReducedRuntimeChange>")]
// struct ChangedDef(Changed<ReducedRuntimeChange>);
// #[derive(Serialize)]
// #[serde(remote = "ChangedDef<ReducedRuntimeChange>")]
// pub enum EnumChangedDef {
// 	Unchanged,
// 	Changed(ReducedRuntimeChange),
// }

// TODO: Rename that
// pub type CompOutput = Changed<ReducedRuntimeChange>;
#[derive(Debug, Serialize)]
pub struct CompOutput(pub(crate) ReducedRuntimeChange);

#[derive(Debug, Serialize)]
/// Currently a wrapper around `Changed<ReducedRuntimeChange>` but that will likely improve.
pub struct ChangedWrapper(pub(crate) CompOutput);

impl From<CompOutput> for ChangedWrapper {
	fn from(c: CompOutput) -> Self {
		Self(c)
	}
}

impl From<ReducedRuntimeChange> for CompOutput {
	fn from(rrc: ReducedRuntimeChange) -> Self {
		Self(rrc)
	}
}

pub enum Filter<T> {
	None,
	One(T),
	Some(Vec<T>),
	All,
}

#[derive(Debug)]
pub enum MapChangeFilter {
	Added,
	Changed,
	Removed,
}

///// Filter on a struct that has name and index such as call, event, error
// pub struct IndexNameFilter {
// 	index: Option<Index>,
// 	name: Option<NameFilter>,
// }
///// Filter on a struct that only has a name
// pub struct NameFilter {
// 	name: String,
// }

// // TODO: proc macro filterable
// pub struct PalletItemFilter {
// 	index: Option<Index>,
// 	name: Option<String>,
// 	calls: Option,
// 	Removed,
// 	All
// }

pub enum ChangedFilter {
	Unchanged,
	Changed,
	All,
}

// impl ChangedWrapper {
// 	/// Helps filtering pallets based on whether they changed or not and WHAT (call, constant, etc...) has changed.
// 	// TODO: We cannot use comparable to return what has changed AND what did not change as the comparison ONLY returns
// 	// what did change.
// 	pub fn filter_pallets(&self, changed_filter : ChangedFilter) -> &MapChange<Index, ReducedPalletDesc, Vec<ReducedPalletChange>> {
// 		match changed_filter {
// 			ChangedFilter::Unchanged | ChangedFilter::Changed => unimplemented!("May be implemented later as needed"),

// 			ChangedFilter::All => {
// 				match &self.0 {
// 					Changed::Unchanged => f.write_str("UNCHANGED"),
// 					Changed::Changed(reduced_runtime_change) => {
// 						// println!("c = {:?}", c);

// 						reduced_runtime_change
// 							.pallets
// 							.iter()
// 							.collect()
// 					}
// 				}
// 			}
// 		}
// 	}
// }

impl Display for ChangedWrapper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// match &self {
		// 	Changed::Unchanged => f.write_str("UNCHANGED"),
		// 	Changed::Changed(reduced_runtime_change) => {
		self.0 .0.pallets.iter().for_each(
			|mc: &MapChange<Index, ReducedPalletDesc, Vec<ReducedPalletChange>>| match mc {
				MapChange::Added(key, reduced_pallet) => {
					let _ = writeln!(f, "[+] id: {:>2} - new pallet: {}", key, reduced_pallet.name);
				}
				MapChange::Removed(key) => {
					let _ = writeln!(f, "[-] {}", key);
				}

				MapChange::Changed(key, changes) => {
					let _ = writeln!(f, "[≠] pallet: {} -> {} change(s)", key, changes.len());
					changes.iter().for_each(|reduced_pallet_change| {
						let _ = writeln!(f, "{}", reduced_pallet_change);
					});
				}
			},
		);
		Ok(())
		// 	}
		// }
	}
}

impl AsRef<CompOutput> for ChangedWrapper {
	fn as_ref(&self) -> &CompOutput {
		&self.0
	}
}