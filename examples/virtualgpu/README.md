# Example - Virtual GPU

## Run

* Command:

```bash
> make example-virtualgpu 
```

* Result:

```bash
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

...

Applied config:
ConfigProto {
    device_count: {},
    intra_op_parallelism_threads: 1,
    inter_op_parallelism_threads: 1,
    use_per_session_threads: true,
    session_inter_op_thread_pool: [
        ThreadPoolOptionProto {
            num_threads: 16,
            global_name: "",
        },
    ],
    placement_period: 0,
    device_filters: [],
    gpu_options: Some(
        GpuOptions {
            per_process_gpu_memory_fraction: 0.0,
            allow_growth: false,
            allocator_type: "",
            deferred_deletion_bytes: 0,
            visible_device_list: "",
            polling_active_delay_usecs: 0,
            polling_inactive_delay_msecs: 0,
            force_gpu_compatible: false,
            experimental: Some(
                Experimental {
                    virtual_devices: [
                        VirtualDevices {
                            memory_limit_mb: [
                                512.0,
                                512.0,
                            ],
                            priority: [],
                            device_ordinal: [],
                        },
                    ],
                    use_unified_memory: false,
                    num_dev_to_dev_copy_streams: 2,
                    collective_ring_order: "",
                    timestamped_allocator: true,
                    kernel_tracker_max_interval: 0,
                    kernel_tracker_max_bytes: 0,
                    kernel_tracker_max_pending: 0,
                    internal_fragmentation_fraction: 0.0,
                    use_cuda_malloc_async: true,
                    disallow_retry_on_allocation_failure: false,
                },
            ),
        },
    ),
    allow_soft_placement: false,
    log_device_placement: false,
    graph_options: None,
    operation_timeout_in_ms: 0,
    rpc_options: None,
    cluster_def: None,
    isolate_session_state: true,
    share_cluster_devices_in_session: false,
    experimental: None,
}

Device List:
[
    Device {
        name: "/job:localhost/replica:0/task:0/device:CPU:0",
        device_type: "CPU",
        memory_bytes: 268435456,
        incarnation: 17982606704470137112,
    },
    Device {
        name: "/job:localhost/replica:0/task:0/device:GPU:0",
        device_type: "GPU",
        memory_bytes: 536870912,
        incarnation: 14744079129675543878,
    },
    Device {
        name: "/job:localhost/replica:0/task:0/device:GPU:1",
        device_type: "GPU",
        memory_bytes: 536870912,
        incarnation: 12051077205799003133,
    },
]
```
