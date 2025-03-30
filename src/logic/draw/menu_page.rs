use crate::{
    logic::info::date::Date,
    utils::protocols::{get_resolution, stdout_text_color},
};
use alloc::format;
use uefi::{
    boot::ScopedProtocol,
    print,
    proto::{
        console::text::{Color, Output},
        network::snp::SimpleNetwork,
    },
    Result, Status,
};

const VERSION: &'static str = " Efifetch 0.1.9";
const COLORS: [Color; 14] = [
    Color::LightRed,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightGray,
    Color::LightCyan,
    Color::LightMagenta,
    Color::White,
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::DarkGray,
    Color::Cyan,
    Color::Magenta,
    Color::Black,
];
const LOGO: &'static str = r#"
          .^!??~              
      .:~7?7~:    .:!~:       
    ^7?7~:     :~?5PY7:      .
^!JPGJ.     ~?YPPJ!:     :~?!^
GGGGG5~.    ??7^    .:~7J5GGGG
GP?!~!77!~^.    .:^7JJ7~^::!PG
P~       !YP5YYY5PGJ^       7G
J     :^   !PGGGGP~  :77.^!?YG
J     ~J7.  ^PGGP^   YGPJ?!:7G
P:           ~GG?    7?5:   7G
GJ    7J!^..  YG7    :5.    7G
GGJ:  :7?7??!:JG7    ~Y5    7G
^!YY7:     ^!?PG?    5G5    ^^
   :~!!^:    !GG?    YPJ      
      .:^!?YPGGG?    ^:.      
           ^!JPP!             
"#;

pub fn draw(
    mut stdout: &mut ScopedProtocol<Output>,
    net: &ScopedProtocol<SimpleNetwork>,
    date: Date,
) -> Result<Status> {
    stdout.clear()?;

    let (rows, columns) = get_resolution(&mut stdout)?;
    let mut logo = LOGO.lines().skip(1);

    let resolution = format!(" Resolution: {}x{}", columns, rows);
    let date = format!(" BIOS Date: {}/{}/{}", date.day, date.month, date.year);
    let revision = format!(" UEFI Revision: {}", uefi::system::uefi_revision());
    let firmware_vendor = format!(" Firmware Vendor: {}", uefi::system::firmware_vendor());
    let firmware_revision = format!(" Firmware Revision: {}", uefi::system::firmware_revision());
    let network_state = format!(" Network state: {:?}", net.mode().state);

    assert!(rows >= 24);
    assert!(columns >= 100);

    //              Top frame               //
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("┌{:─<32}┬{:─>right$}┐", "", "", right = columns - 35);

    //              Info bar                //
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("   NET: ");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F1");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<width$}CPU: ", "", width = (columns - 81) / 5);
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F2");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<width$}MEM: ", "", width = (columns - 81) / 5);
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F3");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<width$}PCI: ", "", width = (columns - 81) / 5);
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F4");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<width$}ACPI: ", "", width = (columns - 81) / 5);
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F5");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<width$}HOST: ", "", width = (columns - 81) / 5);
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F6   │");

    print!(
        "│ {} ├{:─<width$}┤",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );

    //      UEFI and runtime inf lines      //
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{VERSION}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - VERSION.len() - 35);

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{resolution}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - resolution.len() - 35);

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{date}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - date.len() - 35);

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{revision}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - revision.len() - 35);

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{firmware_revision}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<width$}│",
        "",
        width = columns - firmware_revision.len() - 35
    );

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{firmware_vendor}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<width$}│",
        "",
        width = columns - firmware_vendor.len() - 35
    );

    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{network_state}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<width$}│",
        "",
        width = columns - network_state.len() - 35
    );

    //             Blank lines              //
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!("│{:<32}│{:>width$}││ ", "", "", width = columns - 35);

    for i in 0..=6 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        print!("██");
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<left_space$}│{:<right_space$}││ ",
        "",
        "",
        left_space = 17,
        right_space = columns - 35
    );

    for i in 7..=13 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        print!("██");
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<left_space$}│{:<right_space$}│",
        "",
        "",
        left_space = 17,
        right_space = columns - 35
    );
    print!(
        "└{:─<32}┴{:─<right_space$}┘",
        "",
        "",
        right_space = columns - 35
    );

    Ok(Status::SUCCESS)
}
