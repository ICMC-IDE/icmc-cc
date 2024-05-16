extern crate icmc_cc;

use icmc_cc::gen_asm::gen_asm;
use icmc_cc::gen_ir::gen_ir;
use icmc_cc::irdump::dump_ir;
use icmc_cc::parse::parse;
use icmc_cc::preprocess::Preprocessor;
use icmc_cc::regalloc::alloc_regs;
use icmc_cc::sema::sema;
use icmc_cc::token::tokenize;

use std::env;
use std::process;

fn usage() -> ! {
    eprintln!("Usage: icmc-cc [-dump-ir1] [-dump-ir2] <file>");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
    }

    let mut dump_ir1 = false;
    let mut dump_ir2 = false;
    let path;

    if args.len() == 3 && args[1] == "-dump-ir1" {
        dump_ir1 = true;
        path = args[2].clone();
    } else if args.len() == 3 && args[1] == "-dump-ir2" {
        dump_ir2 = true;
        path = args[2].clone();
    } else {
        if args.len() != 2 {
            usage();
        }
        path = args[1].clone();
    }

    // Tokenize and parse.
    let tokens = tokenize(path, &mut Preprocessor::new());

    let nodes = parse(&tokens);
    let (nodes, globals) = sema(nodes);
    let mut fns = gen_ir(nodes);

    if dump_ir1 {
        dump_ir(&fns);
    }

    alloc_regs(&mut fns);

    if dump_ir2 {
        dump_ir(&fns);
    }

    gen_asm(globals, fns);
}
