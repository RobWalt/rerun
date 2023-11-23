// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/collection.hpp>
#include <rerun/result.hpp>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class ListBuilder;
} // namespace arrow

namespace rerun::components {
    struct AffixFuzzer11 {
        std::optional<rerun::Collection<float>> many_floats_optional;

      public:
        AffixFuzzer11() = default;

        AffixFuzzer11(std::optional<rerun::Collection<float>> many_floats_optional_)
            : many_floats_optional(std::move(many_floats_optional_)) {}

        AffixFuzzer11& operator=(std::optional<rerun::Collection<float>> many_floats_optional_) {
            many_floats_optional = std::move(many_floats_optional_);
            return *this;
        }
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::AffixFuzzer11> {
        static constexpr const char Name[] = "rerun.testing.components.AffixFuzzer11";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::ListBuilder* builder, const components::AffixFuzzer11* elements,
            size_t num_elements
        );

        /// Serializes an array of `rerun::components::AffixFuzzer11` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::AffixFuzzer11* instances, size_t num_instances
        );
    };
} // namespace rerun
