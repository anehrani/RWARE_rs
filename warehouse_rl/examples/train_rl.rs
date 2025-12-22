use warehouse_rl::train::train;
use burn::backend::wgpu::Wgpu;
use burn::backend::Autodiff;

fn main() {
    // Check for GPU or use CPU
    let device = burn::backend::wgpu::WgpuDevice::default();
    
    println!("Starting RL training for Warehouse Bot...");
    train::<Autodiff<Wgpu>>(device, 100, 32, 50);
    println!("Training complete.");
}
