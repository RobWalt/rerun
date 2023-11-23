// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#include "affix_fuzzer4.hpp"

#include <rerun/collection_adapter_builtins.hpp>

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::AffixFuzzer4>::serialize(
        const archetypes::AffixFuzzer4& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(19);

        if (archetype.fuzz2101.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2101.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2102.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2102.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2103.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2103.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2104.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2104.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2105.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2105.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2106.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2106.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2107.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2107.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2108.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2108.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2109.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2109.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2110.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2110.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2111.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2111.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2112.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2112.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2113.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2113.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2114.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2114.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2115.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2115.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2116.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2116.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2117.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2117.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.fuzz2118.has_value()) {
            auto result = DataCell::from_loggable(archetype.fuzz2118.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = AffixFuzzer4::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
