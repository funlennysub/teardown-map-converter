use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};

use regex::Regex;

use crate::utils::{find_teardown_folder, TXT_NAME, VOX_FOLDER_NAME};

pub struct NewMod {
  pub(crate) name: String,
  pub(crate) author: String,
  pub(crate) desc: String,
  pub(crate) vox_files: Vec<PathBuf>,
  pub(crate) xml_file: Option<PathBuf>,
}

impl NewMod {
  pub fn create_mod(&mut self) -> std::io::Result<()> {
    self.name = if self.name.trim().len() == 0 { "Converted map".to_string() } else { self.name.trim().to_string() };
    self.author = if self.author.trim().len() == 0 { "Map converter by funlennysub#6727".to_string() } else { self.author.trim().to_string() };
    self.desc = if self.desc.trim().len() == 0 { "No description".to_string() } else { self.desc.trim().to_string() };

    /* Teardown folder */
    let teardown_mods_dir = find_teardown_folder();
    if let Some(teardown_mods_dir) = teardown_mods_dir.as_ref() {
      println!("Teardown path for mods is {}", teardown_mods_dir.display());
    }
    let idk_why_but_rust_asked_for_this_thing = teardown_mods_dir.unwrap();
    let main_path = idk_why_but_rust_asked_for_this_thing.display();

    fs::create_dir_all(format!("{}/mods/{}/{}", main_path, self.name, VOX_FOLDER_NAME))?;
    let mut info_txt = File::create(format!("{}/mods/{}/{}.txt", main_path, self.name, TXT_NAME))?;
    info_txt.write_all(format!("name = {}\nauthor = {}\ndescription = {}", self.name, self.author, self.desc).as_ref())?;
    info_txt.sync_data()?;

    let vox_file_name_regex = Regex::new(r"(?:(?:.*?)\\)+(?P<name>.*?)\.vox$").unwrap();

    for vox in &self.vox_files {
      let caps = vox_file_name_regex.captures(vox.as_path().to_str().unwrap()).unwrap();
      fs::copy(vox.as_path(), format!("{}/mods/{}/{}/{}.vox", main_path, self.name, VOX_FOLDER_NAME, &caps["name"]))?;
      println!("{}.vox location: {}", &caps["name"], format!("{}\\mods\\{}\\{}\\{}.vox", main_path, self.name, VOX_FOLDER_NAME, &caps["name"]));
    }
    fs::copy(self.xml_file.as_ref().unwrap(), format!("{}/mods/{}/main.xml", main_path, self.name))?;
    Ok(())
  }
}