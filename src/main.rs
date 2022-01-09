use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::from_args();

    println!("input={:?}, output={:?}", &args.input_path, &args.output_path);
}
