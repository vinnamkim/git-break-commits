#[cfg(test)]
mod tests {
    use std::{
        fs::{self, create_dir, File},
        io::{Error, Write},
        process::Command,
    };

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn my_test() -> Result<(), std::io::Error> {
        let temp_dir = tempdir()?;
        let dir_names = ["dir_1", "dir_2", "dir_3"];
        let file_names = ["commit_1", "commit_2", "commit_3"];

        for dir_name in dir_names {
            let dir_path = temp_dir.path().join(dir_name);
            create_dir(&dir_path)?;

            for fname in file_names {
                let mut file = File::create(dir_path.join(fname))?;
                let _ = file.write(b"")?;
            }
        }

        let init = command_git(&temp_dir).arg("init").output()?;
        let set_author_name = command_git(&temp_dir)
            .args(["config", "--local", "user.name", "Anonymous"])
            .output()?;
        let set_author_email = command_git(&temp_dir)
            .args([
                "config",
                "--local",
                "user.email",
                "anonymous@anonymous.com",
            ])
            .output()?;

        for fname in file_names {
            let mut commit_file_list = vec![];
            for dir_name in dir_names {
                commit_file_list
                    .push(temp_dir.path().join(dir_name).join(fname));
            }
            let add = command_git(&temp_dir)
                .arg("add")
                .args(commit_file_list)
                .output()?;

            let commit = command_git(&temp_dir)
                .arg("commit")
                .args(["-m", fname])
                .output()?;
        }

        let log = command_git(&temp_dir).arg("log").output()?;
        println!("{:?}", log);

        Ok(())
    }

    fn command_git(temp_dir: &tempfile::TempDir) -> Command {
        let mut command = Command::new("git");
        command.current_dir(temp_dir);
        command
    }
}
