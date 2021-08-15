use std::path::{Path, PathBuf};
use core_foundationr::{CFData, CFPropertyList, CFTypeBehavior, StrongCell, CFString, CFArray, CFDictionary};
use std::ffi::c_void;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    CFError(StrongCell<core_foundationr::CFError>)
}


pub async fn mount(path: &Path,priority: kiruna::Priority) -> Result<PathBuf, Error> {
    let output = command_rs::Command::new("hdiutil").arg("mount").arg(path.as_os_str()).arg("-plist").output(priority)
        .await.map_err(|e| Error::IOError(e))?;
    let dispatch_data = output.stdout.as_dispatch_data();
    //dispatch_data is bridged with cfdata
    let cfdata = unsafe{ CFData::from_ref(&*(dispatch_data as *const _ as *const c_void)) };
    let plist = CFPropertyList::from_data(cfdata).map_err(|e| Error::CFError(e))?;
    let dictionary: StrongCell<CFDictionary> = plist.cast_checked();
    //There are, I think, varoius extra copies in here for things that could be static strings etc
    let strong_str = CFString::from_str("system-entities");
    let borrow: &CFString = &strong_str;
    let system_entities = dictionary.get_with_key(borrow);
    let array: &CFArray = system_entities.unwrap().checked_cast();
    let r = array.iter().find_map(|p| {
        let d: &CFDictionary = p.checked_cast();
        d.get_with_key(&*CFString::from_str("mount-point"))
    });
    let str_mount_point: &CFString = r.unwrap().checked_cast();
    use std::str::FromStr;
    let s = PathBuf::from_str(&str_mount_point.as_string()).unwrap();
    return Ok(s);
}

#[test] fn test() {
    use kiruna::test::test_await;
    let f = mount(Path::new("testdata/test_compressed.dmg"),kiruna::Priority::Testing);
    let result = test_await(f, std::time::Duration::from_secs(10));
    let result2 = result.unwrap();
    assert_eq!(result2.to_str().unwrap(), "/Volumes/test");
}