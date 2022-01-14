use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use structopt::StructOpt;

pub mod internal;

use internal::converters;

#[derive(StructOpt)]
struct CliArgs {
    #[structopt(
        parse(from_os_str),
        short = "f",
        help = "Read from a file instead from stdin"
    )]
    input_path: Option<std::path::PathBuf>,
    #[structopt(
        parse(from_os_str),
        short = "o",
        help = "Write to a file instead of stdout"
    )]
    output_path: Option<std::path::PathBuf>,
}

fn open_src_file(path: &str) -> BufReader<File> {
    let src_file = File::open(path).unwrap();
    let src_stream = BufReader::new(src_file);
    return src_stream;
}

fn open_dst_file(path: &str) -> BufWriter<File> {
    let dst_file = File::create(path).unwrap();
    let dst_stream = BufWriter::new(dst_file);
    return dst_stream;
}

fn main() {
    // TODO handle errors better than using 'unwrap'
    let args = CliArgs::from_args();

    let src_stream: Box<dyn io::BufRead> = match args.input_path {
        None => Box::new(BufReader::new(io::stdin())),
        Some(file_name) => Box::new(open_src_file(
            file_name.into_os_string().into_string().unwrap().as_str(),
        )),
    };

    let mut dst_stream: Box<dyn io::Write> = match args.output_path {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(file_name) => Box::new(open_dst_file(
            file_name.into_os_string().into_string().unwrap().as_str(),
        )),
    };

    converters::process(src_stream, &mut dst_stream);
}
