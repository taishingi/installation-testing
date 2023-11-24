# Installation testing 

A program to test installation


## Installation

```bash
cargo add installation-testing
```

### Example 

```rust
    use installation_testing::git::vcs::Git;
    use std::process::Command;

    let mut cargo = Command::new("cargo");

    let test = cargo.arg("build");
    let x =  Git::new("https://github.com/taishingi/zuu","/tmp/zuu"); // Clone directly the repository

    assert!(x.run(test).clean());  
```


