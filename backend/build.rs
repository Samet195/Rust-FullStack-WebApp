use std::{env, fs};

use anyhow::Result;
use minify_js::{Session, TopLevelMode};

fn main() -> Result<()> {
    minify_assets()?;
    Ok(())
}

fn minify_assets() -> Result<()> {
    // minify_javascript()?;
    Ok(())
}

// ERROR: thread 'main' panicked at .../minify-js-0.6.0/src/minify/pass1.rs:331:13:
// ERROR: assertion failed: cons_expr.returns && alt_expr.returns
#[allow(unused)]
fn minify_javascript() -> Result<()> {
    let mut output = Vec::new();
    let dir = env::var("CARGO_MANIFEST_DIR")?;
    let session = Session::new();
    let code = fs::read(format!("{}/assets/frontend.js", dir))?;

    env::set_current_dir(&dir)?;
    minify_js::minify(&session, TopLevelMode::Module, code.as_slice(), &mut output).unwrap();
    fs::write(format!("{}/assets/frontend.js", dir), output)?;

    Ok(())
}
