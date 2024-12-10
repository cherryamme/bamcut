#![allow(warnings)]
pub mod bam;
pub mod args;
use clap::Parser;
use log::{info,debug};

use bam::process_bam;

fn main() {
    // std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let comands: Vec<String> = std::env::args().collect();
    let mut args = args::Args::parse();
    info!("Run Command: {:?}", comands);
    info!("Args: {:?}", args);

    // let file= "/home/jiangchen/project/Jolish/example/example.fq.bam".to_string();
    // let outfile= "/home/jiangchen/project/Jolish/example/example_extract.fq.gz".to_string();
    process_bam(args.inputs, args.outfile,args.region, args.max_reads);
}
