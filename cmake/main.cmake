add_subdirectory(rust_part)
target_link_libraries(${PLUGIN_NAME} PRIVATE ${CRATE_NAME})