pub mod types;
pub mod format;

use crate::format::line_format;

fn main() -> std::io::Result<()> {
    line_format()
}
