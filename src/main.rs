use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::device::{DeviceExtensions, Features, Device, physical::PhysicalDevice};
use vulkano::command_buffer::{PrimaryCommandBuffer, AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::image::{AttachmentImage, ImageUsage};
use vulkano::format::{Format, ClearValue};
use vulkano::sync::GpuFuture;
use vulkano::Version;

fn main() {
	let app_infos = vulkano::app_info_from_cargo_toml!();
	let instance = Instance::new(Some(&app_infos), Version::V1_2, &InstanceExtensions::none(), Some("VK_LAYER_KHRONOS_validation")).unwrap();
	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
	let queue_family = physical.queue_families().next().unwrap();
	let (device, mut queues) = Device::new(physical, &Features::none(), &DeviceExtensions::none(), Some((queue_family, 1.0))).unwrap();
	let queue = queues.next().unwrap();
	
	let image_usage = ImageUsage {
		transfer_source: true,
		transfer_destination: true,
		..ImageUsage::none()
	};
	
	let image = AttachmentImage::with_usage(device.clone(), [256, 256], Format::R8G8B8A8Unorm, image_usage).unwrap();
	
	let mut builder = AutoCommandBufferBuilder::primary(device.clone(), queue_family, CommandBufferUsage::OneTimeSubmit).unwrap();
	
	builder.clear_color_image(image.clone(), ClearValue::Float([1.0, 1.0, 1.0, 1.0])).unwrap();
	
	let future = builder.build().unwrap()
	                    .execute(queue.clone()).unwrap();
	
	future.flush().unwrap();
	
	let mut builder = AutoCommandBufferBuilder::primary(device.clone(), queue_family, CommandBufferUsage::OneTimeSubmit).unwrap();
	
	builder.clear_color_image(image.clone(), ClearValue::Float([1.0, 1.0, 1.0, 1.0])).unwrap();
	
	let future = future.then_execute(queue, builder.build().unwrap()).unwrap();
	
	future.flush().unwrap();
}
