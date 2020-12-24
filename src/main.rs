use native_dialog::FileDialog;

use crate::convert::NewMod;
use crate::utils::create_question;

mod convert;
mod utils;

fn main() -> std::io::Result<()> {
  println!("Mod folder name will be the same as map name you enter.\n");
  /* Create txt file with mod info */
  let map_name = create_question(String::from("Map name:"));
  let map_author = create_question(String::from("\nMap author:"));
  let map_desc = create_question(String::from("\nMap description:"));

  println!("Select '.vox' files you want to convert.");
  let vox_file_dialog = FileDialog::new()
      .add_filter(".vox files", &["vox"])
      .show_open_multiple_file()
      .unwrap();
  println!("Select your main xml file.");
  let xml_file_dialog = FileDialog::new()
      .add_filter("XML file", &["xml"])
      .show_open_single_file()
      .unwrap();

  NewMod::create_mod(&mut NewMod {
    name: map_name,
    author: map_author,
    desc: map_desc,
    vox_files: (*vox_file_dialog).to_vec(),
    xml_file: xml_file_dialog,
  })
}