use crate::gen_ir::{Function, IROp, IR};
use crate::{Scope, Var, REGS_N};

const REGS: [&str; REGS_N] = ["r1", "r2", "r3", "r4", "r5", "r6", "r7"];

use std::sync::Mutex;

lazy_static! {
    static ref LABEL: Mutex<usize> = Mutex::new(0);
}

macro_rules! emit{
    ($fmt:expr) => (print!(concat!("\t", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!("\t", $fmt, "\n"), $($arg)*));
}

fn emit_cmp(ir: IR, cmp: u8) {
    let lhs = ir.lhs.unwrap();
    let rhs = ir.rhs.unwrap();
    emit!("cmp {}, {}", REGS[lhs], REGS[rhs]);
    emit!("push fr");
    emit!("pop {}", REGS[lhs]);
    emit!("loadn r0, #{}", cmp);
    emit!("and {}, {}, r0", REGS[lhs], REGS[lhs]);
}

fn gen(f: Function) {
    use self::IROp::*;
    let ret = format!("Lend{}", *LABEL.lock().unwrap());
    *LABEL.lock().unwrap() += 1;

    println!("{}:", f.name);

    for ir in f.ir {
        let lhs = ir.lhs.unwrap();
        let rhs = ir.rhs.unwrap_or(0);
        match ir.op {
            Imm => emit!("loadn {}, #{}", REGS[lhs], rhs as i32),
            Mov => emit!("mov {}, {}", REGS[lhs], REGS[rhs]),
            Return => {
                emit!("mov r0, {}", REGS[lhs]);
                emit!("jmp {}", ret);
            }
            Outchar => emit!("outchar {}, {}", REGS[lhs], REGS[rhs]),
            Call(name, _, _) => {
                // for i in 0..nargs {
                //     emit!("mov {}, {}", ARGREGS[i], REGS[args[i]]);
                // }
                // emit!("push r10");
                // emit!("push r11");
                // emit!("mov rax, 0");
                emit!("call {}", name);
                // emit!("pop r11");
                // emit!("pop r10");
                emit!("mov {}, r0", REGS[lhs]);
            }
            Label => println!("L{}:", lhs),
            LabelAddr(name) => emit!("loadn {}, #{}", REGS[lhs], name),
            Neg => emit!("not {}", REGS[lhs]),
            EQ => emit_cmp(ir, 0b100),
            NE => emit_cmp(ir, 0),
            LT => emit_cmp(ir, 0b10),
            LE => emit_cmp(ir, 0),
            AND => emit!("and {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            OR => emit!("or {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            XOR => emit!("xor {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SHL => emit!("shiftl0 {}, {}", REGS[lhs], REGS[rhs]),
            SHR => emit!("shiftr0 {}, {}", REGS[lhs], REGS[rhs]),
            Mod => emit!("mod {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Jmp => emit!("jmp L{}", lhs),
            If => emit!("jnz L{}", rhs),
            Unless => emit!("jz L{}", rhs),
            Load(_) => emit!("loadi {}, {}", REGS[lhs], REGS[rhs]),
            Store(_) => emit!("storei {}, {}", REGS[lhs], REGS[rhs]),
            StoreArg(_) => {
                emit!("mov [rbp-{}], {}", lhs, rhs);
            }
            Add => emit!("add {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            AddImm => {
                if rhs == 1 {
                    emit!("inc {}", REGS[lhs]);
                } else {
                    emit!("loadn r0, #{}", rhs as i16);
                    emit!("add {}, {}, r0", REGS[lhs], REGS[lhs]);
                }
            }
            Sub => emit!("sub {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SubImm => {
                if rhs == 1 {
                    emit!("dec {}", REGS[lhs]);
                } else {
                    emit!("loadn r0, #{}", rhs as i32);
                    emit!("sub {}, {}, r0", REGS[lhs], REGS[lhs]);
                }
            }
            Bprel => emit!("lea {}, [rbp-{}]", REGS[lhs], rhs),
            Mul => emit!("mul {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            MulImm => {
                if rhs < 256 && rhs.count_ones() == 1 {
                    emit!("shiftl0 {}, {}", REGS[lhs], rhs.trailing_zeros());
                } else {
                    emit!("loadn r0, #{}", rhs as i16);
                    emit!("mul {}, {}, r0", REGS[lhs], REGS[lhs]);
                }
            }
            Div => emit!("div {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Nop | Kill => (),
        }
    }

    println!("{}:", ret);
    emit!("rts");
}

pub fn gen_x86(globals: Vec<Var>, fns: Vec<Function>) {
    println!("call main");
    println!("halt");

    for f in fns {
        gen(f);
    }

    for var in globals {
        if let Scope::Global(data, len, is_extern) = var.scope {
            if is_extern {
                continue;
            }
            println!("{} : var #{}", var.name, len);

            if data.len() > 0 {
                println!("static {}, #{}", var.name, data);
            }

            continue;
        }
        unreachable!();
    }
}
