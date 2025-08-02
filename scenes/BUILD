load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "scenes",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = [
        # Removed dependency on raytracer to avoid circular dependency
        # Add direct dependencies that scenes actually needs from raytracer
        "//raytracer:raytracer",
        "@crate_index//:indicatif",
    ],
    data = ["earthmap.jpg"],
    visibility = ["//visibility:public"],
)
