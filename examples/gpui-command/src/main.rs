#[cfg(not(target_family = "wasm"))]
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--alert") {
        justdo_command::alert::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--alert-dialog") {
        justdo_command::alert_dialog::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--bubble") {
        justdo_command::bubble::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--breadcrumb") {
        justdo_command::breadcrumb::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--badge") {
        justdo_command::badge::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--avatar") {
        justdo_command::avatar::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--attachment") {
        justdo_command::attachment::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--aspect-ratio") {
        justdo_command::aspect_ratio::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--accordion") {
        justdo_command::accordion::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("default"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--tabs") {
        justdo_command::motion_tabs::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("pill"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--card") {
        justdo_command::card::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("basic"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--calendar") {
        justdo_command::calendar::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("basic"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--button-group") {
        justdo_command::button_group::run_native(
            args.get(index + 1).map(String::as_str).unwrap_or("basic"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--button") {
        justdo_command::button::run_native(
            args.get(index + 1)
                .map(String::as_str)
                .unwrap_or("variants"),
        );
    } else if let Some(index) = args.iter().position(|arg| arg == "--metallic") {
        justdo_command::motion_button::run_native(
            args.get(index + 1)
                .map(String::as_str)
                .unwrap_or("metallic"),
        );
    } else if args.iter().any(|arg| arg == "--command-menu") {
        justdo_command::run_native();
    } else {
        justdo_command::action_swap::run_native();
    }
}
#[cfg(target_family = "wasm")]
fn main() {}
