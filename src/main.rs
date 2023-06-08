use std::str::FromStr;
mod chunk;
mod args;
mod chunk_type;
mod commands;
mod png;


use clap::Parser;
use crate::png::Png;
use crate::chunk_type::ChunkType; 
use crate::chunk::Chunk;

use crate::args::Cli;
use crate::args::Commands;



pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main() -> Result<()>{
//  let args:Vec<String> =env::args().collect();
//  dbg!(args);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode(args) =>{
            let input_file = &args.file_path;
            let chunk_type = ChunkType::from_str(&args.chunk_type.as_str())?;
            let message = &args.message.clone().into_bytes();
            let message_chunk = Chunk::new(chunk_type, message.to_vec());

            let mut png = Png::from_file(input_file).unwrap();
            png.append_chunk(message_chunk);

            if let Some(output_path) = &args.output{
                std::fs::write(output_path, png.as_bytes()).unwrap();
            }
        },
        Commands::Decode(args) => {
            let input_file = &args.file_path;
            let chunk_type  = args.chunk_type.as_str();
            let png = Png::from_file(input_file).unwrap();

            let message = png.data_string_by_type(chunk_type);

            match message {
                None => { println!("no such message for chunk_type: {}.", chunk_type) },
                Some(msg) => {
                    println!("secret msg for {} is: {}", chunk_type, msg)
                }
            }

        },
        Commands::Print(args)=> {
            let input_file = &args.file_path;
            let png = Png::from_file(input_file).unwrap();
            let v = &png.chunks();
            println!("====================all chunk type({})====================", v.len());  
            for i in 0..v.len() {
                print!("{}, {}", i,v[i].chunk_type().to_string());
            }
        }
        Commands::Remove(args) => {
            let input_file = &args.file_path;
            let chunk_type = &args.chunk_type.as_str();
            let mut png = Png::from_file(input_file).unwrap();
            let chunk_removed = png.remove_chunk(chunk_type).unwrap();
            std::fs::write(input_file, png.as_bytes()).unwrap();
            println!("remove chunk type: {}", chunk_removed);
        },


    
    }
    Ok(())

}
