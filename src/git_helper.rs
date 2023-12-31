use rand::{distributions::Alphanumeric, Rng};

use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};
use thiserror::Error;

use tempfile::NamedTempFile;

#[derive(Error, Debug)]
pub enum GitCommandError {
    #[error("IO error")]
    IOError { value: io::Error },
    #[error("Cannot change to UTF8 format")]
    UTF8Error { value: std::str::Utf8Error },
    #[error("status: {0:?}, stderr: {1:?}", value.status, value.stderr)]
    GitError { value: Output },
    #[error("Invalid function call")]
    InvalidFunctionCallError,
    #[error("Empty list (level: {0:?}", level)]
    EmptyListError { level: u8 },
}

impl From<std::io::Error> for GitCommandError {
    fn from(value: std::io::Error) -> Self {
        GitCommandError::IOError { value }
    }
}

impl From<std::str::Utf8Error> for GitCommandError {
    fn from(value: std::str::Utf8Error) -> Self {
        GitCommandError::UTF8Error { value }
    }
}

pub struct GitCommitCandidate {
    pub msg: String,
    pub file_paths: Vec<PathBuf>,
}

pub struct GitHelper {
    depth: u8,
    curr_branch_name: String,
    temp_branch_name: Option<String>,
}

impl GitHelper {
    pub fn new(depth: u8) -> Result<GitHelper, GitCommandError> {
        let git_helper = GitHelper {
            depth,
            curr_branch_name: GitHelper::get_current_branch_name()?,
            temp_branch_name: None,
        };

        Ok(git_helper)
    }

    fn get_current_branch_name() -> Result<String, GitCommandError> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .output()?;

        if !output.status.success() {
            return Err(GitCommandError::GitError { value: output });
        }

        let branch_name = std::str::from_utf8(output.stdout.as_slice())?
            .trim_end_matches("\r\n")
            .trim_end_matches("\n")
            .to_owned();
        Ok(branch_name)
    }

    pub fn list(&self) -> Result<Vec<PathBuf>, GitCommandError> {
        let start = format!("HEAD~{}", self.depth);
        let output = Command::new("git")
            .args(["diff", "--name-only", start.as_str(), "HEAD"])
            .output()?;

        if !output.status.success() {
            return Err(GitCommandError::GitError { value: output });
        }

        let diff_list: Vec<PathBuf> =
            std::str::from_utf8(output.stdout.as_ref())?
                .split("\n")
                .map(|line| {
                    std::ffi::OsStr::new(
                        line.strip_suffix("\r").unwrap_or(line),
                    )
                })
                .filter(|f| !f.is_empty())
                .map(|line| std::path::Path::new(line).to_owned())
                .collect();

        if diff_list.len() != 0 {
            Ok(diff_list)
        } else {
            Err(GitCommandError::EmptyListError { level: self.depth })
        }
    }

    pub fn checkout_to_temp_branch(
        &mut self,
    ) -> Result<Output, GitCommandError> {
        let rand_key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let branch_name = format!("tmp-branch/{}", rand_key);
        let output = Command::new("git")
            .args(["checkout", "-b", branch_name.as_str()])
            .output()?;

        if !output.status.success() {
            return Err(GitCommandError::GitError { value: output });
        }

        self.temp_branch_name = Some(branch_name);
        Ok(output)
    }

    pub fn restore_branch(&mut self) -> Result<Output, GitCommandError> {
        if let Some(name) = &self.temp_branch_name {
            let output = Command::new("git")
                .args(["checkout", "-B", self.curr_branch_name.as_str()])
                .output()?;

            if !output.status.success() {
                return Err(GitCommandError::GitError { value: output });
            }

            let output = Command::new("git")
                .args(["branch", "-d", name.as_str()])
                .output()?;

            if !output.status.success() {
                return Err(GitCommandError::GitError { value: output });
            }

            Ok(output)
        } else {
            Err(GitCommandError::InvalidFunctionCallError)
        }
    }

    pub fn reset(&self) -> Result<Output, GitCommandError> {
        let start = format!("HEAD~{}", self.depth);
        let output = Command::new("git")
            .args(["reset", "--soft", start.as_str()])
            .output()?;

        if !output.status.success() {
            return Err(GitCommandError::GitError { value: output });
        }

        Ok(output)
    }

    pub fn commit(
        &self,
        commits: &Vec<GitCommitCandidate>,
    ) -> Result<Vec<Output>, GitCommandError> {
        let mut outputs = vec![];

        for commit in commits {
            let mut file = NamedTempFile::new()?;

            let path: Vec<&str> = commit
                .file_paths
                .iter()
                .map(|item| {
                    item.as_os_str()
                        .to_str()
                        .expect("Cannot change the file path to str")
                })
                .collect();
            let path = path.join("\n");

            file.write(path.as_bytes())?;
            let spec_filepath = file
                .path()
                .to_str()
                .expect("Cannot change the named temporary file path to str");

            let msg = &commit.msg;
            let args = vec![
                "commit",
                "-m",
                msg.as_str(),
                "--pathspec-from-file",
                spec_filepath,
            ];

            let output = Command::new("git").args(args).output()?;

            if !output.status.success() {
                return Err(GitCommandError::GitError { value: output });
            }

            outputs.push(output);
        }
        Ok(outputs)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        fs::{create_dir, File},
        io::Write,
        process::Command,
    };

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test() {
        let result = test_impl();
        if result.is_err() {
            dbg!(&result);
        }
        assert!(result.is_ok());
    }

    fn test_impl() -> Result<(), GitCommandError> {
        let temp_dir = tempdir()?;
        env::set_current_dir(&temp_dir)?;
        dbg!(&temp_dir);

        let log = prepare_git_project()?;
        println!("{:?}", log);
        let branch_name = GitHelper::get_current_branch_name()?;

        let mut helper = GitHelper::new(3)?;
        let file_paths = helper.list()?;
        println!("{:?}", file_paths);
        println!("{:?}", helper.checkout_to_temp_branch()?);
        println!("{:?}", helper.reset()?);
        let msg = "test".to_owned();
        let commit_cands = vec![GitCommitCandidate { msg, file_paths }];
        let outputs = helper.commit(&commit_cands)?;
        assert_eq!(outputs.len(), commit_cands.len());
        for output in outputs {
            println!("{:?}", output);
        }
        let output = helper.restore_branch()?;

        assert_eq!(branch_name, GitHelper::get_current_branch_name()?);

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
