use anyhow::Result;
use prost::Message;
use tensorflow::{Graph, Session, SessionOptions};
use tfprotos_config::tensorflow::gpu_options::experimental::VirtualDevices;
use tfprotos_config::tensorflow::gpu_options::Experimental as ExperimentalGpu;
use tfprotos_config::tensorflow::{ConfigProto, GpuOptions, ThreadPoolOptionProto};

fn main() -> Result<()> {
    let graph_context = Graph::new();
    let mut session_options = SessionOptions::new();
    let example_config = ConfigProto {
        allow_soft_placement: false,
        gpu_options: Some(GpuOptions {
            allow_growth: false,
            experimental: Some(ExperimentalGpu {
                virtual_devices: vec![VirtualDevices {
                    memory_limit_mb: vec![512.0, 512.0],
                    ..Default::default()
                }],
                num_dev_to_dev_copy_streams: 2,
                use_cuda_malloc_async: true,
                timestamped_allocator: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        inter_op_parallelism_threads: 1,
        intra_op_parallelism_threads: 1,
        isolate_session_state: true,
        session_inter_op_thread_pool: vec![ThreadPoolOptionProto {
            num_threads: 16,
            ..Default::default()
        }],
        use_per_session_threads: true,
        ..Default::default()
    };
    let example_config_bytes = example_config.encode_to_vec();
    session_options.set_config(&example_config_bytes)?;
    let session = Session::new(&session_options, &graph_context)?;
    let device_list = session.device_list()?;

    println!("\nApplied config:\n{:#?}", example_config);
    println!("\nDevice List:\n{:#?}", device_list);

    Ok(())
}
