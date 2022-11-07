# Tensorflow Protobufs - Config

TBD

## Development

### Regenerating Protobuf Files

```bash
> make regenerate

...

regenerated protos:
tensorflow
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

3 directories, 21 files
```

## Licenses

- [Codebase License](LICENSE)
- [Tensorflow License](https://github.com/tensorflow/tensorflow/blob/v2.10.0/LICENSE)
