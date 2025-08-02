load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary", "rust_test")

# Note: The build script functionality is disabled due to missing cargo_build_script.bzl
# If you need the slint UI functionality, please ensure rules_rust is properly set up with cargo support

rust_library(
    name = "raytracer",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        # Build script removed due to missing cargo_build_script.bzl
        "@crate_index//:rand",
        "@crate_index//:once_cell",
        "@crate_index//:config",
        "@crate_index//:serde",
        "@crate_index//:serde-aux",
        "@crate_index//:image",
        "@crate_index//:rayon",
        #"@crate_index//:slint",
    ],
    proc_macro_deps = [
        "@crate_index//:embed-doc-image",
    ],
    data = glob(["doc_images/**/*", "examples/slint_ui/**/*.slint"]),
)

[rust_binary(
    name = name,
    srcs = ["examples/" + name + ".rs"],
    deps = [
        ":raytracer",
        "//scenes",
        "@crate_index//:indicatif",
        "@crate_index//:timeit",
    ],
) for name in [
    "checkered_spheres",
    "cornellbox",
    "cornellbox_with_smoke",
    "cornellbox_with_two_boxes",
    "depth_of_field_camera",
    "earth_scene",
    "marble_spheres",
    "rectangle_light_scene",
    "rtnextweek",
    "rtweekendone_moving_spheres_checkered_texture",
    "wide_angle_camera_scene",
]]

rust_binary(
    name = "rtweekendone",
    srcs = glob(["examples/rtweekendone/**/*.rs"]),
    deps = [
        ":raytracer",
        "//scenes",
        "@crate_index//:indicatif",
        "@crate_index//:timeit",
    ],
)

rust_binary(
    name = "slint_ui",
    srcs = glob(["examples/slint_ui/**/*.rs"]),
    deps = [
        ":raytracer",
        "//scenes",
        "@crate_index//:slint",
    ],
    data = glob(["examples/slint_ui/**/*.slint"]),
)

# Add a filegroup to collect all targets for convenience
filegroup(
    name = "all",
    srcs = [
        ":raytracer",
        ":rtweekendone",
        ":slint_ui",
    ] + [":" + name for name in [
        "checkered_spheres",
        "cornellbox",
        "cornellbox_with_smoke",
        "cornellbox_with_two_boxes",
        "depth_of_field_camera",
        "earth_scene",
        "marble_spheres",
        "rectangle_light_scene",
        "rtnextweek",
        "rtweekendone_moving_spheres_checkered_texture",
        "wide_angle_camera_scene",
    ]],
    visibility = ["//visibility:public"],
)
