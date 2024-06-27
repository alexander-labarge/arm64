use std::time::Instant;
mod utils;

fn main() {
    // Ensure script is run as root
    utils::check_root::check_root();

    // Start the timer
    let start = Instant::now();

    // Parse command line arguments
    let params = utils::arguments::parse_arguments();

    if params.contains_key("--help") {
        utils::display_help::display_help();
        std::process::exit(0);
    }

    // Run the installer with the provided parameters
    utils::run_installer::run_installer(params);

    // Measure the elapsed time
    let duration = start.elapsed();

    // Print the elapsed time in a readable format
    println!(
        "Installation completed in {} seconds ({} minutes and {} seconds)",
        duration.as_secs(),
        duration.as_secs() / 60,
        duration.as_secs() % 60
    );
}
