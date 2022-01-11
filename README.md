# dmg

dmg is an async Rust library for macOS that mounts a DMG file.  It is useful for sysadmin tasks.

dmg has minimal dependencies and is designed for use in any async runtime.  In particular, it is tested with
[kiruna](https://github.com/drewcrawford/kiruna).

# Example
```
async fn example() -> Result<(),Box<dyn std::error::Error>> {
    let result_path = dmg::mount(std::path::Path::new("testdata/test_compressed.dmg"), kiruna::Priority::UserWaiting).await?;
    println!("result_path {:?}",result_path);
    Ok(())
}
```

# Similar libraries

Since Rust is a compiled language, its binaries are self-contained.  Therefore you can write tools to bring up a production or
development environment in Rust itself, compile them, and shoot them over to new servers via SSH.

You might be interested my expanded universe of sysadmin libraries:

* [rustupr](https://github.com/drewcrawford/rustupr), which installs Rust
* [github-actions-runner](https://github.com/drewcrawford/github-actions-runner), which installs GitHub's action runner
* [mac-install](https://github.com/drewcrawford/mac-install) which installs mac packages