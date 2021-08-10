extern crate itertools;
extern crate semver;
extern crate serde;
extern crate winres;
#[path = "src/jsonstructs_versionsdb.rs"]
mod jsonstructs_versionsdb;

use anyhow::Result;
use itertools::Itertools;
use jsonstructs_versionsdb::{JuliaupVersionDB, JuliaupVersionDBChannel, JuliaupVersionDBVersion};
use semver::Version;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn produce_version_db() -> Result<JuliaupVersionDB> {
    let mut original_available_versions: Vec<Version> = Vec::new();

    let lts_version = Version::parse("1.0.5")?;
    let beta_version = Version::parse("1.7.0-beta3")?;
    let rc_version = Version::parse("1.7.0-beta3")?;

    original_available_versions.push(Version::parse("0.7.0")?);
    original_available_versions.push(Version::parse("1.0.0")?);
    original_available_versions.push(Version::parse("1.0.1")?);
    original_available_versions.push(Version::parse("1.0.2")?);
    original_available_versions.push(Version::parse("1.0.3")?);
    original_available_versions.push(Version::parse("1.0.4")?);
    original_available_versions.push(Version::parse("1.0.5")?);
    original_available_versions.push(Version::parse("1.1.0")?);
    original_available_versions.push(Version::parse("1.1.1")?);
    original_available_versions.push(Version::parse("1.2.0")?);
    original_available_versions.push(Version::parse("1.3.0")?);
    original_available_versions.push(Version::parse("1.3.1")?);
    original_available_versions.push(Version::parse("1.4.0")?);
    original_available_versions.push(Version::parse("1.4.1")?);
    original_available_versions.push(Version::parse("1.4.2")?);
    original_available_versions.push(Version::parse("1.5.0")?);
    original_available_versions.push(Version::parse("1.5.1")?);
    original_available_versions.push(Version::parse("1.5.2")?);
    original_available_versions.push(Version::parse("1.5.3")?);
    original_available_versions.push(Version::parse("1.5.4")?);
    original_available_versions.push(Version::parse("1.6.0")?);
    original_available_versions.push(Version::parse("1.6.1")?);
    original_available_versions.push(Version::parse("1.6.2")?);
    original_available_versions.push(Version::parse("1.7.0-beta1")?);
    original_available_versions.push(Version::parse("1.7.0-beta2")?);
    original_available_versions.push(Version::parse("1.7.0-beta3")?);

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS")?;

    let mut db = JuliaupVersionDB {
        available_versions: HashMap::new(),
        available_channels: HashMap::new(),
    };

    for v in &original_available_versions {
        if target_os == "windows" && target_arch == "x86_64" {
            db.available_versions.insert(
                format!("{}+0~x64", v),
                JuliaupVersionDBVersion {url: format!("https://julialang-s3.julialang.org/bin/winnt/x64/{}.{}/julia-{}-win64.tar.gz", v.major, v.minor, v)}
            );
            db.available_versions.insert(
                format!("{}+0~x86", v),
                JuliaupVersionDBVersion {url: format!("https://julialang-s3.julialang.org/bin/winnt/x86/{}.{}/julia-{}-win64.tar.gz", v.major, v.minor, v)}
            );

            db.available_channels.insert(
                format!("{}", v),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x64", v),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x86", v),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else if target_os == "windows" && target_arch == "x86" {
            db.available_versions.insert(
                format!("{}+0~x86", v),
                JuliaupVersionDBVersion {
                    url: format!(
                        "https://julialang-s3.julialang.org/bin/winnt/x86/{}/julia-{}-win64.tar.gz",
                        v.minor, v
                    ),
                },
            );

            db.available_channels.insert(
                format!("{}", v),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x86", v),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else {
            panic!("Building on this platform is currently not supported.")
        }
    }

    let minor_channels = &original_available_versions
        .iter()
        .filter(|&v| v.pre == semver::Prerelease::EMPTY)
        .into_grouping_map_by(|&v| (v.major, v.minor))
        .max();

    let major_channels = &original_available_versions
        .iter()
        .filter(|&v| v.pre == semver::Prerelease::EMPTY)
        .into_grouping_map_by(|&v| v.major)
        .max();

    for ((major, minor), v) in minor_channels {
        if target_os == "windows" && target_arch == "x86_64" {
            db.available_channels.insert(
                format!("{}.{}", major, minor),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}.{}~x64", major, minor),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}.{}~x86", major, minor),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else if target_os == "windows" && target_arch == "x86" {
            db.available_channels.insert(
                format!("{}.{}", major, minor),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
            db.available_channels.insert(
                format!("{}.{}~x86", major, minor),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else {
            panic!("Building on this platform is currently not supported.")
        }
    }

    for (major, v) in major_channels {
        if target_os == "windows" && target_arch == "x86_64" {
            db.available_channels.insert(
                format!("{}", major),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x64", major),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x64", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x86", major),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else if target_os == "windows" && target_arch == "x86" {
            db.available_channels.insert(
                format!("{}", major),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
            db.available_channels.insert(
                format!("{}~x86", major),
                JuliaupVersionDBChannel {
                    version: format!("{}+0~x86", v),
                },
            );
        } else {
            panic!("Building on this platform is currently not supported.")
        }
    }

    let release_version = &original_available_versions
        .iter()
        .filter(|&v| v.pre == semver::Prerelease::EMPTY)
        .max().unwrap();

    if target_os == "windows" && target_arch == "x86_64" {
        db.available_channels.insert(
            "release".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", release_version),
            },
        );
        db.available_channels.insert(
            "release~x64".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", release_version),
            },
        );
        db.available_channels.insert(
            "release~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", release_version),
            },
        );

        db.available_channels.insert(
            "lts".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", lts_version),
            },
        );
        db.available_channels.insert(
            "lts~x64".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", lts_version),
            },
        );
        db.available_channels.insert(
            "lts~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", lts_version),
            },
        );     
        
        db.available_channels.insert(
            "beta".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", beta_version),
            },
        );
        db.available_channels.insert(
            "beta~x64".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", beta_version),
            },
        );
        db.available_channels.insert(
            "beta~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", beta_version),
            },
        );  

        db.available_channels.insert(
            "rc".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", rc_version),
            },
        );
        db.available_channels.insert(
            "rc~x64".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x64", rc_version),
            },
        );
        db.available_channels.insert(
            "rc~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", rc_version),
            },
        );  
    } else if target_os == "windows" && target_arch == "x86" {
        db.available_channels.insert(
            "release".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", release_version),
            },
        );
        db.available_channels.insert(
            "release~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", release_version),
            },
        );

        db.available_channels.insert(
            "lts".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", lts_version),
            },
        );
        db.available_channels.insert(
            "lts~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", lts_version),
            },
        );        

        db.available_channels.insert(
            "beta".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", beta_version),
            },
        );
        db.available_channels.insert(
            "beta~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", beta_version),
            },
        );     

        db.available_channels.insert(
            "rc".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", rc_version),
            },
        );
        db.available_channels.insert(
            "rc~x86".to_string(),
            JuliaupVersionDBChannel {
                version: format!("{}+0~x86", rc_version),
            },
        );     
    } else {
        panic!("Building on this platform is currently not supported.")
    }

    Ok(db)
}

fn main() -> Result<()> {
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let db = produce_version_db()?;

    let version_db_path = out_path.join("versionsdb.json");
    let file = File::create(&version_db_path)?;
    serde_json::to_writer(file, &db)?;

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/julia.ico");
        res.compile().unwrap();
    }

    Ok(())
}
