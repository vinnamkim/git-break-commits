mod tree;
mod node;

use std::process::{Command, Output};
use std::{io, string};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitCommandError {
    #[error("IO error")]
    IOError { value: io::Error },
    #[error("Cannot change to UTF8 format")]
    UTF8Error { value: std::str::Utf8Error },
    #[error("status: {0:?}, stderr: {1:?}", value.status, value.stderr)]
    Error { value: Output },
}

impl From<std::io::Error> for GitCommandError {
    fn from(value: std::io::Error) -> Self {
        GitCommandError::IOError { value: value }
    }
}

impl From<std::str::Utf8Error> for GitCommandError {
    fn from(value: std::str::Utf8Error) -> Self {
        GitCommandError::UTF8Error { value: value }
    }
}

#[derive(Debug)]
struct SelectablePathBuf {
    selected: bool,
    path_buf: std::path::PathBuf,
}

struct GitSplitter {
    level: u8,
}

impl GitSplitter {
    pub fn list(self) -> Result<Vec<SelectablePathBuf>, GitCommandError> {
        let start = format!("HEAD~{}", self.level);
        let output = Command::new("git")
            .args(["diff", "--name-only", start.as_str(), "HEAD"])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    let diff_list: Vec<SelectablePathBuf> =
                        std::str::from_utf8(out.stdout.as_ref())?
                            .split("\n")
                            .map(|line| {
                                std::ffi::OsStr::new(
                                    line.strip_suffix("\r").unwrap_or(line),
                                )
                            })
                            .filter(|f| !f.is_empty())
                            .map(|line| SelectablePathBuf {
                                selected: false,
                                path_buf: std::path::Path::new(line).to_owned(),
                            })
                            .collect();

                    return Ok(diff_list);
                } else {
                    return Err(GitCommandError::Error { value: out });
                }
            }
            Err(out) => Err(GitCommandError::IOError { value: out }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{self, create_dir, File},
        io::{Error, Write},
        process::Command,
    };

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn my_test() -> Result<(), GitCommandError> {
        let temp_dir = tempdir()?;
        env::set_current_dir(&temp_dir)?;

        let log = prepare_git_project()?;
        println!("{:?}", log);

        let splitter = GitSplitter { level: 3 };
        println!("{:?}", splitter.list()?);

        Ok(())
    }

    fn prepare_git_project() -> Result<Output, std::io::Error> {
        let dir_names = ["dir_1", "dir_2", "dir_3"];
        let file_names = ["commit_1", "commit_2", "commit_3"];
        let curr_dir = env::current_dir()?;

        for dir_name in dir_names {
            let dir_path = curr_dir.join(dir_name);
            // let dir_path = Path:: dir_name.;
            create_dir(&dir_path)?;

            for fname in file_names {
                let mut file = File::create(dir_path.join(fname))?;
                let _ = file.write(b"")?;
            }
        }
        let init = Command::new("git").arg("init").output()?;
        let set_author_name = Command::new("git")
            .args(["config", "--local", "user.name", "Anonymous"])
            .output()?;
        let set_author_email = Command::new("git")
            .args([
                "config",
                "--local",
                "user.email",
                "anonymous@anonymous.com",
            ])
            .output()?;
        Command::new("git")
            .arg("commit")
            .args(["-m", "init", "--allow-empty"])
            .output()?;
        for fname in file_names {
            let mut commit_file_list = vec![];
            for dir_name in dir_names {
                commit_file_list.push(curr_dir.join(dir_name).join(fname));
            }
            let add = Command::new("git")
                .arg("add")
                .args(commit_file_list)
                .output()?;

            let commit = Command::new("git")
                .arg("commit")
                .args(["-m", fname])
                .output()?;
        }

        Command::new("git").arg("log").output()
    }
}
