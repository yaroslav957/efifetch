use crate::{
    logic::info::mem::MappedMemoryInfo,
    utils::protocols::{get_resolution, stdout_text_color},
};
use alloc::format;
use uefi::{
    boot::ScopedProtocol,
    print,
    proto::console::text::{Color, Output},
    Result,
};

pub fn draw(mut stdout: &mut ScopedProtocol<Output>) -> Result<()> {
    let mem = MappedMemoryInfo::get()?;
    let columns = get_resolution(&mut stdout)?.1;

    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}", " Mem:", width = columns - 13);
    print!("██║        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!(
        "{:<width$}",
        format!(" Total pages: {}", mem.info.pages.total),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("╚═╝        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!(
        "{:<width$}",
        format!(" Used pages: {}", mem.info.pages.used),
        width = columns - 2
    );
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!(
        "{:<width$}",
        format!(" Physical start: {}", mem.info.phys_addr),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("██╗        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!(
        "{:<width$}",
        format!(" Virtual start: {}", mem.info.virt_addr),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("██║        ");
    print!("│");
    Ok(())
}
