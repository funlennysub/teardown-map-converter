use std::io;
use std::path::PathBuf;

/* May change */
pub static TXT_NAME: &'static str = "info";
pub static VOX_FOLDER_NAME: &'static str = "vox";

/* Get teardown folder */
#[cfg(target_os = "windows")]
pub fn find_teardown_folder() -> Option<PathBuf> {
  let teardown_mods_dir = PathBuf::from(std::env::var("USERPROFILE").ok()?)
      .join("Documents")
      .join("Teardown");
  if teardown_mods_dir.is_dir() {
    Some(teardown_mods_dir)
  } else {
    None
  }
}

/* Create question */
pub fn create_question(question: String) -> String {
  let mut data = String::new();
  println!("{}", question);
  io::stdin().read_line(&mut data).unwrap();
  data.trim().parse().unwrap()
}