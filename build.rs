// build.rs

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::os::unix;
use std::path::Path;
use std::process::Command;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ValueAlternatives {
    alt: Vec<String>,
}

impl ValueAlternatives {
    pub fn new(alt: Vec<String>) -> Self {
        ValueAlternatives { alt: alt }
    }

    pub fn into_env<F: Fn(&String) -> bool>(self, key: &String, predicate: &F) -> bool {
        self
            .alt
            .into_iter()
            .find(predicate)
            .map(|v| env::set_var(key, &v))
            .is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub enum LinkSourceType {
    Direct,
    Env
}

#[derive(Serialize, Deserialize)]
pub struct LinkSource {
    source_type: LinkSourceType,
    value: String 
}

impl LinkSource {
    pub fn new(source_type: LinkSourceType, value: String) -> Self {
        LinkSource { source_type: source_type, value: value }
    }
    /// link to current working directory
    pub fn link_to<Q: AsRef<Path>>(self, link: Q) -> std::io::Result<()> {
        match self.source_type {
            LinkSourceType::Direct => unix::fs::symlink(self.value, link),
            LinkSourceType::Env => match env::var(&self.value) {
                Ok(o) => unix::fs::symlink(o, link),
                Err(_) => {
                    Ok(println!("cargo:warning=Env var `{}` not found", self.value))
                },
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BuildConfigurator {
    env: BTreeMap<String, ValueAlternatives>,
    sources: Vec<String>,
    links: Vec<LinkSource>
}

impl BuildConfigurator {
    pub fn new(env: BTreeMap<String, ValueAlternatives>, sources: Vec<String>, links: Vec<LinkSource>) -> Self {
        BuildConfigurator { env: env, sources: sources, links: links }
    }
    pub fn into_env<F: Fn(&String) -> bool>(self, predicate: &F) {
        for src in self.sources.into_iter() {
            Command::new("source")
                .arg(src)
                .output()
                .unwrap();
        }

        for (k, v) in self.env.into_iter() {
            if !v.into_env(&k, predicate) {
                println!("cargo:warning=No value alternative exist for key `{}`", k)
            }
        }

        for l in self.links.into_iter() {
            l.link_to(env::current_dir().unwrap().as_path()).unwrap()
        }
    }
}


fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();


    println!("gogadoda3");
    println!("cargo:warning={} {}", "output dir:", out_dir.into_string().unwrap());
    println!("cargo:rerun-if-changed=build.rs");
}
