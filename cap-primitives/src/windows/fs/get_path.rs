use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Calculates system path of `file`.
///
/// Note that this function will automatically strip the extended
/// prefix from the resultant path to allow for joining this resultant
/// path with relative components.
pub(crate) fn get_path(file: &fs::File) -> io::Result<PathBuf> {
    // get system path to the handle
    let path = winx::file::get_file_path(file)?;

    // strip extended prefix; otherwise we will error out on any relative
    // components with `out_path`
    let wide: Vec<_> = path.as_os_str().encode_wide().collect();
    let wide_final = if wide.starts_with(&['\\' as u16, '\\' as _, '?' as _, '\\' as _]) {
        &wide[4..]
    } else {
        &wide
    };
    Ok(PathBuf::from(OsString::from_wide(wide_final)))
}

/// Convenience function for calling `get_path` and concatenating the result
/// with `path`. This function also checks if `path` is absolute in which case,
/// it emulates POSIX and returns the absolute path unmodified instead.
pub(super) fn concatenate_or_return_absolute(file: &fs::File, path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.into());
    }

    let file_path = get_path(file)?;
    Ok(file_path.join(path))
}
