workspace(name = "raytracing")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Rust rules
http_archive(
    name = "rules_rust",
    sha256 = "c30dfdf1e86fd50650a76ea645b3a45f2f00667b06187a685e9554e167ca97ee",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.40.0/rules_rust-v0.40.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = ["1.84.1"],
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    manifests = [
        "//:Cargo.toml",
        "//raytracer:Cargo.toml",
        "//scenes:Cargo.toml",
    ],
    # Disable usage of the Cargo config files completely
    generate_binaries = True,
    isolated = True,
    supported_platform_triples = [
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "aarch64-apple-darwin",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()
