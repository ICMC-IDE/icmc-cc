use crate::gen_ir::{Function, IROp, IR};
use crate::{Scope, Var, REGS_N};

const REGS: [&str; REGS_N] = ["r1", "r2", "r3", "r4", "r5", "r6", "r7"];

enum CMPS {
    EQ,
    NE,
    LT,
    LE,
}

use std::sync::Mutex;

lazy_static! {
    static ref LABEL: Mutex<usize> = Mutex::new(0);
}

macro_rules! emit{
    ($fmt:expr) => (print!(concat!("\t", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("\t", $fmt, "\n"), $($arg)*));
}

fn emit_cmp(ir: IR, cmp: CMPS) {
    let lhs = ir.lhs.unwrap();
    let rhs = ir.rhs.unwrap();

    emit!("cmp {}, {}", REGS[lhs], REGS[rhs]);
    emit!("push fr");
    emit!("pop {}", REGS[lhs]);

    match cmp {
        CMPS::EQ => emit!("loadn r7, #{}", 0b100),
        CMPS::NE => emit!("loadn r7, #{}", 0),
        CMPS::LT => emit!("loadn r7, #{}", 0b10),
        CMPS::LE => emit!("loadn r7, #{}", 0b110),
    }

    emit!("and {}, {}, r7", REGS[lhs], REGS[lhs]);
}

fn gen(f: Function) {
    use self::IROp::*;
    let mut last_cmp: Option<CMPS> = None;
    let ret = format!("Lend{}", *LABEL.lock().unwrap());
    *LABEL.lock().unwrap() += 1;

    println!("F{}:", f.name);

    if f.stacksize > 0 {
        emit!("push r0");
        emit!("mov r0, sp");
        emit!("loadn r7, #{}", f.stacksize + 1);
        emit!("sub r7, r0, r7");
        emit!("mov sp, r7");
    }

    for ir in f.ir {
        let lhs = ir.lhs.unwrap();
        let rhs = ir.rhs.unwrap_or(0);
        match ir.op {
            Imm => emit!("loadn {}, #{}", REGS[lhs], rhs as i16),
            Mov => emit!("mov {}, {}", REGS[lhs], REGS[rhs]),
            Return => {
                emit!("mov r7, {}", REGS[lhs]);
                emit!("jmp {}", ret);
            }
            Outchar => emit!("outchar {}, {}", REGS[lhs], REGS[rhs]),
            Inchar => emit!("inchar {}", REGS[lhs]),
            Call(name, nargs, _) => {
                for i in nargs..lhs {
                    emit!("push {}", REGS[i]);
                }
                emit!("call F{}", name);
                emit!("mov {}, r7", REGS[lhs]);
                for i in (nargs..lhs).rev() {
                    emit!("pop {}", REGS[i]);
                }
            }
            Label => println!("L{}:", lhs),
            LabelAddr(name) => emit!("loadn {}, #{}", REGS[lhs], name),
            Neg => emit!("not {}, {}", REGS[lhs], REGS[lhs]),
            EQ => {
                last_cmp = Some(CMPS::EQ);
                emit_cmp(ir, CMPS::EQ);
            }
            NE => {
                last_cmp = Some(CMPS::NE);
                emit_cmp(ir, CMPS::NE);
            } // TODO
            LT => {
                last_cmp = Some(CMPS::LT);
                emit_cmp(ir, CMPS::LT);
            }
            LE => {
                last_cmp = Some(CMPS::LE);
                emit_cmp(ir, CMPS::LE);
            }
            AND => emit!("and {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            OR => emit!("or {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            XOR => emit!("xor {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SHL => emit!("shiftl0 {}, {}", REGS[lhs], REGS[rhs]),
            SHR => emit!("shiftr0 {}, {}", REGS[lhs], REGS[rhs]),
            Mod => emit!("mod {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Jmp => emit!("jmp L{}", lhs),
            If => match last_cmp {
                Some(CMPS::EQ) => emit!("jne L{}", rhs),
                Some(CMPS::NE) => emit!("jeq L{}", rhs),
                Some(CMPS::LT) => emit!("jeg L{}", rhs),
                Some(CMPS::LE) => emit!("jgr L{}", rhs),
                _ => unreachable!(),
            },
            Unless => match last_cmp {
                Some(CMPS::EQ) => emit!("jeq L{}", rhs),
                Some(CMPS::NE) => emit!("jne L{}", rhs),
                Some(CMPS::LT) => emit!("jle L{}", rhs),
                Some(CMPS::LE) => emit!("jel L{}", rhs),
                _ => unreachable!(),
            },
            Load(_) => emit!("loadi {}, {}", REGS[lhs], REGS[rhs]),
            Store(_) => emit!("storei {}, {}", REGS[lhs], REGS[rhs]),
            StoreArg(_) => {
                emit!("loadn r7, #{}", lhs);
                emit!("sub r7, r0, r7");
                emit!("storei r7, {}", REGS[rhs]);
            }
            Add => emit!("add {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            AddImm => {
                if rhs == 0 {
                    continue;
                }
                if rhs == 1 {
                    emit!("inc {}", REGS[lhs]);
                } else {
                    emit!("loadn {}, #{}", REGS[lhs + 1], rhs as i16);
                    emit!("add {}, {}, {}", REGS[lhs], REGS[lhs], REGS[lhs + 1]);
                }
            }
            Sub => emit!("sub {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SubImm => {
                if rhs == 0 {
                    continue;
                }
                if rhs == 1 {
                    emit!("dec {}", REGS[lhs]);
                } else {
                    emit!("loadn {}, #{}", REGS[lhs + 1], rhs as i16);
                    emit!("sub {}, {}, {}", REGS[lhs], REGS[lhs], REGS[lhs + 1]);
                }
            }
            Bprel => {
                emit!("loadn {}, #{}", REGS[lhs], rhs);
                emit!("sub {}, r0, {}", REGS[lhs], REGS[lhs]);
            }
            Mul => emit!("mul {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            MulImm => {
                if rhs < 256 && rhs.count_ones() == 1 {
                    emit!("shiftl0 {}, {}", REGS[lhs], rhs.trailing_zeros());
                } else {
                    emit!("loadn r7, #{}", rhs as i16);
                    emit!("mul {}, {}, r7", REGS[lhs], REGS[lhs]);
                }
            }
            Div => emit!("div {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Nop | Kill => (),
        }
    }

    println!("{}:", ret);
    if f.stacksize > 0 {
        emit!("mov sp, r0");
        emit!("pop r0")
    }
    emit!("rts");
}

pub fn gen_asm(globals: Vec<Var>, fns: Vec<Function>) {
    println!("call Fmain");
    println!("halt");

    for f in fns {
        gen(f);
    }

    for var in globals {
        if let Scope::Global(data, len, is_extern) = var.scope {
            if is_extern {
                continue;
            }

            if data.len() > 0 {
                println!("{} : string \"{}\"", var.name, data);
            } else {
                println!("{} : var #{}", var.name, len);
            }

            continue;
        }
        unreachable!();
    }
}
