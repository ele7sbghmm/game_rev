use std::path::PathBuf;

enum FileExtension {
    Xbr,
    Res,
    Pak,
}
impl FileExtension {
    pub fn extension(&self) -> String {
        match &self {
            FileExtension::Xbr => format!("xbr"),
            FileExtension::Res => format!("res"),
            FileExtension::Pak => format!("pak"),
        }
    }
    pub fn from_extension(path: PathBuf) -> Self {
        match path
            .extension()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .as_str()
        {
            "xbr" => FileExtension::Xbr,
            "res" => FileExtension::Res,
            "pak" => FileExtension::Pak,
            _ => panic!("no extension {path:?}"),
        }
    }
}
