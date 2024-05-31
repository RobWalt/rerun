// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

#[derive(Clone, Debug, PartialEq)]
pub struct AffixFuzzer3 {
    pub fuzz2001: Option<crate::testing::components::AffixFuzzer1>,
    pub fuzz2002: Option<crate::testing::components::AffixFuzzer2>,
    pub fuzz2003: Option<crate::testing::components::AffixFuzzer3>,
    pub fuzz2004: Option<crate::testing::components::AffixFuzzer4>,
    pub fuzz2005: Option<crate::testing::components::AffixFuzzer5>,
    pub fuzz2006: Option<crate::testing::components::AffixFuzzer6>,
    pub fuzz2007: Option<crate::testing::components::AffixFuzzer7>,
    pub fuzz2008: Option<crate::testing::components::AffixFuzzer8>,
    pub fuzz2009: Option<crate::testing::components::AffixFuzzer9>,
    pub fuzz2010: Option<crate::testing::components::AffixFuzzer10>,
    pub fuzz2011: Option<crate::testing::components::AffixFuzzer11>,
    pub fuzz2012: Option<crate::testing::components::AffixFuzzer12>,
    pub fuzz2013: Option<crate::testing::components::AffixFuzzer13>,
    pub fuzz2014: Option<crate::testing::components::AffixFuzzer14>,
    pub fuzz2015: Option<crate::testing::components::AffixFuzzer15>,
    pub fuzz2016: Option<crate::testing::components::AffixFuzzer16>,
    pub fuzz2017: Option<crate::testing::components::AffixFuzzer17>,
    pub fuzz2018: Option<crate::testing::components::AffixFuzzer18>,
}

impl ::re_types_core::SizeBytes for AffixFuzzer3 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.fuzz2001.heap_size_bytes()
            + self.fuzz2002.heap_size_bytes()
            + self.fuzz2003.heap_size_bytes()
            + self.fuzz2004.heap_size_bytes()
            + self.fuzz2005.heap_size_bytes()
            + self.fuzz2006.heap_size_bytes()
            + self.fuzz2007.heap_size_bytes()
            + self.fuzz2008.heap_size_bytes()
            + self.fuzz2009.heap_size_bytes()
            + self.fuzz2010.heap_size_bytes()
            + self.fuzz2011.heap_size_bytes()
            + self.fuzz2012.heap_size_bytes()
            + self.fuzz2013.heap_size_bytes()
            + self.fuzz2014.heap_size_bytes()
            + self.fuzz2015.heap_size_bytes()
            + self.fuzz2016.heap_size_bytes()
            + self.fuzz2017.heap_size_bytes()
            + self.fuzz2018.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::testing::components::AffixFuzzer1>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer2>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer3>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer4>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer5>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer6>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer7>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer8>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer9>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer10>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer11>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer12>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer13>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer14>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer15>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer16>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer17>>::is_pod()
            && <Option<crate::testing::components::AffixFuzzer18>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.testing.components.AffixFuzzer3Indicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 18usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer3Indicator".into(),
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
        ]
    });

impl AffixFuzzer3 {
    /// The total number of components in the archetype: 0 required, 1 recommended, 18 optional
    pub const NUM_COMPONENTS: usize = 19usize;
}

/// Indicator component for the [`AffixFuzzer3`] [`::re_types_core::Archetype`]
pub type AffixFuzzer3Indicator = ::re_types_core::GenericIndicatorComponent<AffixFuzzer3>;

impl ::re_types_core::Archetype for AffixFuzzer3 {
    type Indicator = AffixFuzzer3Indicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer3".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Affix fuzzer 3"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: AffixFuzzer3Indicator = AffixFuzzer3Indicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let fuzz2001 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer1") {
                <crate::testing::components::AffixFuzzer1>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2001")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2002 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer2") {
                <crate::testing::components::AffixFuzzer2>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2002")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2003 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer3") {
                <crate::testing::components::AffixFuzzer3>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2003")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2004 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer4") {
                <crate::testing::components::AffixFuzzer4>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2004")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2005 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer5") {
                <crate::testing::components::AffixFuzzer5>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2005")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2006 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer6") {
                <crate::testing::components::AffixFuzzer6>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2006")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2007 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer7") {
                <crate::testing::components::AffixFuzzer7>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2007")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2008 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer8") {
                <crate::testing::components::AffixFuzzer8>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2008")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2009 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer9") {
                <crate::testing::components::AffixFuzzer9>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2009")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2010 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer10") {
                <crate::testing::components::AffixFuzzer10>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2010")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2011 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer11") {
                <crate::testing::components::AffixFuzzer11>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2011")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2012 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer12") {
                <crate::testing::components::AffixFuzzer12>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2012")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2013 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer13") {
                <crate::testing::components::AffixFuzzer13>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2013")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2014 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer14") {
                <crate::testing::components::AffixFuzzer14>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2014")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2015 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer15") {
                <crate::testing::components::AffixFuzzer15>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2015")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2016 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer16") {
                <crate::testing::components::AffixFuzzer16>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2016")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2017 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer17") {
                <crate::testing::components::AffixFuzzer17>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2017")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let fuzz2018 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer18") {
                <crate::testing::components::AffixFuzzer18>::from_arrow_opt(&**array)
                    .with_context("rerun.testing.archetypes.AffixFuzzer3#fuzz2018")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            fuzz2001,
            fuzz2002,
            fuzz2003,
            fuzz2004,
            fuzz2005,
            fuzz2006,
            fuzz2007,
            fuzz2008,
            fuzz2009,
            fuzz2010,
            fuzz2011,
            fuzz2012,
            fuzz2013,
            fuzz2014,
            fuzz2015,
            fuzz2016,
            fuzz2017,
            fuzz2018,
        })
    }
}

impl ::re_types_core::AsComponents for AffixFuzzer3 {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.fuzz2001
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2002
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2003
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2004
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2005
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2006
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2007
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2008
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2009
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2010
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2011
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2012
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2013
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2014
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2015
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2016
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2017
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.fuzz2018
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl AffixFuzzer3 {
    /// Create a new `AffixFuzzer3`.
    #[inline]
    pub fn new() -> Self {
        Self {
            fuzz2001: None,
            fuzz2002: None,
            fuzz2003: None,
            fuzz2004: None,
            fuzz2005: None,
            fuzz2006: None,
            fuzz2007: None,
            fuzz2008: None,
            fuzz2009: None,
            fuzz2010: None,
            fuzz2011: None,
            fuzz2012: None,
            fuzz2013: None,
            fuzz2014: None,
            fuzz2015: None,
            fuzz2016: None,
            fuzz2017: None,
            fuzz2018: None,
        }
    }

    #[inline]
    pub fn with_fuzz2001(
        mut self,
        fuzz2001: impl Into<crate::testing::components::AffixFuzzer1>,
    ) -> Self {
        self.fuzz2001 = Some(fuzz2001.into());
        self
    }

    #[inline]
    pub fn with_fuzz2002(
        mut self,
        fuzz2002: impl Into<crate::testing::components::AffixFuzzer2>,
    ) -> Self {
        self.fuzz2002 = Some(fuzz2002.into());
        self
    }

    #[inline]
    pub fn with_fuzz2003(
        mut self,
        fuzz2003: impl Into<crate::testing::components::AffixFuzzer3>,
    ) -> Self {
        self.fuzz2003 = Some(fuzz2003.into());
        self
    }

    #[inline]
    pub fn with_fuzz2004(
        mut self,
        fuzz2004: impl Into<crate::testing::components::AffixFuzzer4>,
    ) -> Self {
        self.fuzz2004 = Some(fuzz2004.into());
        self
    }

    #[inline]
    pub fn with_fuzz2005(
        mut self,
        fuzz2005: impl Into<crate::testing::components::AffixFuzzer5>,
    ) -> Self {
        self.fuzz2005 = Some(fuzz2005.into());
        self
    }

    #[inline]
    pub fn with_fuzz2006(
        mut self,
        fuzz2006: impl Into<crate::testing::components::AffixFuzzer6>,
    ) -> Self {
        self.fuzz2006 = Some(fuzz2006.into());
        self
    }

    #[inline]
    pub fn with_fuzz2007(
        mut self,
        fuzz2007: impl Into<crate::testing::components::AffixFuzzer7>,
    ) -> Self {
        self.fuzz2007 = Some(fuzz2007.into());
        self
    }

    #[inline]
    pub fn with_fuzz2008(
        mut self,
        fuzz2008: impl Into<crate::testing::components::AffixFuzzer8>,
    ) -> Self {
        self.fuzz2008 = Some(fuzz2008.into());
        self
    }

    #[inline]
    pub fn with_fuzz2009(
        mut self,
        fuzz2009: impl Into<crate::testing::components::AffixFuzzer9>,
    ) -> Self {
        self.fuzz2009 = Some(fuzz2009.into());
        self
    }

    #[inline]
    pub fn with_fuzz2010(
        mut self,
        fuzz2010: impl Into<crate::testing::components::AffixFuzzer10>,
    ) -> Self {
        self.fuzz2010 = Some(fuzz2010.into());
        self
    }

    #[inline]
    pub fn with_fuzz2011(
        mut self,
        fuzz2011: impl Into<crate::testing::components::AffixFuzzer11>,
    ) -> Self {
        self.fuzz2011 = Some(fuzz2011.into());
        self
    }

    #[inline]
    pub fn with_fuzz2012(
        mut self,
        fuzz2012: impl Into<crate::testing::components::AffixFuzzer12>,
    ) -> Self {
        self.fuzz2012 = Some(fuzz2012.into());
        self
    }

    #[inline]
    pub fn with_fuzz2013(
        mut self,
        fuzz2013: impl Into<crate::testing::components::AffixFuzzer13>,
    ) -> Self {
        self.fuzz2013 = Some(fuzz2013.into());
        self
    }

    #[inline]
    pub fn with_fuzz2014(
        mut self,
        fuzz2014: impl Into<crate::testing::components::AffixFuzzer14>,
    ) -> Self {
        self.fuzz2014 = Some(fuzz2014.into());
        self
    }

    #[inline]
    pub fn with_fuzz2015(
        mut self,
        fuzz2015: impl Into<crate::testing::components::AffixFuzzer15>,
    ) -> Self {
        self.fuzz2015 = Some(fuzz2015.into());
        self
    }

    #[inline]
    pub fn with_fuzz2016(
        mut self,
        fuzz2016: impl Into<crate::testing::components::AffixFuzzer16>,
    ) -> Self {
        self.fuzz2016 = Some(fuzz2016.into());
        self
    }

    #[inline]
    pub fn with_fuzz2017(
        mut self,
        fuzz2017: impl Into<crate::testing::components::AffixFuzzer17>,
    ) -> Self {
        self.fuzz2017 = Some(fuzz2017.into());
        self
    }

    #[inline]
    pub fn with_fuzz2018(
        mut self,
        fuzz2018: impl Into<crate::testing::components::AffixFuzzer18>,
    ) -> Self {
        self.fuzz2018 = Some(fuzz2018.into());
        self
    }
}
