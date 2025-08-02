use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstallInfo {
    pub architecture: String,

    pub bucket: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub homepage: String,

    // pub license: License,
    pub license: Option<LicenseField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub innosetup: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,

    /// Architecture-independent - `noarch` download url(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<OneOrVec<String>>,

    /// The `extract_dir` field is used to define the directory to which the
    /// archive should be extracted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_dir: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_to: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_install: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub installer: Option<Installer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_uninstall: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstaller: Option<Uninstaller>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_uninstall: Option<OneOrVec<String>>,

    /// The `bin` field is used to define binaries that need to be shimmed/added
    /// to the `shimes` directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<OneOrVec<OneOrVec<String>>>,

    /// The `env_add_path` field is used to define path(s) that need to be added
    /// to the `PATH` environment variable during installation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_add_path: Option<OneOrVec<String>>,

    /// The `env_set` field is used to define environment variables that should
    /// be set during installation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_set: Option<HashMap<String, String>>,

    /// The `shortcuts` field is used to define shortcuts that need to be created
    /// in the `Scoop Apps` directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<Vec<OneOrVec<String>>>,

    /// The `persist` field is used to define files/directories that need to be
    /// persisted during uninstallation.
    #[serde(skip_serializing_if = "Option::is_none")]
    persist: Option<OneOrVec<String>>,

    /// The `psmodule` field is used to define PowerShell module that need to
    /// be imported during installation.
    #[serde(skip_serializing_if = "Option::is_none")]
    psmodule: Option<Psmodule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<HashMap<String, OneOrVec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkver: Option<CheckverField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoupdate: Option<Autoupdate>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub notes: Option<Vectorized<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<OneOrVec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Architecture {
    /// Ia32 architecture specification.
    #[serde(rename = "32bit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ia32: Option<AutoupdateArchSpec>,

    /// Amd64 architecture specification.
    #[serde(rename = "64bit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd64: Option<AutoupdateArchSpec>,

    /// Aarch64 architecture specification.
    #[serde(rename = "arm64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aarch64: Option<AutoupdateArchSpec>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AutoupdateArchSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<HashField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_install: Option<Vectorized<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcuts: Option<Vec<OneOrVec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<OneOrVec<OneOrVec<String>>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LicenseField {
    Complex(License),
    Simple(String),
}

/// License information of a Scoop package.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct License {
    /// The identifier of the license, which is intended to be a SPDX license.
    identifier: String,

    /// The url to the license text.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vectorized<T>(Vec<T>);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HashString {
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Installer {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<Vectorized<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<Vectorized<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Uninstaller {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vectorized<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Vectorized<String>>,
}

/// PowerShell module information of a Scoop package.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Psmodule {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sourceforge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,

    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CheckverField {
    /// 原始复杂形式：单个或多个 Checkver
    Complex(OneOrVec<Checkver>),
    /// 简单字符串形式
    Simple(String),
}

// impl CheckverField {
//     /// 统一抽象成 `OneOrVec<Checkver>`（例如方便后续处理）
//     fn as_one_or_vec(&self) -> OneOrVec<Checkver> {
//         match self {
//             CheckverField::Complex(v) => v.clone(),
//             CheckverField::Simple(s) => OneOrVec(vec![Checkver::from(s.clone())]),
//         }
//     }
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Checkver {
    #[serde(alias = "re")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(alias = "jp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonpath: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub useragent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Vectorized<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceforge: Option<Sourceforge>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Autoupdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<AutoupdateArchitecture>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_dir: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<HashField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<OneOrVec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AutoupdateArchitecture {
    #[serde(rename = "32bit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ia32: Option<AutoupdateArchSpec>,
    #[serde(rename = "64bit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd64: Option<AutoupdateArchSpec>,
    #[serde(rename = "arm64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aarch64: Option<AutoupdateArchSpec>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HashField {
    Complex(HashExtraction),
    Simple(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HashExtraction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    #[serde(alias = "jp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonpath: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpath: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<HashExtractionMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HashExtractionMode {
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "extract")]
    Extract,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "xpath")]
    Xpath,
    #[serde(rename = "rdf")]
    Rdf,
    #[serde(rename = "metalink")]
    Metalink,
    #[serde(rename = "fosshub")]
    Fosshub,
    #[serde(rename = "sourceforge")]
    Sourceforge,
}

#[derive(Clone, Debug)]
pub struct OneOrVec<T>(pub Vec<T>);

impl<T> From<Vec<T>> for OneOrVec<T> {
    fn from(v: Vec<T>) -> Self {
        OneOrVec(v)
    }
}
impl<T> From<T> for OneOrVec<T> {
    fn from(t: T) -> Self {
        OneOrVec(vec![t])
    }
}

impl<T: Serialize> Serialize for OneOrVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // match self.0.len() {
        //     0 => serializer.serialize_none(),
        //     1 => serializer.serialize_some(&self.0[0]),
        //     _ => serializer.collect_seq(self.0.iter()),
        // }
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}
impl<'de, T> Deserialize<'de> for OneOrVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<OneOrVec<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper<T> {
            Single(T),
            Multiple(Vec<T>),
        }

        match Helper::<T>::deserialize(deserializer)? {
            Helper::Single(t) => Ok(OneOrVec(vec![t])),
            Helper::Multiple(v) => Ok(OneOrVec(v)),
        }
    }
}

#[cfg(test)]
mod tests_manifest {

    use std::fs;

    use serde_json::json;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Ta<T> {
        content: OneOrVec<T>,
    }

    #[test]
    fn test_oneorvec() {
        let a: Ta<i32> = Ta {
            content: OneOrVec::from(vec![]),
        };
        let sa = serde_json::to_string(&a).unwrap();
        let da: Ta<i32> = serde_json::from_str(&sa).unwrap();
        println!("sa:{}", sa);
        println!("da:{:?}", da);
        assert_eq!(sa, "{\"content\":[]}".to_string());

        let b: Ta<i32> = Ta {
            content: OneOrVec::from(vec![1]),
        };
        let sb = serde_json::to_string(&b).unwrap();
        let db: Ta<i32> = serde_json::from_str(&sb).unwrap();
        println!("sb:{}", sb);
        println!("db:{:?}", db);
        assert_eq!(sb, "{\"content\":1}".to_string());

        let c: Ta<i32> = Ta {
            content: OneOrVec::from(vec![1, 2]),
        };
        let sc = serde_json::to_string(&c).unwrap();
        let dc: Ta<i32> = serde_json::from_str(&sc).unwrap();
        println!("sb:{}", sc);
        println!("db:{:?}", dc);
        assert_eq!(sc, "{\"content\":[1,2]}".to_string());
        // println!("b:{}", serde_json::to_string(&b).unwrap());
    }

    #[test]
    fn it_works() {
        let data = json!({
    "version": "250625",
    "description": "Highly customizable media player",
    "homepage": "https://potplayer.daum.net",
    "license": {
        "identifier": "Freeware",
        "url": "https://potplayer.daum.net/publicRelation"
    },
    "architecture": {
        "64bit": {
            "url": "https://t1.daumcdn.net/potplayer/PotPlayer/Version/250625/PotPlayerSetup64.exe#/dl.7z",
            "hash": "e290a654afbe4554b45568bc536e2b0dc47d9e9cbe47b22337757100fdccb265",
            "shortcuts": [
                [
                    "PotPlayer64.exe",
                    "PotPlayer"
                ],
                [
                    "PotPlayerMini64.exe",
                    "PotPlayer Mini"
                ]
            ]
        },
        "32bit": {
            "url": "https://t1.daumcdn.net/potplayer/PotPlayer/Version/250625/PotPlayerSetup.exe#/dl.7z",
            "hash": "6c7399185d24a403f291d6df3ad957706d8704b0261e899f3e5174295620ecd1",
            "shortcuts": [
                [
                    "PotPlayer.exe",
                    "PotPlayer"
                ],
                [
                    "PotPlayerMini.exe",
                    "PotPlayer Mini"
                ]
            ]
        }
    },
    "pre_install": [
        "Remove-Item \"$dir\\`$*\" -Force -Recurse",
        "$CONT = @(",
        "    '[Settings]'",
        "    'AutoUpdateStart=0'",
        "    'CheckAutoUpdate=0'",
        "    'FastAutoUpdate=0'",
        ")",
        "    # Set Configs for all executables",
        "foreach ($f in @('PotPlayerMini64.ini', 'PotPlayerMini.ini', 'PotPlayer64.ini', 'PotPlayer.ini')) {",
        "    if (-not (Test-Path \"$persist_dir\\$f\")) {",
        "        Write-Host 'File' $f 'does not exists. Creating' -f Yellow",
        "        Set-Content \"$dir\\$f\" $CONT -Encoding Ascii",
        "    }",
        "}"
    ],
    "persist": [
        "Capture",
        "Extension",
        "Extention",
        "IconPack",
        "Logos",
        "Module",
        "Playlist",
        "PxShader",
        "Skins",
        "UrlList",
        "PotPlayer.ini",
        "PotPlayer64.ini",
        "PotPlayerMini.ini",
        "PotPlayerMini64.ini"
    ],
    "checkver": {
        "url": "https://t1.daumcdn.net/potplayer/PotPlayer/v4/Update2/UpdateEng.html",
        "regex": "\\[(\\d+)\\]"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://t1.daumcdn.net/potplayer/PotPlayer/Version/$version/PotPlayerSetup64.exe#/dl.7z"
            },
            "32bit": {
                "url": "https://t1.daumcdn.net/potplayer/PotPlayer/Version/$version/PotPlayerSetup.exe#/dl.7z"
            }
        }
    }
}
)
        .to_string();
        // let data = fs::read_to_string("tests\\everything.json")
        //     .expect("LogRocket: Should have been able to read the file");
        let v: Manifest = serde_json::from_str(&data).unwrap();
        println!("{}", serde_json::to_string(&v).unwrap())
    }
}
