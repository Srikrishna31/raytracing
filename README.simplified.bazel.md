# Simplified Bazel Setup

This project has been configured with simplified Bazel files that make the code visible to Bazel but don't attempt to build it with Bazel's Rust rules. This approach avoids complex Rust toolchain setup issues.

## Why this approach?

The original Bazel setup encountered issues with the Rust toolchain and linking on Windows. Instead of complex troubleshooting, we've simplified the Bazel configuration to focus on making files available to Bazel rather than building with Bazel.

## Building

To build the project, use Cargo:

```bash
cargo build --release
```

Or for specific examples:

```bash
cargo run --release --example rtweekendone
```

## Using Bazel

If you want to use Bazel, you can try the simplified files:

```bash
cp raytracer/BUILD.simple raytracer/BUILD
cp scenes/BUILD.simple scenes/BUILD
```

Then build with:

```bash
bazel build //:all
```

This will only collect the files, not compile them. Actual compilation should be done with Cargo.

## Full Bazel Integration

If you want to restore full Bazel integration, you'll need to:

1. Ensure your Bazel WORKSPACE is properly configured with rules_rust
2. Fix the Rust toolchain setup for Windows
3. Resolve dependency issues between raytracer and scenes

See the Bazel documentation and rules_rust documentation for more details:
https://bazelbuild.github.io/rules_rust/
