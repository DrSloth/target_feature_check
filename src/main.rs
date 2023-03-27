//! Output all target features the current device supports

use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout().lock();
    let features = target_feature_check::get_target_features();
    if let Err(e) = output_features(&features, &mut stdout) {
        eprintln!("Error while writing to stdout: {}", e)
    }
}

/// Output all given features to stdout as a feature list
fn output_features<W: Write>(features: &[&str], writer: &mut W) -> io::Result<()> {
    let mut features = features.iter();
    match features.next() {
        Some(s) => write!(writer, "target-feature=+{}", s)?,
        None => return Ok(()),
    }

    for feature in features {
        write!(writer, ",+{}", feature)?;
    }

    writeln!(writer)
}
