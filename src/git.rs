pub mod vcs {
    use std::path::Path;
    use std::{fs::remove_dir_all, process::Command};

    pub struct Git {
        repository: String,
        url: String,
    }

    impl Git {
        ///
        /// Git constructor
        ///
        /// - `repo`    The repository install path
        /// - `uri`     The repository remote url
        ///
        /// ```
        /// use installation_testing::git::vcs::Git;
        /// use std::process::Command;
        ///
        /// let mut cargo = Command::new("cargo");
        ///
        /// let test = cargo.arg("build");
        /// let mut checker =  Git::new("https://github.com/taishingi/zuu","/tmp/zuu");
        ///
        /// checker.run(test).clean();
        /// ```
        ///
        pub fn new(uri: &str, repo: &str) -> Git {
            Self {
                repository: repo.to_string(),
                url: uri.to_string(),
            }
            .get()
        }

        ///
        /// # Get the remote repository
        ///
        fn get(self) -> Git {
            if Path::new(self.repository.as_str()).is_dir() {
                remove_dir_all(self.repository.as_str())
                    .expect("failed to remove the last repository");
            }
            assert!(Command::new("git")
                .arg("clone")
                .arg(self.url.as_str())
                .arg(self.repository.as_str())
                .spawn()
                .expect("Failed to clone repository")
                .wait()
                .expect("msg")
                .success());

            self
        }

        ///
        /// # Execute the command in the repository
        ///
        /// - `command` The command instance to use
        ///
        pub fn run(&mut self, command: &mut Command) -> &mut Git {
            command
                .current_dir(self.repository.as_str())
                .spawn()
                .expect("failed")
                .wait()
                .expect("");
            self
        }

        ///
        /// # remove the local repository
        ///
        pub fn clean(&mut self) -> bool {
            remove_dir_all(self.repository.as_str()).expect("Failed to remove the directory");
            true
        }
    }
}
