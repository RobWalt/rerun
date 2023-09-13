// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points2d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A 2D point cloud with positions and optional colors, radii, labels, etc.
///
/// ## Example
///
/// ```ignore
/// //! Log some very simple points.
///
/// use rerun::{
///    archetypes::{Boxes2D, Points2D},
///    RecordingStreamBuilder,
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let (rec, storage) = RecordingStreamBuilder::new("rerun_example_points2d").memory()?;
///
///    rec.log("points", &Points2D::new([(0.0, 0.0), (1.0, 1.0)]))?;
///
///    // Log an extra rect to set the view bounds
///    rec.log("bounds", &Boxes2D::from_half_sizes([(2.0, 1.5)]))?;
///
///    rerun::native_viewer::show(storage.take())?;
///    Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Points2D {
    /// All the actual 2D points that make up the point cloud.
    pub points: Vec<crate::components::Point2D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the points.
    pub labels: Option<Vec<crate::components::Text>>,

    /// An optional floating point value that specifies the 2D drawing order.
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<crate::components::DrawOrder>,

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    pub keypoint_ids: Option<Vec<crate::components::KeypointId>>,

    /// Unique identifiers for each individual point in the batch.
    pub instance_keys: Option<Vec<crate::components::InstanceKey>>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.point2d".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.colorrgba".into(),
            "rerun.components.Points2DIndicator".into(),
            "rerun.radius".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.class_id".into(),
            "rerun.draw_order".into(),
            "rerun.instance_key".into(),
            "rerun.keypoint_id".into(),
            "rerun.label".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.point2d".into(),
            "rerun.colorrgba".into(),
            "rerun.components.Points2DIndicator".into(),
            "rerun.radius".into(),
            "rerun.class_id".into(),
            "rerun.draw_order".into(),
            "rerun.instance_key".into(),
            "rerun.keypoint_id".into(),
            "rerun.label".into(),
        ]
    });

impl Points2D {
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Points2D`] [`crate::Archetype`]
pub type Points2DIndicator = crate::GenericIndicatorComponent<Points2D>;

impl crate::Archetype for Points2D {
    type Indicator = Points2DIndicator;

    #[inline]
    fn name() -> crate::ArchetypeName {
        "rerun.archetypes.Points2D".into()
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [crate::ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        self.points.len()
    }

    fn as_component_batches(&self) -> Vec<crate::MaybeOwnedComponentBatch<'_>> {
        [
            Some(Self::Indicator::batch(self.num_instances() as _).into()),
            Some((&self.points as &dyn crate::ComponentBatch).into()),
            self.radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.draw_order
                .as_ref()
                .map(|comp| (comp as &dyn crate::ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.keypoint_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
            self.instance_keys
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn crate::ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn try_to_arrow(
        &self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::{Loggable as _, ResultExt as _};
        Ok([
            {
                Some({
                    let array = <crate::components::Point2D>::try_to_arrow(self.points.iter());
                    array.map(|array| {
                        let datatype = ::arrow2::datatypes::DataType::Extension(
                            "rerun.components.Point2D".into(),
                            Box::new(array.data_type().clone()),
                            Some("rerun.point2d".into()),
                        );
                        (
                            ::arrow2::datatypes::Field::new("points", datatype, false),
                            array,
                        )
                    })
                })
                .transpose()
                .with_context("rerun.archetypes.Points2D#points")?
            },
            {
                self.radii
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Radius>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Radius".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.radius".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("radii", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#radii")?
            },
            {
                self.colors
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Color>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Color".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.colorrgba".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("colors", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#colors")?
            },
            {
                self.labels
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::Text>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.Text".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.label".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("labels", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#labels")?
            },
            {
                self.draw_order
                    .as_ref()
                    .map(|single| {
                        let array = <crate::components::DrawOrder>::try_to_arrow([single]);
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.DrawOrder".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.draw_order".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("draw_order", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#draw_order")?
            },
            {
                self.class_ids
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::ClassId>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.ClassId".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.class_id".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("class_ids", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#class_ids")?
            },
            {
                self.keypoint_ids
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::KeypointId>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.KeypointId".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.keypoint_id".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("keypoint_ids", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
            },
            {
                self.instance_keys
                    .as_ref()
                    .map(|many| {
                        let array = <crate::components::InstanceKey>::try_to_arrow(many.iter());
                        array.map(|array| {
                            let datatype = ::arrow2::datatypes::DataType::Extension(
                                "rerun.components.InstanceKey".into(),
                                Box::new(array.data_type().clone()),
                                Some("rerun.instance_key".into()),
                            );
                            (
                                ::arrow2::datatypes::Field::new("instance_keys", datatype, false),
                                array,
                            )
                        })
                    })
                    .transpose()
                    .with_context("rerun.archetypes.Points2D#instance_keys")?
            },
        ]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        arrow_data: impl IntoIterator<
            Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>),
        >,
    ) -> crate::DeserializationResult<Self> {
        use crate::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let points = {
            let array = arrays_by_name
                .get("points")
                .ok_or_else(crate::DeserializationError::missing_data)
                .with_context("rerun.archetypes.Points2D#points")?;
            <crate::components::Point2D>::try_from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points2D#points")?
                .into_iter()
                .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Points2D#points")?
        };
        let radii = if let Some(array) = arrays_by_name.get("radii") {
            Some({
                <crate::components::Radius>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("colors") {
            Some({
                <crate::components::Color>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("labels") {
            Some({
                <crate::components::Text>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#labels")?
            })
        } else {
            None
        };
        let draw_order = if let Some(array) = arrays_by_name.get("draw_order") {
            Some({
                <crate::components::DrawOrder>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#draw_order")?
                    .into_iter()
                    .next()
                    .flatten()
                    .ok_or_else(crate::DeserializationError::missing_data)
                    .with_context("rerun.archetypes.Points2D#draw_order")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("class_ids") {
            Some({
                <crate::components::ClassId>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#class_ids")?
            })
        } else {
            None
        };
        let keypoint_ids = if let Some(array) = arrays_by_name.get("keypoint_ids") {
            Some({
                <crate::components::KeypointId>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#keypoint_ids")?
            })
        } else {
            None
        };
        let instance_keys = if let Some(array) = arrays_by_name.get("instance_keys") {
            Some({
                <crate::components::InstanceKey>::try_from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points2D#instance_keys")?
                    .into_iter()
                    .map(|v| v.ok_or_else(crate::DeserializationError::missing_data))
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points2D#instance_keys")?
            })
        } else {
            None
        };
        Ok(Self {
            points,
            radii,
            colors,
            labels,
            draw_order,
            class_ids,
            keypoint_ids,
            instance_keys,
        })
    }
}

impl Points2D {
    pub fn new(points: impl IntoIterator<Item = impl Into<crate::components::Point2D>>) -> Self {
        Self {
            points: points.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            draw_order: None,
            class_ids: None,
            keypoint_ids: None,
            instance_keys: None,
        }
    }

    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = Some(draw_order.into());
        self
    }

    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_keypoint_ids(
        mut self,
        keypoint_ids: impl IntoIterator<Item = impl Into<crate::components::KeypointId>>,
    ) -> Self {
        self.keypoint_ids = Some(keypoint_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_instance_keys(
        mut self,
        instance_keys: impl IntoIterator<Item = impl Into<crate::components::InstanceKey>>,
    ) -> Self {
        self.instance_keys = Some(instance_keys.into_iter().map(Into::into).collect());
        self
    }
}
