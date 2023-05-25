use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    format::Format,
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo, SubpassContents,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Features,
        QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            depth_stencil::DepthStencilState,
            // rasterization::{PolygonMode, RasterizationState},
            tessellation::TessellationState,
            vertex_input::Vertex,
            viewport::{Viewport, ViewportState},
        },
        Pipeline,
        GraphicsPipeline,
        PipelineBindPoint,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{
        acquire_next_image, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary, descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet, allocator::StandardDescriptorSetAllocator},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/vertex.glsl.vert",
    }
}

mod tcs {
    vulkano_shaders::shader! {
        ty: "tess_ctrl",
        path: "src/edge_length_tessellation.glsl.tesc",
    }
}

mod tes {
    vulkano_shaders::shader! {
        ty: "tess_eval",
        path: "src/curves_defined_triangle_tessellation.glsl.tese",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/fragment.glsl.frag",
    }
}

mod mesh;
use mesh::{
    Mesh, VertexData, Edge, Face,
};

fn main() {
    let pi = std::f32::consts::PI;

    let mesh = Mesh {
        vertices: vec![
            VertexData {
                position: [0.0, 0.5, 0.0],
            },
            VertexData {
                position: [0.5*(pi*0.0/3.0).sin(), 0.0, 0.5*(pi*0.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*2.0/3.0).sin(), 0.0, 0.5*(pi*2.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*4.0/3.0).sin(), 0.0, 0.5*(pi*4.0/3.0).cos()],
            },
        ],

        edges: vec![
            Edge{
                padding: [0,0],
                vertices: [0, 1],
                control_points: [
                    [0.2*(pi*0.0/3.0).sin(), 0.3, 0.2*(pi*0.0/3.0).cos(), 1.0],
                    [0.3*(pi*0.0/3.0).sin(), 0.2, 0.3*(pi*0.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [0, 2],
                control_points: [
                    [0.2*(pi*2.0/3.0).sin(), 0.3, 0.2*(pi*2.0/3.0).cos(), 1.0],
                    [0.3*(pi*2.0/3.0).sin(), 0.2, 0.3*(pi*2.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [0, 3],
                control_points: [
                    [0.2*(pi*4.0/3.0).sin(), 0.3, 0.2*(pi*4.0/3.0).cos(), 1.0],
                    [0.3*(pi*4.0/3.0).sin(), 0.2, 0.3*(pi*4.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [1, 2],
                control_points: [
                    [0.35*(pi*0.0/3.0).sin() +0.15*(pi*2.0/3.0).sin(), 0.0, 0.35*(pi*0.0/3.0).cos() +0.15*(pi*2.0/3.0).cos(), 1.0],
                    [0.15*(pi*0.0/3.0).sin() +0.35*(pi*2.0/3.0).sin(), 0.0, 0.15*(pi*0.0/3.0).cos() +0.35*(pi*2.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [2, 3],
                control_points: [
                    [0.35*(pi*2.0/3.0).sin() +0.15*(pi*4.0/3.0).sin(), 0.0, 0.35*(pi*2.0/3.0).cos() +0.15*(pi*4.0/3.0).cos(), 1.0],
                    [0.15*(pi*2.0/3.0).sin() +0.35*(pi*4.0/3.0).sin(), 0.0, 0.15*(pi*2.0/3.0).cos() +0.35*(pi*4.0/3.0).cos(), 1.0],
                ]
            },
            Edge{
                padding: [0,0],
                vertices: [3, 1],
                control_points: [
                    [0.35*(pi*4.0/3.0).sin() +0.15*(pi*0.0/3.0).sin(), 0.0, 0.35*(pi*4.0/3.0).cos() +0.15*(pi*0.0/3.0).cos(), 1.0],
                    [0.15*(pi*4.0/3.0).sin() +0.35*(pi*0.0/3.0).sin(), 0.0, 0.15*(pi*4.0/3.0).cos() +0.35*(pi*0.0/3.0).cos(), 1.0],
                ],
            },
        ],
    
        faces: vec![
            Face{edges: [0, 3, 1]},
            Face{edges: [1, 4, 2]},
            Face{edges: [2, 5, 0]},
            Face{edges: [3, 4, 5]},
        ],
    
        /* can be calculated by faces, edges */
        vertex_indices: vec![
            0,1,2,
            0,2,3,
            0,3,1,
            1,2,3
        ],
    
    };

    let library = VulkanLibrary::new().unwrap();
    let required_extensions = vulkano_win::required_extensions(&library);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            enumerate_portability: true,
            ..Default::default()
        },
    )
    .unwrap();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();


    let (device, mut queues) = {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let features = Features {
            tessellation_shader: true,
            fill_mode_non_solid: true,
            ..Features::empty()
        };
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter(|p| p.supported_features().contains(&features))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                enabled_features: features,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap()
    };
    let queue = queues.next().unwrap();

    let (mut swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        let image_format = Some(
            device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );
        let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

        Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count,
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let descriptor_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let push_constants = {
        let a = 0.1f32;
        vs::PushConstantData {
            view: [
                [-1.0, 0.0, 0.0, 0.0f32],
                [0.0, a.cos(), a.sin(), 0.0f32],
                [0.0, a.sin(), -a.cos(), 0.0f32],
                [0.0, 0.0, 0.5, 1.0f32], 
            ],
        }
    };

    let vertex_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        mesh.vertices,
    ).unwrap();

    let index_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        mesh.vertex_indices,
    ).unwrap();

    let ssbo_edges = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        mesh.edges,
    ).unwrap();

    let ssbo_faces = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        mesh.faces,
    ).unwrap();


    let vs = vs::load(device.clone()).unwrap();
    let tcs = tcs::load(device.clone()).unwrap();
    let tes = tes::load(device.clone()).unwrap();
    let fs = fs::load(device.clone()).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: Format::D16_UNORM,
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {depth},
        },
    )
    .unwrap();

    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(VertexData::per_vertex())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .tessellation_shaders(
            tcs.entry_point("main").unwrap(),
            (),
            tes.entry_point("main").unwrap(),
            (),
        )
        .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::PatchList))
        // .rasterization_state(RasterizationState::new().polygon_mode(PolygonMode::Line))
        .tessellation_state(
            TessellationState::new()
                .patch_control_points(3),
        )
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .depth_stencil_state(DepthStencilState::simple_depth_test())
        .build(device.clone())
        .unwrap();

    let layout = pipeline.layout().set_layouts();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_allocator,
        layout[0].clone(),
        [
            WriteDescriptorSet::buffer(0, ssbo_edges.clone()),
            WriteDescriptorSet::buffer(1, ssbo_faces.clone()),
        ],
    ).unwrap();

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());
    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };
    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport, &memory_allocator);

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
            let dimensions = window.inner_size();
            if dimensions.width == 0 || dimensions.height == 0 {
                return;
            }

            previous_frame_end.as_mut().unwrap().cleanup_finished();

            if recreate_swapchain {
                let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                    image_extent: dimensions.into(),
                    ..swapchain.create_info()
                }) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                    Err(e) => panic!("failed to recreate swapchain: {e}"),
                };

                swapchain = new_swapchain;
                framebuffers =
                    window_size_dependent_setup(&new_images, render_pass.clone(), &mut viewport, &memory_allocator);
                recreate_swapchain = false;
            }

            let (image_index, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let mut builder = AutoCommandBufferBuilder::primary(
                &command_buffer_allocator,
                queue.queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();
            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into()), Some(1.0.into())],
                        ..RenderPassBeginInfo::framebuffer(
                            framebuffers[image_index as usize].clone(),
                        )
                    },
                    SubpassContents::Inline,
                )
                .unwrap()
                .set_viewport(0, [viewport.clone()])
                .bind_pipeline_graphics(pipeline.clone())
                .push_constants(pipeline.layout().clone(), 0, push_constants)
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .bind_index_buffer(index_buffer.clone())
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    pipeline.layout().clone(),
                    0,
                    vec![descriptor_set.clone()]
                )
                .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                .unwrap()
                .end_render_pass()
                .unwrap();
            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                )
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    println!("failed to flush future: {e}");
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }
        _ => (),
    });
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
    memory_allocator: &StandardMemoryAllocator,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];
    let depth_attachment = ImageView::new_default(
        AttachmentImage::with_usage(
            memory_allocator,
            dimensions,
            Format::D16_UNORM,
            ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
        )
        .unwrap(),
    )
    .unwrap();

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_attachment.clone()],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
