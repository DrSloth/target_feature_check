use std::io::{self, Write};


fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    let features = target_feature_check::get_target_features();
    output_features(&features, &mut stdout)
}

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
