use wgpu_demo::runv1;

fn main() {
    pollster::block_on(runv1());
    println!("Hello, world!");
}
