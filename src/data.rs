use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::{RwLock, RwLockWriteGuard};
use serde::{Deserialize, Serialize};

static ZINE_DATA: OnceCell<RwLock<ZineData>> = OnceCell::new();

pub fn load<P: AsRef<Path>>(path: P) {
    ZINE_DATA.get_or_init(|| RwLock::new(ZineData::new(path.as_ref()).unwrap()));
}

pub fn get() -> RwLockWriteGuard<'static, ZineData> {
    ZINE_DATA.get().unwrap().write()
}

/// Export all data into the `zine-data.json` file.
/// If the data is empty, we never create the `zine-data.json` file.
pub fn export<P: AsRef<Path>>(path: P) -> Result<()> {
    let data = get();
    if !data.is_empty() {
        let mut file = File::create(path.as_ref().join("zine-data.json"))?;
        file.write_all(data.export_to_json()?.as_bytes())?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZineData {
    url_previews: BTreeMap<String, (String, String)>,
}

impl ZineData {
    pub fn new(source: impl AsRef<Path>) -> Result<Self> {
        let path = source.as_ref().join("zine-data.json");
        if path.exists() {
            let json = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&json)?)
        } else {
            Ok(ZineData {
                url_previews: BTreeMap::default(),
            })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.url_previews.is_empty()
    }

    pub fn url_previews(&self) -> &BTreeMap<String, (String, String)> {
        &self.url_previews
    }

    pub fn insert_url_preview(&mut self, url: &str, preview: (String, String)) {
        self.url_previews.insert(url.to_owned(), preview);
    }

    fn export_to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
