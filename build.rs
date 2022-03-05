// build.rs



mod build_cfg {
    use std::{collections::BTreeMap, borrow::Cow};
    use std::env;
    use std::os::unix;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use regex::{Regex, Captures};
    use serde::{Serialize, Deserialize};



    #[derive(Serialize, Deserialize)]
    pub struct EnvStr {
        str: String
    }
    impl From<String> for EnvStr {
        fn from(s: String) -> Self { EnvStr { str: s } }
    }
    impl From<&str> for EnvStr {
        fn from(s: &str) -> Self { EnvStr { str: String::from(s) } }
    }
    
    impl EnvStr {
        pub fn str(&self) -> Cow<'_, str> {
            Regex::new(r"\$[_a-zA-Z][_a-zA-Z0-9]*").unwrap().replace_all(&self.str, |caps: &Captures| -> String {
                match env::var(caps[0][1..].to_string()) {
                    Ok(res) => res,
                    Err(_) => {
                        println!("cargo:warning=Env var: `{:?}` not present", &caps[0][1..]);
                        caps[0].to_string()
                    },
                }
            })
        }

        pub fn path(&self) -> std::io::Result<PathBuf> { std::fs::canonicalize(self.str().into_owned()) }
    }


    #[derive(Serialize, Deserialize)]
    pub struct ValueAlternatives {
        alt: Vec<EnvStr>,
    }

    impl ValueAlternatives {
        pub fn new(alt: Vec<EnvStr>) -> Self {
            ValueAlternatives { alt: alt }
        }

        pub fn into_env<F: Fn(&String) -> bool>(self, key: &String, predicate: &F) -> bool {
            self
                .alt
                .into_iter().map(|x| x.str().into_owned())
                .find(predicate)
                .map(|v| env::set_var(key, &v))
                .is_some()
        }
    }

    impl From<EnvStr> for ValueAlternatives {
        fn from(s: EnvStr) -> Self {
            ValueAlternatives::new(vec![s])
        }
    }
    impl From<&str> for ValueAlternatives {
        fn from(s: &str) -> Self {
            ValueAlternatives::from(EnvStr::from(s))
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
    pub struct BuildConfiguration {
        env: BTreeMap<String, ValueAlternatives>,
        sources: Vec<EnvStr>,
        links: Vec<LinkSource>
    }

    impl BuildConfiguration {
        pub fn new(env: BTreeMap<String, ValueAlternatives>, sources: Vec<EnvStr>, links: Vec<LinkSource>) -> Self {
            BuildConfiguration { env: env, sources: sources, links: links }
        }
        pub fn into_env<F: Fn(&String) -> bool>(self, predicate: &F) {
            for src in self.sources.into_iter() {
                let cmd = src.path().unwrap();

                println!("cargo:warning=source: `{:?}`", &cmd);

                Command::new("/bin/bash")
                    .arg("-c")
                    .arg(format!("\". {:?}\"", &cmd))
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

    #[derive(Serialize, Deserialize)]
    pub struct BuildConfigProvider {
        targets: BTreeMap<String, BuildConfiguration>,
        default: BuildConfiguration
    }

    impl BuildConfigProvider {
        pub fn new(targets: BTreeMap<String, BuildConfiguration>, default: BuildConfiguration) -> Self {
            BuildConfigProvider { targets: targets, default: default }
        }

        //pub fn load(path: Path) -> Self {
        //    
        //}

        pub fn get(self, target_triple: &String) -> BuildConfiguration {
            match self.targets.into_iter().find(|(k, _)| k == target_triple) {
                Some((_, v)) => v,
                None => self.default,
            }
        } 

        pub fn get_from_env(self, target_triple_key: &String) -> BuildConfiguration {
            self.get(&env::var(target_triple_key).unwrap())
        }
    }
}



fn main() {
    use std::{collections::BTreeMap, env, path::Path};
    use crate::build_cfg::{BuildConfigProvider, BuildConfiguration, ValueAlternatives, LinkSource, EnvStr};

    //"TARGET"


    println!("cargo:warning=$PB_SDK_DIR/usr/bin/$PB_SUSTEM_PATH/config -> {}", EnvStr::from("$PB_SDK_DIR/usr/bin/$PB_SYSTEM_PATH/config").str().into_owned());


    let cfg = BuildConfigProvider::new(BTreeMap::from([
        (String::from("armv7-unknown-linux-gnueabi"), BuildConfiguration::new(
            BTreeMap::from([
                (String::from("CC"), ValueAlternatives::from("$PB_SDK_DIR/usr/bin/arm-obreey-linux-gnueabi-gcc")),
                (String::from("CXX"), ValueAlternatives::from("$PB_SDK_DIR/usr/bin/arm-obreey-linux-gnueabi-g++")),
                (String::from("QMAKE"), ValueAlternatives::from("$TOOLCHAIN_PATH/$TOOLCHAIN_PREFIX/sysroot/ebrmain/bin/qmake")),
                (String::from("QT_INCLUDE_PATH"), ValueAlternatives::from("$TOOLCHAIN_PATH/$TOOLCHAIN_PREFIX/sysroot/ebrmain/include")),
                (String::from("QT_LIBRARY_PATH"), ValueAlternatives::from("$TOOLCHAIN_PATH/$TOOLCHAIN_PREFIX/sysroot/ebrmain/lib")),
                (String::from("LD_LIBRARY_PATH"), ValueAlternatives::from("$QT_LIBRARY_PATH:$LD_LIBRARY_PATH"))
                ]),
            vec![EnvStr::from("$PB_SDK_DIR/../env_set.sh")],
            vec![LinkSource::new(build_cfg::LinkSourceType::Env, String::from("$PB_SYSTEM_PATH"))]
        )),
    ]),

    BuildConfiguration::new(
        BTreeMap::from([
            (String::from("QMAKE"), ValueAlternatives::from("$PB_SDK_DIR/local/qt5/bin/qmake")),
            (String::from("QT_INCLUDE_PATH"), ValueAlternatives::from("$PB_SDK_DIR/local/qt5/include")),
            (String::from("QT_LIBRARY_PATH"), ValueAlternatives::from("$PB_SDK_DIR/local/qt5/lib")),
            (String::from("LD_LIBRARY_PATH"), ValueAlternatives::from("$QT_LIBRARY_PATH:$LD_LIBRARY_PATH"))
        ]),
        vec![],
        vec![LinkSource::new(build_cfg::LinkSourceType::Env, String::from("$PB_SYSTEM_PATH"))]
    ));

    cfg
        .get_from_env(&String::from("TARGET"))
        .into_env(&|s: &String| Path::new(s).exists());


    let cc = env::var("CC").unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    println!("gogadoda3");
    println!("cargo:warning={} {}", "output dir:", out_dir.into_string().unwrap());
    println!("cargo:warning={} {}", "CC:", cc);


    env::set_var("PATH", "/home/ivan/workspace/SDK-B288/usr/bin:/home/ivan/.cargo/bin:~/apps/giteye:~/apps/giteye:~/apps/giteye:~/apps/giteye:/home/ivan/.cargo/bin:/home/ivan/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin");


    
    println!("cargo:rerun-if-changed=build.rs");
}
