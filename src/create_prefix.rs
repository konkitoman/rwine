use std::path::PathBuf;

use crate::reactor::Reactor;

pub fn create_prefix() {
    let mut reactor = Reactor::default();

    let user_dir: std::path::PathBuf =
        PathBuf::from("~/.rwine/c:/users").join(dirs::home_dir().unwrap().file_name().unwrap());

    std::fs::create_dir("~/.rwine");
    std::fs::create_dir("~/.rwine/c:/");
    std::fs::create_dir("~/.rwine/c:/windows");
    std::fs::create_dir("~/.rwine/c:/windows/system32");
    std::fs::create_dir("~/.rwine/c:/windows/syswow64");
    std::fs::create_dir("~/.rwine/c:/windows/temp");
    std::fs::create_dir("~/.rwine/c:/windows/Fonts");
    std::fs::create_dir("~/.rwine/c:/users");
    std::fs::create_dir("~/.rwine/c:/ProgramData");
    std::fs::create_dir("~/.rwine/c:/ProgramFiles");
    std::fs::create_dir("~/.rwine/c:/ProgramFiles (x86)");

    std::fs::create_dir(&user_dir);
    std::fs::create_dir(user_dir.join("Contacts"));
    std::fs::create_dir(user_dir.join("Downloads"));
    std::fs::create_dir(user_dir.join("Desktop"));
    std::fs::create_dir(user_dir.join("Documents"));
    std::fs::create_dir(user_dir.join("AppData"));
    std::fs::create_dir(user_dir.join("Favorites"));
    std::fs::create_dir(user_dir.join("Links"));
    std::fs::create_dir(user_dir.join("Music"));
    std::fs::create_dir(user_dir.join("Pictures"));
    std::fs::create_dir(user_dir.join("Temp"));
    std::fs::create_dir(user_dir.join("Videos"));

    reactor.run();
}
