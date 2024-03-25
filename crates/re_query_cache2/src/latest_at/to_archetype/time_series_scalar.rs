// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/to_archetype.rs

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(deprecated)]

use crate::CachedLatestAtResults;
use re_query2::{PromiseResolver, PromiseResult};
use re_types_core::{Archetype, Loggable as _};
use std::sync::Arc;

impl crate::ToArchetype<re_types::archetypes::TimeSeriesScalar> for CachedLatestAtResults {
    #[inline]
    fn to_archetype(
        &self,
        resolver: &PromiseResolver,
    ) -> PromiseResult<re_types::archetypes::TimeSeriesScalar> {
        re_tracing::profile_function!(<re_types::archetypes::TimeSeriesScalar>::name());

        // --- Required ---

        use re_types::components::Scalar;
        let scalar = match self.get_required(<Scalar>::name()) {
            Ok(scalar) => scalar,
            Err(err) => return PromiseResult::Error(Arc::new(err)),
        };
        let scalar = match scalar.to_dense::<Scalar>(resolver).flatten() {
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

        use re_types::components::Radius;
        let radius = if let Some(radius) = self.get(<Radius>::name()) {
            match radius.to_dense::<Radius>(resolver).flatten() {
                PromiseResult::Ready(data) => data.first().cloned(),
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(err) => return PromiseResult::Error(err),
            }
        } else {
            None
        };

        use re_types::components::Color;
        let color = if let Some(color) = self.get(<Color>::name()) {
            match color.to_dense::<Color>(resolver).flatten() {
                PromiseResult::Ready(data) => data.first().cloned(),
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(err) => return PromiseResult::Error(err),
            }
        } else {
            None
        };

        use re_types::components::Text;
        let label = if let Some(label) = self.get(<Text>::name()) {
            match label.to_dense::<Text>(resolver).flatten() {
                PromiseResult::Ready(data) => data.first().cloned(),
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(err) => return PromiseResult::Error(err),
            }
        } else {
            None
        };

        use re_types::components::ScalarScattering;
        let scattered = if let Some(scattered) = self.get(<ScalarScattering>::name()) {
            match scattered.to_dense::<ScalarScattering>(resolver).flatten() {
                PromiseResult::Ready(data) => data.first().cloned(),
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(err) => return PromiseResult::Error(err),
            }
        } else {
            None
        };

        // ---

        let arch = re_types::archetypes::TimeSeriesScalar {
            scalar,
            radius,
            color,
            label,
            scattered,
        };

        PromiseResult::Ready(arch)
    }
}
