fn main() -> Result<(), slint_build::CompileError> {
    slint_build::compile("./gui/export.slint")?;
    Ok(())
}
