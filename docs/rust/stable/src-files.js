var srcIndex = new Map(JSON.parse('[\
["2d",["",[],["2d.rs","framework.rs"]]],\
["build_re_types",["",[],["build_re_types.rs"]]],\
["clamped_zip",["",[],["clamped_zip.rs"]]],\
["clock",["",[],["main.rs"]]],\
["custom_data_loader",["",[],["main.rs"]]],\
["custom_space_view",["",[],["color_coordinates_space_view.rs","color_coordinates_visualizer_system.rs","main.rs"]]],\
["custom_store_subscriber",["",[],["main.rs"]]],\
["depth_cloud",["",[],["depth_cloud.rs","framework.rs"]]],\
["depth_offset",["",[],["depth_offset.rs","framework.rs"]]],\
["dna",["",[],["main.rs"]]],\
["extend_viewer_ui",["",[],["main.rs"]]],\
["incremental_logging",["",[],["main.rs"]]],\
["log_benchmark",["",[],["image.rs","main.rs","points3d_large_batch.rs","points3d_many_individual.rs","points3d_shared.rs"]]],\
["log_file",["",[],["main.rs"]]],\
["minimal",["",[],["main.rs"]]],\
["minimal_options",["",[],["main.rs"]]],\
["minimal_serve",["",[],["main.rs"]]],\
["multiview",["",[],["framework.rs","multiview.rs"]]],\
["objectron",["",[],["main.rs","objectron.rs"]]],\
["outlines",["",[],["framework.rs","outlines.rs"]]],\
["picking",["",[],["framework.rs","picking.rs"]]],\
["plot_dashboard_stress",["",[],["main.rs"]]],\
["range_zip",["",[],["range_zip.rs"]]],\
["raw_mesh",["",[],["main.rs"]]],\
["re_analytics",["",[["native",[],["config.rs","mod.rs","pipeline.rs","sink.rs"]]],["cli.rs","event.rs","lib.rs","posthog.rs"]]],\
["re_blueprint_tree",["",[],["blueprint_tree.rs","lib.rs"]]],\
["re_build_info",["",[],["build_info.rs","crate_version.rs","lib.rs"]]],\
["re_build_tools",["",[],["git.rs","hashing.rs","lib.rs","rebuild_detector.rs"]]],\
["re_case",["",[],["lib.rs"]]],\
["re_chunk",["",[],["batcher.rs","chunk.rs","lib.rs","shuffle.rs","transport.rs","util.rs"]]],\
["re_context_menu",["",[["actions",[],["add_container.rs","add_entities_to_new_space_view.rs","add_space_view.rs","clone_space_view.rs","collapse_expand_all.rs","mod.rs","move_contents_to_new_container.rs","remove.rs","show_hide.rs"]]],["lib.rs","sub_menu.rs"]]],\
["re_crash_handler",["",[],["lib.rs"]]],\
["re_data_loader",["",[],["lib.rs","load_file.rs","loader_archetype.rs","loader_directory.rs","loader_external.rs","loader_rrd.rs"]]],\
["re_data_source",["",[],["data_source.rs","lib.rs","load_stdin.rs","web_sockets.rs"]]],\
["re_data_store",["",[],["arrow_util.rs","lib.rs","store.rs","store_arrow.rs","store_dump.rs","store_event.rs","store_format.rs","store_gc.rs","store_helpers.rs","store_read.rs","store_sanity.rs","store_stats.rs","store_subscriber.rs","store_write.rs","test_util.rs"]]],\
["re_data_ui",["",[],["annotation_context.rs","app_id.rs","blueprint_data.rs","blueprint_types.rs","component.rs","component_name.rs","component_path.rs","component_ui_registry.rs","data.rs","data_source.rs","entity_db.rs","entity_path.rs","image.rs","image_meaning.rs","instance_path.rs","item_ui.rs","lib.rs","log_msg.rs","pinhole.rs","rotation3d.rs","store_id.rs","transform3d.rs"]]],\
["re_dev_tools",["",[["build_examples",[],["example.rs","install.rs","manifest.rs","mod.rs","rrd.rs","snippets.rs","wait_for_output.rs"]],["build_search_index",[["ingest",[],["cpp.rs","docs.rs","examples.rs","python.rs","rust.rs"]]],["build.rs","ingest.rs","meili.rs","mod.rs","repl.rs","util.rs"]],["build_web_viewer",[],["lib.rs","mod.rs"]]],["main.rs"]]],\
["re_edit_ui",["",[["datatype_editors",[],["bool_toggle.rs","enum_combobox.rs","float_drag.rs","mod.rs","singleline_string.rs"]]],["color.rs","lib.rs","marker_shape.rs","material.rs","radius.rs","range1d.rs","response_utils.rs","visual_bounds2d.rs"]]],\
["re_entity_db",["",[],["entity_db.rs","entity_tree.rs","instance_path.rs","lib.rs","store_bundle.rs","time_histogram_per_timeline.rs","times_per_timeline.rs","versioned_instance_path.rs"]]],\
["re_error",["",[],["lib.rs"]]],\
["re_format",["",[],["lib.rs","time.rs"]]],\
["re_format_arrow",["",[],["lib.rs"]]],\
["re_int_histogram",["",[],["lib.rs","tree.rs"]]],\
["re_log",["",[],["channel_logger.rs","lib.rs","multi_logger.rs","result_extensions.rs","setup.rs"]]],\
["re_log_encoding",["",[["decoder",[],["mod.rs","stream.rs"]]],["encoder.rs","file_sink.rs","lib.rs","stream_rrd_from_http.rs"]]],\
["re_log_types",["",[["path",[],["component_path.rs","data_path.rs","entity_path.rs","entity_path_filter.rs","entity_path_part.rs","mod.rs","natural_ordering.rs","parse_path.rs"]],["time_point",[],["mod.rs","non_min_i64.rs","time_int.rs","timeline.rs"]]],["arrow_msg.rs","data_cell.rs","data_row.rs","data_table.rs","example_components.rs","hash.rs","instance.rs","lib.rs","resolved_time_range.rs","time.rs","time_real.rs","vec_deque_ext.rs"]]],\
["re_memory",["",[],["accounting_allocator.rs","allocation_tracker.rs","backtrace_native.rs","lib.rs","memory_history.rs","memory_limit.rs","memory_use.rs","ram_warner.rs","util.rs"]]],\
["re_query",["",[["clamped_zip",[],["generated.rs","mod.rs"]],["latest_at",[],["helpers.rs","mod.rs","query.rs","results.rs"]],["range",[],["mod.rs","query.rs","results.rs"]],["range_zip",[],["generated.rs","mod.rs"]]],["cache.rs","cache_stats.rs","flat_vec_deque.rs","lib.rs","promise.rs"]]],\
["re_renderer",["",[["allocator",[],["cpu_write_gpu_read_belt.rs","data_texture_source.rs","gpu_readback_belt.rs","mod.rs","uniform_buffer_fill.rs"]],["draw_phases",[],["mod.rs","outlines.rs","picking_layer.rs","screenshot.rs"]],["error_handling",[],["error_tracker.rs","mod.rs","now_or_never.rs","wgpu_core_error.rs","wgpu_error_scope.rs"]],["importer",[],["gltf.rs","mod.rs","obj.rs","stl.rs"]],["renderer",[],["compositor.rs","debug_overlay.rs","depth_cloud.rs","generic_skybox.rs","lines.rs","mesh_renderer.rs","mod.rs","point_cloud.rs","rectangles.rs","test_triangle.rs"]],["resource_managers",[],["mesh_manager.rs","mod.rs","resource_manager.rs","texture_manager.rs"]],["wgpu_resources",[],["bind_group_layout_pool.rs","bind_group_pool.rs","buffer_pool.rs","dynamic_resource_pool.rs","mod.rs","pipeline_layout_pool.rs","render_pipeline_pool.rs","resource.rs","sampler_pool.rs","shader_module_pool.rs","static_resource_pool.rs","texture_pool.rs"]]],["color.rs","colormap.rs","config.rs","context.rs","debug_label.rs","depth_offset.rs","file_resolver.rs","file_server.rs","file_system.rs","global_bindings.rs","lib.rs","line_drawable_builder.rs","mesh.rs","point_cloud_builder.rs","queueable_draw_data.rs","rect.rs","size.rs","texture_info.rs","transform.rs","view_builder.rs","wgpu_buffer_types.rs"]]],\
["re_sdk",["",[],["binary_stream_sink.rs","global.rs","lib.rs","log_sink.rs","recording_stream.rs","spawn.rs","web_viewer.rs"]]],\
["re_sdk_comms",["",[],["buffered_client.rs","lib.rs","server.rs","tcp_client.rs"]]],\
["re_selection_panel",["",[],["defaults_ui.rs","lib.rs","selection_history_ui.rs","selection_panel.rs","space_view_entity_picker.rs","space_view_space_origin_ui.rs","visible_time_range_ui.rs","visualizer_ui.rs"]]],\
["re_smart_channel",["",[],["lib.rs","receive_set.rs","receiver.rs","sender.rs"]]],\
["re_space_view",["",[],["controls.rs","heuristics.rs","lib.rs","query.rs","results_ext.rs","screenshot.rs","view_property_ui.rs"]]],\
["re_space_view_bar_chart",["",[],["lib.rs","space_view_class.rs","visualizer_system.rs"]]],\
["re_space_view_dataframe",["",[],["lib.rs","space_view_class.rs","visualizer_system.rs"]]],\
["re_space_view_spatial",["",[["contexts",[],["annotation_context.rs","depth_offsets.rs","mod.rs","transform_context.rs"]],["visualizers",[["utilities",[],["entity_iterator.rs","labels.rs","mod.rs","spatial_view_visualizer.rs","textured_rect.rs"]]],["arrows2d.rs","arrows3d.rs","assets3d.rs","boxes2d.rs","boxes3d.rs","cameras.rs","depth_images.rs","images.rs","lines2d.rs","lines3d.rs","meshes.rs","mod.rs","points2d.rs","points3d.rs","segmentation_images.rs","transform3d_arrows.rs"]]],["eye.rs","heuristics.rs","instance_hash_conversions.rs","lib.rs","max_image_dimension_subscriber.rs","mesh_cache.rs","mesh_loader.rs","pickable_image.rs","picking.rs","scene_bounding_boxes.rs","space_camera_3d.rs","spatial_topology.rs","ui.rs","ui_2d.rs","ui_3d.rs","view_2d.rs","view_2d_properties.rs","view_3d.rs","view_3d_properties.rs"]]],\
["re_space_view_tensor",["",[],["dimension_mapping.rs","lib.rs","space_view_class.rs","tensor_dimension_mapper.rs","tensor_slice_to_gpu.rs","visualizer_system.rs"]]],\
["re_space_view_text_document",["",[],["lib.rs","space_view_class.rs","visualizer_system.rs"]]],\
["re_space_view_text_log",["",[],["lib.rs","space_view_class.rs","visualizer_system.rs"]]],\
["re_space_view_time_series",["",[],["aggregation.rs","lib.rs","line_visualizer_system.rs","point_visualizer_system.rs","space_view_class.rs","util.rs"]]],\
["re_string_interner",["",[],["lib.rs"]]],\
["re_time_panel",["",[],["data_density_graph.rs","lib.rs","paint_ticks.rs","time_axis.rs","time_control_ui.rs","time_ranges_ui.rs","time_selection_ui.rs"]]],\
["re_tracing",["",[],["lib.rs","server.rs"]]],\
["re_tuid",["",[],["lib.rs"]]],\
["re_types",["",[["archetypes",[],["annotation_context.rs","arrows2d.rs","arrows2d_ext.rs","arrows3d.rs","arrows3d_ext.rs","asset3d.rs","asset3d_ext.rs","bar_chart.rs","boxes2d.rs","boxes2d_ext.rs","boxes3d.rs","boxes3d_ext.rs","depth_image.rs","depth_image_ext.rs","disconnected_space.rs","image.rs","image_ext.rs","line_strips2d.rs","line_strips3d.rs","mesh3d.rs","mesh3d_ext.rs","mod.rs","pinhole.rs","pinhole_ext.rs","points2d.rs","points3d.rs","points3d_ext.rs","scalar.rs","segmentation_image.rs","segmentation_image_ext.rs","series_line.rs","series_point.rs","tensor.rs","tensor_ext.rs","text_document.rs","text_document_ext.rs","text_log.rs","transform3d.rs","transform3d_ext.rs","view_coordinates.rs","view_coordinates_ext.rs"]],["blueprint",[["archetypes",[],["background.rs","mod.rs","plot_legend.rs","scalar_axis.rs","space_view_blueprint.rs","space_view_contents.rs","tensor_scalar_mapping.rs","tensor_slice_selection.rs","tensor_view_fit.rs","visible_time_ranges.rs","visible_time_ranges_ext.rs","visual_bounds2d.rs"]],["components",[],["active_tab.rs","background_kind.rs","column_share.rs","corner2d.rs","corner2d_ext.rs","included_content.rs","interactive.rs","interactive_ext.rs","lock_range_during_zoom.rs","mod.rs","panel_state.rs","panel_state_ext.rs","query_expression.rs","row_share.rs","space_view_class.rs","space_view_class_ext.rs","space_view_origin.rs","space_view_origin_ext.rs","tensor_dimension_index_slider.rs","tensor_dimension_index_slider_ext.rs","view_fit.rs","viewer_recommendation_hash.rs","viewer_recommendation_hash_ext.rs","visible.rs","visible_ext.rs","visible_time_range.rs","visual_bounds2d.rs","visual_bounds2d_ext.rs"]],["datatypes",[],["mod.rs","tensor_dimension_index_slider.rs"]],["views",[],["bar_chart_view.rs","mod.rs","spatial2d_view.rs","spatial3d_view.rs","tensor_view.rs","text_document_view.rs","text_log_view.rs","time_series_view.rs"]]],["mod.rs"]],["components",[],["aggregation_policy.rs","annotation_context.rs","axis_length.rs","axis_length_ext.rs","blob.rs","blob_ext.rs","class_id.rs","class_id_ext.rs","color.rs","color_ext.rs","colormap.rs","depth_meter.rs","depth_meter_ext.rs","disconnected_space.rs","disconnected_space_ext.rs","draw_order.rs","draw_order_ext.rs","fill_ratio.rs","fill_ratio_ext.rs","gamma_correction.rs","gamma_correction_ext.rs","half_size2d.rs","half_size2d_ext.rs","half_size3d.rs","half_size3d_ext.rs","image_plane_distance.rs","image_plane_distance_ext.rs","keypoint_id.rs","keypoint_id_ext.rs","line_strip2d.rs","line_strip2d_ext.rs","line_strip3d.rs","line_strip3d_ext.rs","magnification_filter.rs","marker_shape.rs","marker_shape_ext.rs","marker_size.rs","marker_size_ext.rs","material.rs","material_ext.rs","media_type.rs","media_type_ext.rs","mod.rs","name.rs","name_ext.rs","opacity.rs","opacity_ext.rs","out_of_tree_transform3d.rs","pinhole_projection.rs","pinhole_projection_ext.rs","position2d.rs","position2d_ext.rs","position3d.rs","position3d_ext.rs","radius.rs","radius_ext.rs","range1d.rs","range1d_ext.rs","resolution.rs","resolution_ext.rs","rotation3d.rs","rotation3d_ext.rs","scalar.rs","scalar_ext.rs","stroke_width.rs","stroke_width_ext.rs","tensor_data.rs","tensor_dimension_index_selection.rs","tensor_dimension_index_selection_ext.rs","tensor_height_dimension.rs","tensor_width_dimension.rs","texcoord2d.rs","texcoord2d_ext.rs","text.rs","text_ext.rs","text_log_level.rs","text_log_level_ext.rs","transform3d.rs","transform3d_ext.rs","triangle_indices.rs","triangle_indices_ext.rs","vector2d.rs","vector2d_ext.rs","vector3d.rs","vector3d_ext.rs","view_coordinates.rs","view_coordinates_ext.rs"]],["datatypes",[],["angle.rs","angle_ext.rs","annotation_info.rs","annotation_info_ext.rs","bool.rs","bool_ext.rs","class_description.rs","class_description_ext.rs","class_description_map_elem.rs","class_description_map_elem_ext.rs","class_id.rs","class_id_ext.rs","keypoint_id.rs","keypoint_id_ext.rs","keypoint_pair.rs","keypoint_pair_ext.rs","mat3x3.rs","mat3x3_ext.rs","mat4x4.rs","mat4x4_ext.rs","material.rs","material_ext.rs","mod.rs","quaternion.rs","quaternion_ext.rs","range1d.rs","range1d_ext.rs","range2d.rs","range2d_ext.rs","rgba32.rs","rgba32_ext.rs","rotation3d.rs","rotation3d_ext.rs","rotation_axis_angle.rs","rotation_axis_angle_ext.rs","scale3d.rs","scale3d_ext.rs","tensor_buffer.rs","tensor_buffer_ext.rs","tensor_data.rs","tensor_data_ext.rs","tensor_dimension.rs","tensor_dimension_ext.rs","tensor_dimension_index_selection.rs","tensor_dimension_selection.rs","tensor_dimension_selection_ext.rs","transform3d.rs","transform3d_ext.rs","translation_and_mat3x3.rs","translation_and_mat3x3_ext.rs","translation_rotation_scale3d.rs","translation_rotation_scale3d_ext.rs","uuid.rs","uuid_ext.rs","uvec2d.rs","uvec2d_ext.rs","uvec3d.rs","uvec3d_ext.rs","uvec4d.rs","uvec4d_ext.rs","vec2d.rs","vec2d_ext.rs","vec3d.rs","vec3d_ext.rs","vec4d.rs","vec4d_ext.rs"]],["testing",[["archetypes",[],["affix_fuzzer1.rs","affix_fuzzer2.rs","affix_fuzzer3.rs","affix_fuzzer4.rs","mod.rs"]],["components",[],["affix_fuzzer1.rs","affix_fuzzer10.rs","affix_fuzzer11.rs","affix_fuzzer12.rs","affix_fuzzer13.rs","affix_fuzzer14.rs","affix_fuzzer15.rs","affix_fuzzer16.rs","affix_fuzzer17.rs","affix_fuzzer18.rs","affix_fuzzer19.rs","affix_fuzzer2.rs","affix_fuzzer20.rs","affix_fuzzer21.rs","affix_fuzzer22.rs","affix_fuzzer3.rs","affix_fuzzer4.rs","affix_fuzzer5.rs","affix_fuzzer6.rs","affix_fuzzer7.rs","affix_fuzzer8.rs","affix_fuzzer9.rs","enum_test.rs","mod.rs"]],["datatypes",[],["affix_fuzzer1.rs","affix_fuzzer2.rs","affix_fuzzer20.rs","affix_fuzzer21.rs","affix_fuzzer22.rs","affix_fuzzer3.rs","affix_fuzzer3_ext.rs","affix_fuzzer4.rs","affix_fuzzer4_ext.rs","affix_fuzzer5.rs","flattened_scalar.rs","mod.rs","primitive_component.rs","string_component.rs"]]],["mod.rs"]]],["image.rs","lib.rs","tensor_data.rs","view_coordinates.rs"]]],\
["re_types_blueprint",["",[["blueprint",[["archetypes",[],["container_blueprint.rs","mod.rs","panel_blueprint.rs","viewport_blueprint.rs"]],["components",[],["auto_layout.rs","auto_layout_ext.rs","auto_space_views.rs","container_kind.rs","grid_columns.rs","included_space_view.rs","mod.rs","root_container.rs","space_view_maximized.rs","visualizer_overrides.rs"]]],["mod.rs"]]],["lib.rs"]]],\
["re_types_builder",["",[["codegen",[["cpp",[],["array_builder.rs","forward_decl.rs","includes.rs","method.rs","mod.rs"]],["docs",[],["mod.rs"]],["python",[],["mod.rs","views.rs"]],["rust",[],["api.rs","arrow.rs","blueprint_validation.rs","deserializer.rs","mod.rs","reflection.rs","serializer.rs","util.rs"]]],["common.rs","mod.rs"]],["format",[],["cpp.rs","mod.rs","python.rs","rust.rs"]]],["arrow_registry.rs","docs.rs","lib.rs","objects.rs","reflection.rs","report.rs"]]],\
["re_types_core",["",[["archetypes",[],["clear.rs","clear_ext.rs","mod.rs"]],["components",[],["clear_is_recursive.rs","clear_is_recursive_ext.rs","mod.rs"]],["datatypes",[],["entity_path.rs","float32.rs","float32_ext.rs","mod.rs","time_int.rs","time_int_ext.rs","time_range.rs","time_range_boundary.rs","time_range_boundary_ext.rs","time_range_ext.rs","uint32.rs","uint64.rs","utf8.rs","utf8_ext.rs","visible_time_range.rs","visible_time_range_ext.rs"]]],["archetype.rs","arrow_buffer.rs","arrow_string.rs","lib.rs","loggable.rs","loggable_batch.rs","macros.rs","reflection.rs","result.rs","size_bytes.rs","tuid.rs","view.rs"]]],\
["re_ui",["",[["list_item",[],["button_content.rs","item_button.rs","label_content.rs","list_item.rs","mod.rs","other_contents.rs","property_content.rs","scope.rs"]]],["command.rs","command_palette.rs","context_ext.rs","design_tokens.rs","drag_and_drop.rs","icons.rs","layout_job_builder.rs","lib.rs","modal.rs","section_collapsing_header.rs","syntax_highlighting.rs","toasts.rs","ui_ext.rs"]]],\
["re_viewer",["",[["blueprint",[["validation_gen",[],["mod.rs"]]],["mod.rs","validation.rs"]],["reflection",[],["mod.rs"]],["ui",[["welcome_screen",[],["example_section.rs","mod.rs","no_data_ui.rs","welcome_section.rs"]]],["memory_panel.rs","mobile_warning_ui.rs","mod.rs","recordings_panel.rs","rerun_menu.rs","top_panel.rs"]],["viewer_analytics",[],["event.rs","mod.rs"]]],["app.rs","app_blueprint.rs","app_state.rs","background_tasks.rs","env_vars.rs","lib.rs","loading.rs","native.rs","saving.rs","screenshotter.rs"]]],\
["re_viewer_context",["",[["gpu_bridge",[],["colormap.rs","mod.rs","re_renderer_callback.rs","tensor_to_gpu.rs"]],["space_view",[],["highlights.rs","mod.rs","named_system.rs","space_view_class.rs","space_view_class_placeholder.rs","space_view_class_registry.rs","spawn_heuristics.rs","system_execution_output.rs","view_context.rs","view_context_system.rs","view_query.rs","view_states.rs","visualizer_entity_subscriber.rs","visualizer_system.rs"]],["tensor",[],["mod.rs","tensor_decode_cache.rs","tensor_stats.rs","tensor_stats_cache.rs"]],["utils",[],["color.rs","mod.rs","text.rs"]]],["annotations.rs","app_options.rs","blueprint_helpers.rs","blueprint_id.rs","caches.rs","clipboard.rs","collapsed_id.rs","command_sender.rs","component_fallbacks.rs","component_ui_registry.rs","contents.rs","item.rs","lib.rs","query_context.rs","query_range.rs","selection_history.rs","selection_state.rs","store_context.rs","store_hub.rs","test_context.rs","time_control.rs","typed_entity_collections.rs","viewer_context.rs"]]],\
["re_viewport",["",[],["auto_layout.rs","lib.rs","screenshot.rs","space_view_highlights.rs","system_execution.rs","viewport.rs"]]],\
["re_viewport_blueprint",["",[["ui",[],["add_space_view_or_container_modal.rs","mod.rs"]]],["container.rs","lib.rs","space_view.rs","space_view_contents.rs","tree_actions.rs","view_properties.rs","viewport_blueprint.rs"]]],\
["re_web_viewer_server",["",[],["lib.rs"]]],\
["re_ws_comms",["",[],["client.rs","lib.rs","server.rs"]]],\
["rerun",["",[],["clap.rs","demo_util.rs","lib.rs","log_integration.rs","native_viewer.rs","run.rs","sdk.rs"]]],\
["rerun_bindings",["",[],["arrow.rs","lib.rs","python_bridge.rs"]]],\
["rerun_c",["",[],["component_type_registry.rs","error.rs","lib.rs","ptr.rs","recording_streams.rs"]]],\
["rerun_loader_rust_file",["",[],["main.rs"]]],\
["roundtrip_annotation_context",["",[],["main.rs"]]],\
["roundtrip_arrows2d",["",[],["main.rs"]]],\
["roundtrip_arrows3d",["",[],["main.rs"]]],\
["roundtrip_boxes2d",["",[],["main.rs"]]],\
["roundtrip_boxes3d",["",[],["main.rs"]]],\
["roundtrip_depth_image",["",[],["main.rs"]]],\
["roundtrip_disconnected_space",["",[],["main.rs"]]],\
["roundtrip_image",["",[],["main.rs"]]],\
["roundtrip_line_strips2d",["",[],["main.rs"]]],\
["roundtrip_line_strips3d",["",[],["main.rs"]]],\
["roundtrip_pinhole",["",[],["main.rs"]]],\
["roundtrip_points2d",["",[],["main.rs"]]],\
["roundtrip_points3d",["",[],["main.rs"]]],\
["roundtrip_segmentation_image",["",[],["main.rs"]]],\
["roundtrip_tensor",["",[],["main.rs"]]],\
["roundtrip_text_document",["",[],["main.rs"]]],\
["roundtrip_text_log",["",[],["main.rs"]]],\
["roundtrip_transform3d",["",[],["main.rs"]]],\
["roundtrip_view_coordinates",["",[],["main.rs"]]],\
["roundtrip_visible_time_ranges",["",[],["main.rs"]]],\
["run_wasm",["",[],["main.rs"]]],\
["shared_recording",["",[],["main.rs"]]],\
["snippets",["",[["snippets",[],["annotation_context_connections.rs","annotation_context_rects.rs","annotation_context_segmentation.rs","arrow2d_simple.rs","arrow3d_simple.rs","asset3d_out_of_tree.rs","asset3d_simple.rs","bar_chart.rs","box2d_simple.rs","box3d_batch.rs","box3d_simple.rs","clear_recursive.rs","clear_simple.rs","custom_data.rs","depth_image_3d.rs","depth_image_simple.rs","disconnected_space.rs","entity_path.rs","image_simple.rs","line_segments2d_simple.rs","line_segments3d_simple.rs","line_strip2d_batch.rs","line_strip2d_simple.rs","line_strip2d_ui_radius.rs","line_strip3d_batch.rs","line_strip3d_simple.rs","line_strip3d_ui_radius.rs","manual_indicator.rs","mesh3d_indexed.rs","mesh3d_partial_updates.rs","mesh3d_simple.rs","mod.rs","pinhole_perspective.rs","pinhole_simple.rs","point2d_random.rs","point2d_simple.rs","point2d_ui_radius.rs","point3d_random.rs","point3d_simple.rs","point3d_ui_radius.rs","quick_start_connect.rs","quick_start_spawn.rs","scalar_multiple_plots.rs","scalar_simple.rs","segmentation_image_simple.rs","series_line_style.rs","series_point_style.rs","tensor_simple.rs","text_document.rs","text_log.rs","text_log_integration.rs","transform3d_axes.rs","transform3d_simple.rs","view_coordinates_simple.rs"]]],["lib.rs"]]],\
["spawn_viewer",["",[],["main.rs"]]],\
["stdio",["",[],["main.rs"]]],\
["template",["",[],["main.rs"]]],\
["test_api",["",[],["main.rs"]]],\
["test_image_memory",["",[],["main.rs"]]],\
["test_pinhole_projection",["",[],["main.rs"]]]\
]'));
createSrcSidebar();
