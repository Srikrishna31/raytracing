load("@rules_rust//rust:defs.bzl", "rust_library")
# Root BUILD file with simplified targets

filegroup(
    name = "all_files",
    srcs = glob(["**/*"]),
    visibility = ["//visibility:public"],
)

# This alias allows building everything with bazel build //:all
alias(
    name = "all",
    actual = "//raytracer:all",
    visibility = ["//visibility:public"],
)
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "cargo_lock",
    srcs = ["Cargo.lock"],
)
