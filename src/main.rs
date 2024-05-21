extern crate icmc_cc;

use icmc_cc::gen_asm::gen_asm;
use icmc_cc::gen_ir::gen_ir;
// use icmc_cc::irdump::dump_ir;
use icmc_cc::parse::parse;
use icmc_cc::preprocess::Preprocessor;
use icmc_cc::regalloc::alloc_regs;
use icmc_cc::sema::sema;
use icmc_cc::token::tokenize;

use std::fs;
use std::io::{stdin, stdout, Read, Write};

use structopt::StructOpt;

fn fs_read(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "ICMC CC", about = "C compiler for ICMC architecture")]
struct Opt {
    #[structopt(short = "-i", long = "--input")]
    fin: Option<String>,
    #[structopt(short = "-o", long = "--output")]
    fout: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let (input_data, input_file) = match opt.fin {
        Some(path) => (fs_read(path.as_str()), path),
        None => {
            let mut buf = Vec::new();
            stdin().lock().read_to_end(&mut buf).unwrap();
            (String::from_utf8(buf).unwrap(), "".to_string())
        }
    };

    let tokens = tokenize(
        input_data,
        input_file,
        &mut Preprocessor::new(Box::new(fs_read)),
    );

    let nodes = parse(&tokens);
    let (nodes, globals) = sema(nodes);
    let mut fns = gen_ir(nodes);

    alloc_regs(&mut fns);

    let mut output: Box<dyn Write> = match opt.fout {
        Some(path) => Box::new(fs::File::create(path).unwrap()),
        None => Box::new(stdout()),
    };
    gen_asm(&mut output, globals, fns);
}
