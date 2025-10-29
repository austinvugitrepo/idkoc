use clap::{Parser, Subcommand}; //clap
use std::path::PathBuf; //for file extension
//mod ext; bringing ext.rs

#[derive(Parser, Debug)] // derive works with structs, enums, etc.
#[command(version, long_version = "v0.2.0\nLicense:CC0 1.0 Universal\ncreated by: Austin Vu", about = "idkoc, the best CLI image formatter", long_about = "idkoc (I dont know or care) is a modern CLI image formatter\nwritten in Rust designed to be an all in one image tool")]

struct CliArgs {

    /// webp image format
    #[arg(short, long, value_parser)]
    webp: Option<PathBuf>,
                           //my optional arguments
    /// png image format
    #[arg(short, long, value_parser)]
    png: Option<PathBuf>,

    /// jpg image format
    #[arg(short, long, value_parser)]
    jpg: Option<PathBuf>,
    
    #[command(subcommand)]   //sub command
    command: Option<Commands>,

}

#[derive(Subcommand, Debug)] // for subcommands, seems i have to put this next to my enum, guessing
                             // by this the derive for the struct is the same
enum Commands {

  Convert {

   /// webp image format
    #[arg(short, long, value_parser)]
    webp: Option<PathBuf>,
                           //my optional arguments for convert copied from struct
    /// png image format
    #[arg(short, long, value_parser)]
    png: Option<PathBuf>,

    /// jpg image format
    #[arg(short, long, value_parser)]
    jpg: Option<PathBuf>,
    
  },


}

fn main() {

let argument = CliArgs::parse(); //argument parser
                                 //Some means it exists or whatever
if let Some(a) = &argument.webp { // reference shennigans  to see if it exists
    if !a.exists() {  // file checker 
        println!("File does not exist."); 
    } else if let Some(ext) = a.extension().and_then(|x| x.to_str()) { //file extension checker that converts <&OsStr> to &str to compare  
        if ext == "webp" {
            println!("This is a .webp file.");
        } else {
            println!("This is NOT a .webp file.");
        }
    } else { // else case if cant read file extension
        println!("Could NOT read the file extension.");
    }
}
// clone of webp function checker 
if let Some(b) = &argument.png {
    if !b.exists() {
        println!("File does not exist.");
    } else if let Some(ext) = b.extension().and_then(|x| x.to_str()) {
        if ext == "png" {
            println!("This is a .png file.");
        } else {
            println!("This is NOT a .png file.");
        }
    } else {
        println!("Could not read the file extension!");
    }
}

//clone of webp function checker
if let Some(c) = &argument.jpg {
    if !c.exists() {
        println!("File does not exist.");
    } else if let Some(ext) = c.extension().and_then(|x| x.to_str()) {
        if ext == "jpg" {
            println!("This is a .jpg file.");
        } else {
            println!("This is NOT a .jpg file.");
        }
    } else {
        println!("Could not read the file extension!");
    }
}
// sub command convert
if let Some(Commands::Convert { webp, png, jpg }) = &argument.command { //accessing reference
                                                                        //command convert
    let mut counter = 0;

    if webp.is_some() { counter += 1; }
    if png.is_some() { counter += 1; }
    if jpg.is_some() { counter += 1; }
    
    if counter < 2 {
      println!("convert subcommand needs exactly 2 images formats.");
    } else if counter == 2 {
        println!("Hello world!");
    } else {
        println!("convert subcommand needs exactly 2 image formats.");
    }



}


  }
