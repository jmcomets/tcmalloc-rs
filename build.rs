extern crate num_cpus;

use std::env;
use std::process::Command;

fn main() {
    let src = env::current_dir().unwrap();
    let gperftools_dir = src.join("vendor/gperftools");

    macro_rules! gperftools_artifact {
        ($file:expr => $cmd:expr) => {
            if !gperftools_dir.join($file).exists() {
                gperftools_cmd!($cmd);
            }
        };

        ($file:expr => $cmd:expr $(, $arg:expr)*) => {
            if !gperftools_dir.join($file).exists() {
                gperftools_cmd!($cmd $(, $arg )*);
            }
        };
    }

    macro_rules! gperftools_cmd {
        ($cmd:expr) => {
            Command::new($cmd)
                .current_dir(&gperftools_dir)
                .status()
                .unwrap();
        };

        ($cmd:expr $(, $arg:expr)*) => {
            Command::new($cmd)
                .current_dir(&gperftools_dir)
                .args(&[$( $arg, )*])
                .status()
                .unwrap();
        };
    }

    gperftools_artifact!(".git"      => "git", "submodule", "update", "--init");
    gperftools_artifact!("configure" => "./autogen.sh");
    gperftools_artifact!("Makefile"  => "./configure", &format!("--prefix={}/dist", gperftools_dir.display()));
    gperftools_artifact!("dist"      => "make", "-j", &format!("{}", num_cpus::get()), "install");

    println!("cargo:rustc-link-search={}/dist/lib", gperftools_dir.display());
}
