#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::MixFileClasses;

    #[test]
    fn test_cache() {
        let mut mfs = MixFileClasses::new();
        let aud_mix = env!("CARGO_MANIFEST_DIR").to_owned() + "/../tests/testdata/cc1demo1/AUD.MIX";
        let p = Path::new(&aud_mix);
        mfs.try_insert(p).unwrap();
        let mf = mfs.cache(&p).unwrap();
        assert_eq!(mf.Buffer.len(), 32);
        assert_eq!(mf.DataSize, 1890256);
        assert_eq!(mf.Data.len(), 1890256);
        assert_eq!(mf.Filename, PathBuf::from(aud_mix,));
    }
}
