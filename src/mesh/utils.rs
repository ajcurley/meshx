use std::ffi::OsStr;
use std::path::Path;

/// Check if the filename is a GZIP file
pub fn is_gzip(filename: &str) -> bool {
    let path = Path::new(filename);
    let extension = path.extension().and_then(OsStr::to_str);

    if let Some(ext) = extension {
        let ext = ext.to_lowercase();
        return ext == "gz" || ext == "gzip";
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_gzip() {
        let path = "/tmp/model.gz";
        assert!(is_gzip(&path));

        let path = "/tmp/model.GZ";
        assert!(is_gzip(&path));

        let path = "/tmp/model.gzip";
        assert!(is_gzip(&path));

        let path = "/tmp/model.GZIP";
        assert!(is_gzip(&path));
    }
}
