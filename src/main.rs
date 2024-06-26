mod utils;

fn main() {
    // Ensure script is run as root
    utils::check_root::check_root();

    // Parse command line arguments
    let params = utils::arguments::parse_arguments();

    if params.contains_key("--help") {
        utils::display_help::display_help();
        return;
    }

    utils::run_installer::run_installer(params);
}
