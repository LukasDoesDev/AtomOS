use std::{
    path::{Path, PathBuf},
    process::Command,
};
use std::time::Duration;
use colored::Colorize;
use wait_timeout::ChildExt;

const RUN_ARGS: &[&str] = &["--no-reboot", "-s"];
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

fn main() {
    let mut args = std::env::args_os().skip(1); // skip executable name

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };
    let mut pargs = pico_args::Arguments::from_vec(args.collect());
    let no_boot = pargs.contains("--no-run");
    let hide_qemu = pargs.contains("--hide-qemu");

    println!("{} Creating disk image...", "[BOOT]".bold().blue());
    let bios = create_disk_images(&kernel_binary_path);
    println!("{} Created disk image at `{}`", "[BOOT]".bold().blue(), bios.display());

    if no_boot {
        println!("{} Skipping boot", "[BOOT]".bold().blue());
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", bios.display()));
    run_cmd
        .arg("-serial")
        .arg("stdio");
    run_cmd
        .arg("-device")
        .arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
    if hide_qemu {
        println!("{} Hiding QEMU", "[BOOT]".bold().blue());
        run_cmd
            .arg("-display")
            .arg("none");
    }
    run_cmd.args(RUN_ARGS);

    println!("{} Starting QEMU", "[BOOT]".bold().blue());

    let mut child = run_cmd.spawn().unwrap();

    //let exit_status = run_cmd.status().unwrap();
    let exit_status = match child.wait_timeout(TEST_TIMEOUT).unwrap() {
        Some(status) => status,
        None => {
            println!("{} QEMU ran for too long", "[EXIT]".bold().red());
            child.kill().unwrap();
            child.wait().unwrap()
        }
    };
    if !exit_status.success() {
        match exit_status.code() {
            Some(33) => {
                println!("{} Success", "[EXIT]".bold().blue());
            }
            Some(35) => {
                println!("{} System panic", "[EXIT]".bold().red());
                std::process::exit(35);
            }
            Some(code) => {
                println!("{} Unknown exit: {}", "[EXIT]".bold().red(), code);
                std::process::exit(code);
            }
            None => {
                println!("{} Terminated", "[EXIT]".bold().red())
            }
        }
    }
}

pub fn create_disk_images(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.env("CARGO_BUILD_RUSTFLAGS", "--allow stable_features");
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        panic!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        );
    }
    disk_image
}
