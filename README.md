# Tensorflow Protobufs - Config

TBD

## Examples

1. [virtual gpu](examples/virtualgpu/README.md)

## Development

### Minimum Supported Rust Version (MSRV)

The minimum supported rust version is `1.58.1`, can be checked with:

```bash
> make msrv

...

Finished The MSRV is: 1.58.1
```

### Required OS components

Other the [rust compiler](https://www.rust-lang.org/tools/install) the [protoc](https://grpc.io/docs/protoc-installation/) is required.

### Regenerating Protobuf Files

Every build command will regenerate the [tensorflow.rs](src/tensorflow.rs), but the command `make` will regenerate then format the output file. For example:

```bash
> make

...

regenerated protos:
src/protos
└── tensorflow
    └── core
        ├── framework
        │   ├── allocation_description.proto
        │   ├── attr_value.proto
        │   ├── cost_graph.proto
        │   ├── full_type.proto
        │   ├── function.proto
        │   ├── graph.proto
        │   ├── node_def.proto
        │   ├── op_def.proto
        │   ├── resource_handle.proto
        │   ├── step_stats.proto
        │   ├── tensor_description.proto
        │   ├── tensor.proto
        │   ├── tensor_shape.proto
        │   ├── types.proto
        │   └── versions.proto
        └── protobuf
            ├── cluster.proto
            ├── config.proto
            ├── coordination_config.proto
            ├── debug.proto
            ├── rewriter_config.proto
            └── verifier_config.proto

4 directories, 21 files

Generated lines:
2894 src/tensorflow.rs
```

## Licenses

- [Codebase License](LICENSE)
- [Tensorflow License](https://github.com/tensorflow/tensorflow/blob/v2.10.0/LICENSE)
