# Building with Bazel

This project can be built using Bazel, which provides fast, reproducible builds.

## Prerequisites

- Install Bazel: https://bazel.build/install

## Building

```bash
# Debug build
bazel build //...

# Release build
bazel build --config=release //...

# Production build (optimized)
bazel build --config=production //...
```

## Running Examples

```bash
# Run the rtweekendone example
bazel run //raytracer:rtweekendone

# Run the Cornell box example
bazel run //raytracer:cornellbox

# Run with UI
bazel run //raytracer:slint_ui
```

## Development

The project is structured as follows:

- `raytracer/`: Core raytracing library
- `scenes/`: Scene definitions

### Vendoring Dependencies

If you need to work in an offline environment, you can vendor dependencies:

```bash
cargo vendor --versioned-dirs vendor
```

Then Bazel will use the vendored dependencies instead of downloading them.

### Configuration

The `.bazelrc` file contains various build configurations:

- `debug`: Default debug build
- `release`: Optimized build with LTO
- `production`: Fully optimized build for deployment
