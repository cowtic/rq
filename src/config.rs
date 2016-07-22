use std::path;

use glob;
use xdg_basedir::dirs;

use error;

#[derive(Debug)]
pub struct Paths {
    config_home: path::PathBuf,
    cache_home: path::PathBuf,
    data_home: path::PathBuf,
    config_dirs: Vec<path::PathBuf>,
    data_dirs: Vec<path::PathBuf>,
}

pub struct Config {
    write_path: path::PathBuf,
    read_paths: Vec<path::PathBuf>,
}

impl Paths {
    pub fn new() -> error::Result<Paths> {
        fn resolve(mut p: path::PathBuf) -> path::PathBuf {
            p.push("rq");
            p
        }

        fn resolve_all(ps: Vec<path::PathBuf>) -> Vec<path::PathBuf> {
            ps.into_iter().map(resolve).collect()
        }

        Ok(Paths {
            config_home: resolve(try!(dirs::get_config_home())),
            cache_home: resolve(try!(dirs::get_cache_home())),
            data_home: resolve(try!(dirs::get_data_home())),
            config_dirs: resolve_all(dirs::get_config_dirs()),
            data_dirs: resolve_all(dirs::get_data_dirs()),
        })
    }

    pub fn preferred_config<P>(&self, path: P) -> path::PathBuf
        where P: AsRef<path::Path>
    {
        let mut result = self.config_home.clone();
        result.push(path);
        result
    }

    pub fn preferred_cache<P>(&self, path: P) -> path::PathBuf
        where P: AsRef<path::Path>
    {
        let mut result = self.cache_home.clone();
        result.push(path);
        result
    }

    pub fn preferred_data<P>(&self, path: P) -> path::PathBuf
        where P: AsRef<path::Path>
    {
        let mut result = self.data_home.clone();
        result.push(path);
        result
    }

    pub fn find_config(&self, pattern: &str) -> error::Result<Vec<path::PathBuf>> {
        find(&self.config_home, &self.config_dirs, pattern)
    }

    pub fn find_data(&self, pattern: &str) -> error::Result<Vec<path::PathBuf>> {
        find(&self.data_home, &self.data_dirs, pattern)
    }
}

impl Config {
    pub fn new(paths: &Paths) -> error::Result<Config> {
        let write = paths.preferred_config("rq.conf");
        let mut read = try!(paths.find_config("rq.conf"));
        read.push(write.clone());

        Ok(Config {
            read_paths: read,
            write_path: write,
        })
    }

    pub fn get(key: &str) -> String {

    }
}

fn find<P>(home: &path::Path, dirs: &[P], pattern: &str) -> error::Result<Vec<path::PathBuf>>
    where P: AsRef<path::Path>
{
    let mut result = Vec::new();

    try!(run_pattern(home, pattern, &mut result));

    for dir in dirs.iter() {
        try!(run_pattern(dir.as_ref(), pattern, &mut result));
    }

    Ok(result)
}

fn run_pattern(dir: &path::Path,
               pattern: &str,
               result: &mut Vec<path::PathBuf>)
               -> error::Result<()> {

    let full_pattern = format!("{}/{}", dir.to_string_lossy(), pattern);

    for entry in try!(glob::glob(&full_pattern)) {
        result.push(try!(entry));
    }

    Ok(())
}
