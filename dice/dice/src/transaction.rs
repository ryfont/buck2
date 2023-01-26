/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::ops::Deref;
use std::thread;

use allocative::Allocative;
use dupe::Dupe;

use crate::DiceComputations;
use crate::VersionNumber;

/// The base struct for which all computations start. This is clonable, and dupe, and can be
/// moved to different runtimes to start computations.
/// All computations on this transaction will see only changes at the most-up-to-date version at
/// the time of creation of this transaction.
///
/// This SHOULD NOT be ever stored by computations, or any results of computations.
#[derive(Allocative)]
pub struct DiceTransaction(pub(super) DiceComputations);

impl DiceTransaction {
    /// Commit the changes registered via 'changed' and 'changed_to' to the current newest version.
    /// This can only be called when the this is the only node remaining in the computation graph
    pub fn commit(self) -> DiceTransaction {
        DiceTransaction(self.0.0.commit())
    }

    pub fn unstable_take(self) -> Self {
        let map = self.0.0.unstable_take();
        // Destructors can be slow, so we do this in a separate thread.
        thread::spawn(|| drop(map));
        self
    }

    /// Returns whether the `DiceTransaction` is equivalent. Equivalent is defined as whether the
    /// two Transactions are based off the same underlying set of key states. That is, all
    /// injected keys are the same, and the same compute keys are dirtied, and that any computations
    /// that occur between the two transactions can be shared.
    pub fn equivalent<E>(&self, other: &E) -> bool
    where
        E: DiceEquivalent,
    {
        self.version_for_equivalence() == other.version_for_equivalence()
    }

    pub fn version(&self) -> VersionNumber {
        self.0.0.transaction_ctx.get_version()
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for DiceTransaction {}

    impl Sealed for VersionNumber {}
}

pub trait DiceEquivalent: private::Sealed {
    fn version_for_equivalence(&self) -> VersionNumber;
}

impl DiceEquivalent for DiceTransaction {
    fn version_for_equivalence(&self) -> VersionNumber {
        self.version()
    }
}

impl DiceEquivalent for VersionNumber {
    fn version_for_equivalence(&self) -> VersionNumber {
        *self
    }
}

impl Deref for DiceTransaction {
    type Target = DiceComputations;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for DiceTransaction {
    fn clone(&self) -> Self {
        Self(DiceComputations(self.0.0.dupe()))
    }
}

impl Dupe for DiceTransaction {}