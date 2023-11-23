// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#include "affix_fuzzer20.hpp"

#include "primitive_component.hpp"
#include "string_component.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::datatypes {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<datatypes::AffixFuzzer20>::arrow_datatype() {
        static const auto datatype = arrow::struct_({
            arrow::field(
                "p",
                Loggable<rerun::datatypes::PrimitiveComponent>::arrow_datatype(),
                false
            ),
            arrow::field("s", Loggable<rerun::datatypes::StringComponent>::arrow_datatype(), false),
        });
        return datatype;
    }

    rerun::Error Loggable<datatypes::AffixFuzzer20>::fill_arrow_array_builder(
        arrow::StructBuilder* builder, const datatypes::AffixFuzzer20* elements, size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        {
            auto field_builder = static_cast<arrow::UInt32Builder*>(builder->field_builder(0));
            ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                RR_RETURN_NOT_OK(
                    Loggable<rerun::datatypes::PrimitiveComponent>::fill_arrow_array_builder(
                        field_builder,
                        &elements[elem_idx].p,
                        1
                    )
                );
            }
        }
        {
            auto field_builder = static_cast<arrow::StringBuilder*>(builder->field_builder(1));
            ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                RR_RETURN_NOT_OK(
                    Loggable<rerun::datatypes::StringComponent>::fill_arrow_array_builder(
                        field_builder,
                        &elements[elem_idx].s,
                        1
                    )
                );
            }
        }
        ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements), nullptr));

        return Error::ok();
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<datatypes::AffixFuzzer20>::to_arrow(
        const datatypes::AffixFuzzer20* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<datatypes::AffixFuzzer20>::fill_arrow_array_builder(
                static_cast<arrow::StructBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }
} // namespace rerun
