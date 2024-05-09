fn main() -> Result<(), slint_build::CompileError> {
    slint_build::compile("./gui/main/window.slint")?;
    Ok(())
}
