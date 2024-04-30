// Copyright (C) 2024 The Software Heritage developers
// See the AUTHORS file at the top-level directory of this distribution
// License: GNU General Public License version 3, or any later version
// See top-level LICENSE file for more information

#[cfg(feature = "check")]
use thiserror::Error;

use crate::{Hashable, SinglePhf};

#[cfg(feature = "check")]
#[derive(Error, Debug)]
pub enum ViolatedInvariant {
    #[error("Hash is {position} but it should be lower than {table_size}")]
    PositionOutOfRange { position: u64, table_size: u64 },

    #[error("Hash is {position} but the function has only {num_keys} and should be minimal")]
    NotMinimal {
        position: u64,
        table_size: u64,
        num_keys: u64,
    },

    #[error("Two keys have the same hash ({duplicate_hash})")]
    Duplicates { duplicate_hash: u64 },

    #[error("Table size ({table_size}) is lower thannumber of keys ({num_keys})")]
    MismatchedTableSize { table_size: u64, num_keys: u64 },
}

#[cfg(feature = "check")]
/// Checks the function is injective (and bijective in `[0; num_keys)`, if [`Self::MINIMAL`])
pub fn check<Keys: IntoIterator, F: SinglePhf>(keys: Keys, f: &F) -> Result<(), ViolatedInvariant>
where
    <<Keys as IntoIterator>::IntoIter as Iterator>::Item: Hashable,
{
    if f.table_size() < f.num_keys() {
        return Err(ViolatedInvariant::MismatchedTableSize {
            table_size: f.table_size(),
            num_keys: f.num_keys(),
        });
    }

    let keys = keys.into_iter();
    let mut present = sux::bits::BitVec::new(
        f.table_size()
            .try_into()
            .expect("function's table_size overflowed usize"),
    );
    for key in keys {
        let position = f.hash(key);
        let position_usize: usize =
            position
                .try_into()
                .map_err(|_| ViolatedInvariant::PositionOutOfRange {
                    position: usize::MAX as u64,
                    table_size: f.table_size(),
                })?;
        if position >= f.table_size() {
            return Err(ViolatedInvariant::PositionOutOfRange {
                position,
                table_size: f.table_size(),
            });
        }
        if F::MINIMAL && position >= f.num_keys() {
            return Err(ViolatedInvariant::NotMinimal {
                position,
                table_size: f.table_size(),
                num_keys: f.num_keys(),
            });
        }
        if present.get(position_usize) {
            return Err(ViolatedInvariant::Duplicates {
                duplicate_hash: position,
            });
        }
        present.set(position_usize, true);
    }

    Ok(())
}
