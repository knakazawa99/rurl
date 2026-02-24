use indicatif::{ProgressBar, ProgressStyle};

/// Create a download progress bar given an optional content length
pub fn create_progress_bar(content_length: Option<u64>) -> ProgressBar {
    if let Some(len) = content_length {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
        );
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] {bytes} ({bytes_per_sec})",
            )
            .unwrap(),
        );
        pb
    }
}
