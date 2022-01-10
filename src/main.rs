use structopt::StructOpt;

pub mod internal;

use internal::converters;

#[derive(StructOpt)]
struct CliArgs {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::from_args();

    println!(
        "input={:?}, output={:?}",
        &args.input_path, &args.output_path
    );

    converters::process(
        args.input_path.into_os_string().into_string().unwrap().as_str(),
        args.output_path.into_os_string().into_string().unwrap().as_str()
    );
}
