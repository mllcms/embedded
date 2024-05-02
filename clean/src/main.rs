use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    fs::DirEntry,
    path::{Path, PathBuf},
    sync::{mpsc, mpsc::Sender},
    thread,
};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    match Config::load("clean.toml") {
        Ok(config) => match config.clean() {
            Ok(_) => println!("清理成功!"),
            Err(err) => eprintln!("清理失败: {err}"),
        },
        Err(err) => return eprintln!("加载配置失败: {err}"),
    };
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub path:   PathBuf,
    pub clean:  HashSet<PathBuf>,
    pub ignore: HashSet<PathBuf>,
    #[serde(skip)]
    pub output: Option<Sender<String>>,
}

impl Config {
    fn load(path: &str) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        let mut config: Self = toml::from_str(&data)?;
        let (tx, rx) = mpsc::channel();
        config.output = Some(tx);

        thread::spawn(move || {
            for item in rx {
                eprintln!("{item}")
            }
        });
        Ok(config)
    }

    fn clean(&self) -> anyhow::Result<()> {
        fs::read_dir(&self.path)?.par_bridge().flatten().for_each(|entry| {
            if self.ignore.contains(&PathBuf::from(entry.file_name())) {
                return;
            }
            let path = entry.path();
            match entry.metadata() {
                Ok(meta) if !meta.is_dir() => return,
                Err(err) => return self.output(path, err),
                _ => {}
            }
            match fs::read_dir(&path) {
                Ok(v) => v.par_bridge().flatten().for_each(|entry| {
                    let path = entry.path();
                    if let Err(err) = self.delete(entry, &path) {
                        self.output(path, err)
                    }
                }),
                Err(err) => return self.output(path, err),
            }
        });

        Ok(())
    }

    fn delete(&self, entry: DirEntry, path: &PathBuf) -> anyhow::Result<()> {
        if self.clean.contains(&PathBuf::from(entry.file_name())) {
            let meta = entry.metadata()?;
            if meta.is_dir() {
                fs::remove_dir_all(path)?;
            } else if meta.is_file() {
                fs::remove_file(path)?
            }
        }
        Ok(())
    }

    fn output<P: AsRef<Path>>(&self, path: P, err: impl Display) {
        if let Some(tx) = &self.output {
            let path = path.as_ref().display().to_string().replace('\\', "/");
            tx.send(format!("清理失败: {path} {err}")).ok();
        }
    }
}
