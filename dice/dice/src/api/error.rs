/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::sync::Arc;

use allocative::Allocative;
use dupe::Dupe;
use indexmap::IndexSet;
use itertools::Itertools;
use thiserror::Error;

use crate::cycles::RequestedKey;

#[derive(Clone, Dupe, Debug, Error, Allocative)]
#[error(transparent)]
pub struct DiceError(pub(crate) Arc<DiceErrorImpl>);

impl DiceError {
    pub fn cycle(
        trigger: Arc<dyn RequestedKey>,
        cyclic_keys: IndexSet<Arc<dyn RequestedKey>>,
    ) -> Self {
        DiceError(Arc::new(DiceErrorImpl::Cycle {
            trigger,
            cyclic_keys,
        }))
    }

    pub fn duplicate(key: Arc<dyn RequestedKey>) -> Self {
        DiceError(Arc::new(DiceErrorImpl::DuplicateChange(key)))
    }
}

#[derive(Debug, Error, Allocative)]
pub(crate) enum DiceErrorImpl {
    #[error("Cyclic computation detect when computing key `{}`, which forms a cycle in computation chain: `{}`", trigger, cyclic_keys.iter().join(","))]
    Cycle {
        trigger: Arc<dyn RequestedKey>,
        cyclic_keys: IndexSet<Arc<dyn RequestedKey>>,
    },
    #[error("Key `{0}` was marked as changed multiple times on the same transaction.")]
    DuplicateChange(Arc<dyn RequestedKey>),
}

pub type DiceResult<T> = Result<T, DiceError>;