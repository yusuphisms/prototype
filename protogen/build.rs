use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/fancy/fancy.proto"], &["src/"])?;
    Ok(())
}
