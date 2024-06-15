use anyhow::Result;
use vergen::EmitBuilder;

pub fn main() -> Result<()> {
    // NOTE: This will output everything, and requires all features enabled.
    // NOTE: See the EmitBuilder documentation for configuration options.
    EmitBuilder::builder()
        .all_build()
        // .all_cargo()
        .all_git()
        // .git_describe(false, false, None)
        .all_rustc()
        // .all_sysinfo()
        .emit()?;
    Ok(())
}
