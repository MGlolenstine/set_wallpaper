use download_image;
use enquote;
use get_stdout;
use run;
use Result;

/// Returns the current wallpaper.
pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    get_stdout(
        "osascript",
        &[
            "-e",
            r#"tell application "Finder" to get POSIX path of (get desktop picture as alias)"#,
        ],
    )
}

// Sets the wallpaper from a file.
pub fn set_from_path(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    run(
        "osascript",
        &[
            "-e",
            &format!(
                r#"tell application "Finder" to set desktop picture to POSIX file {}"#,
                enquote::enquote('"', path),
            ),
        ],
    )
}

// Sets the wallpaper from a URL.
#[cfg("from-url")]
pub fn set_from_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = download_image(&url.parse()?)?;
    set_from_path(&path)
}
