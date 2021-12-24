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