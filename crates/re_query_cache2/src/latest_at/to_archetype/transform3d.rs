// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/to_archetype.rs

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]

use crate::CachedLatestAtResults;
use re_query2::{PromiseResolver, PromiseResult};
use re_types_core::{Archetype, Loggable as _};
use std::sync::Arc;

impl crate::ToArchetype<re_types::archetypes::Transform3D> for CachedLatestAtResults {
    #[inline]
    fn to_archetype(
        &self,
        resolver: &PromiseResolver,
    ) -> PromiseResult<re_types::archetypes::Transform3D> {
        re_tracing::profile_function!(<re_types::archetypes::Transform3D>::name());

        // --- Required ---

        use re_types::components::Transform3D;
        let transform = match self.get_required(<Transform3D>::name()) {
            Ok(transform) => transform,
            Err(err) => return PromiseResult::Error(Arc::new(err)),
        };
        let transform = match transform.to_dense::<Transform3D>(resolver).flatten() {
            PromiseResult::Ready(data) => {
                let Some(first) = data.first().cloned() else {
                    return PromiseResult::Error(std::sync::Arc::new(
                        re_types_core::DeserializationError::missing_data(),
                    ));
                };
                first
            }
            PromiseResult::Pending => return PromiseResult::Pending,
            PromiseResult::Error(err) => return PromiseResult::Error(err),
        };

        // --- Recommended/Optional ---

        // ---

        let arch = re_types::archetypes::Transform3D { transform };

        PromiseResult::Ready(arch)
    }
}
