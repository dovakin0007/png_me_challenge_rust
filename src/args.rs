use std::{path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand, Command,  value_parser};


use crate::{chunk_type::ChunkType, commands ,};



#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs{

    ///Png File to store message
    #[clap(value_parser)]
    pub file_path:PathBuf,

    /// Chunk type for storing message
    #[clap(value_parser)]
    pub chunk_type: String,

    /// Message to be stored
    #[clap(value_parser)]
     pub message: String,

    /// Write the output PNG file to specific location
    #[clap(value_parser)]
    pub output: Option<PathBuf>,


}
#[derive(Debug, Args)]
pub struct DecodeArgs{
        ///Png File where message is encoded
        #[clap(value_parser)]
        pub file_path:PathBuf,

        /// Chunk type to decode the message
        #[clap(value_parser)]
        pub chunk_type: String,
    

}

#[derive(Debug, Args)]
pub struct RemoveArgs{
            ///Png File where the file located that wants to be removed
            #[clap(value_parser)]
            pub file_path:PathBuf,

            /// Chunk messafe to be removed
            #[clap(value_parser)]
            pub chunk_type: String,
}
#[derive(Debug, Args)]
pub struct PrintArgs{
                ///print the png file from the given path
                #[clap(value_parser)]
                pub file_path:PathBuf,

}

