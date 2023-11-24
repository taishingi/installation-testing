pub mod vcs {
    use std::path::Path;
    use std::{fs::remove_dir_all, process::Command};

    pub struct Hg {
        repository: String,
        url: String,
    }

    impl Hg {
        ///
        /// Git constructor
        ///
        /// - `repo`    The repository install path
        /// - `uri`     The repository remote url
        ///
        /// ```
        ///
        /// use std::process::Command;
        /// use installation_testing::hg::vcs::Hg;
        ///
        /// let mut cargo = Command::new("ls");
        ///
        /// let test = cargo.arg("-l");
        /// let mut checker =  Hg::new("https://www.selenic.com/repo/hello","/tmp/zuul");
        ///
        /// checker.run(test).clean();
        /// ```
        ///
        pub fn new(uri: &str, repo: &str) -> Hg {
            Self {
                repository: repo.to_string(),
                url: uri.to_string(),
            }
            .get()
        }

        ///
        /// # Get the remote repository
        ///
        fn get(self) -> Hg {
            if Path::new(self.repository.as_str()).is_dir() {
                remove_dir_all(self.repository.as_str())
                    .expect("failed to remove the last repository");
            }
            assert!(Command::new("hg")
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
        /// # Run the command inside the repository
        ///
        /// - `command` The command instance to run
        ///
        pub fn run(&mut self, command: &mut Command) -> &mut Hg {
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
