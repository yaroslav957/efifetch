use uefi::proto::console::text::Output;
use uefi::table::boot::ScopedProtocol;

pub fn draw(mut stdout: &mut ScopedProtocol<Output>) {
    // print!("{:^width$}", format!("Cpu brand: {}, Cpu Vendor: {}", cpu.brand.as_str(), cpu.vendor.as_str()), width = columns);
    // let cpu = CpuInfo::get();
}