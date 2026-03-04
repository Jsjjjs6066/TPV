use std::fmt::Debug;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Result, Write};
use std::path::Path;
use std::sync::{Arc, LazyLock};

use crate::btmd::element::RawElement;

static LOG_DIR: &str = ".btmd_log";
static LOG_FILE_NAME: &str = "debug.log";
static PAGE_LOG_FILE_NAME: &str = "page_debug.log";

static LOG_FILE: LazyLock<parking_lot::RwLock<Result<File>>> = LazyLock::new(|| {
    create_dir(LOG_DIR).unwrap_or_default();
    parking_lot::RwLock::new(
        OpenOptions::new()
            .write(true)
            .append(true)
            .read(true)
            .create(true)
            .open(Path::new(LOG_DIR).join(Path::new(LOG_FILE_NAME))),
    )
});

static PAGE_LOG_FILE: LazyLock<parking_lot::RwLock<Result<File>>> = LazyLock::new(|| {
    create_dir(LOG_DIR).unwrap_or_default();
    parking_lot::RwLock::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(Path::new(LOG_DIR).join(Path::new(PAGE_LOG_FILE_NAME))),
    )
});

pub fn write_log(s: &[u8]) -> Result<()> {
    let mut file_guard = LOG_FILE.write();
    let file = file_guard.as_mut().unwrap();
    file.write_all(s)?;
    file.write_all("\n".as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn write_page(page_body: &Vec<Arc<parking_lot::RwLock<RawElement>>>) -> Result<()> {
    PAGE_LOG_FILE
        .write()
        .as_mut()
        .unwrap()
        .write_all(format!("{:#?}\n", page_body).as_bytes())?;
    Ok(())
}

pub fn write_log_debug(s: impl Debug) -> Result<()> {
    write_log((format!("{:#?}", s)).as_bytes())
}