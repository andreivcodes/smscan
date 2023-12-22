use std::process::Command;
fn main() {
    let tailwind_cmd = "pnpm dlx tailwindcss -i ./styles/tailwind.css -o ./assets/main.css";
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(tailwind_cmd).status()
    } else {
        Command::new("sh").arg("-c").arg(tailwind_cmd).status()
    }
    .expect("error running tailwind");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=input.css");
}
