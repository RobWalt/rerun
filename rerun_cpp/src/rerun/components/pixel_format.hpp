// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/pixel_format.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    class Array;
    class DataType;
    class SparseUnionBuilder;
} // namespace arrow

namespace rerun::components {
    /// **Component**: Specifieds a particular format of an `archetypes::Image`.
    ///
    /// Most images can be described by a `components::ColorModel` and a `components::ChannelDataType`,
    /// e.g. `RGB` and `U8` respectively.
    ///
    /// However, some image formats has chroma downsampling and/or
    /// use differing number of bits per channel, and that is what this `components::PixelFormat` is for.
    ///
    /// All these formats support random access.
    ///
    /// For more compressed image formats, see `archetypes::ImageEncoded`.
    enum class PixelFormat : uint8_t {

        /// NV12 (aka Y_UV12) is a YUV 4:2:0 chroma downsampled format with
        /// 12 bits per pixel and 8 bits per channel.
        ///
        /// First comes entire image in Y in one plane,
        /// followed by a plane with interleaved lines ordered as U0, V0, U1, V1, etc.
        NV12 = 1,

        /// YUY2 (aka YUYV or YUYV16), is a YUV 4:2:2 chroma downsampled format with
        /// 16 bits per pixel and 8 bits per channel.
        ///
        /// The order of the channels is Y0, U0, Y1, V0, all in the same plane.
        YUY2 = 2,
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::PixelFormat> {
        static constexpr const char Name[] = "rerun.components.PixelFormat";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::components::PixelFormat` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::PixelFormat* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::SparseUnionBuilder* builder, const components::PixelFormat* elements,
            size_t num_elements
        );
    };
} // namespace rerun
