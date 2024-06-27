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

    // Generate SSH key pair
    let ssh_key_pair = utils::ssh_setup::generate_ssh_key_pair().expect("Failed to generate SSH key pair");

    // Add the generated SSH public key to the parameters
    let mut params_with_ssh_key = params.clone();
    params_with_ssh_key.insert("--ssh_key".to_string(), ssh_key_pair.public_key.clone());

    // Run the installer with the updated parameters
    utils::run_installer::run_installer(params_with_ssh_key);

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
