extern crate getopts;
use getopts::Options;
use std::path::Path;
use std::env;

static DEFAULT_NTHREADS: i32 = 1;
static DEFAULT_NITEMS: i32 = 1<<10;

fn print_usage(program: &str, opts: Options) {
    let path = Path::new(program).file_name();
    let brief = format!("Usage: {:?} [options]", path.unwrap());
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("n","num-items","Number of items to sort","ITEMS");
    opts.optopt("t","num-threads","Number of threads to use","THREADS");
    opts.optflag("h","help","Show this message and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut nthreads = DEFAULT_NTHREADS;
    let mut nitems = DEFAULT_NITEMS;

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("n") {
        nitems = matches.opt_str("n").unwrap()
                        .parse::<i32>().unwrap();
    }
    if matches.opt_present("t") {
        nthreads = matches.opt_str("t").unwrap()
                          .parse::<i32>().unwrap();
    }
    print!("{} {}\n",nitems,nthreads);
}
