use crate::gen_ir::{Function, IROp, IR};
use crate::{Scope, Var, REGS_N};
use std::io::Write;

const REGS: [&str; REGS_N] = ["r1", "r2", "r3", "r4", "r5", "r6", "r7"];

use std::sync::Mutex;

lazy_static! {
    static ref LABEL: Mutex<usize> = Mutex::new(0);
}

enum CMPS {
    EQ,
    NE,
    LT,
    LE,
}

macro_rules! emit{
    ($out:expr, $fmt:expr) => (write!($out, concat!("\t", $fmt, "\n")).unwrap());
    ($out:expr, $fmt:expr, $($arg:tt)*) => (write!($out, concat!("\t", $fmt, "\n"), $($arg)*).unwrap());
}

fn emit_cmp(output: &mut impl Write, ir: IR, cmp: CMPS) {
    let lhs = ir.lhs.unwrap();
    let rhs = ir.rhs.unwrap();

    emit!(output, "cmp {}, {}", REGS[lhs], REGS[rhs]);
    emit!(output, "push fr");
    emit!(output, "pop {}", REGS[lhs]);

    let mask: u16 = match cmp {
        CMPS::EQ => 0b100,
        CMPS::NE => {
            emit!(output, "not {}, {}", REGS[lhs], REGS[lhs]);
            0b100
        }
        CMPS::LT => 0b10,
        CMPS::LE => 0b110,
    };

    emit!(output, "loadn r7, #{}", mask);
    emit!(output, "and {}, {}, r7", REGS[lhs], REGS[lhs]);
}

fn gen(output: &mut impl Write, f: Function) {
    use self::IROp::*;
    let ret = format!("Lend{}", *LABEL.lock().unwrap());
    *LABEL.lock().unwrap() += 1;

    writeln!(output, "{}:", f.name).unwrap();

    if f.stacksize > 0 {
        emit!(output, "push r0");
        emit!(output, "mov r0, sp");
        emit!(output, "loadn r7, #{}", f.stacksize + 1);
        emit!(output, "sub r7, r0, r7");
        emit!(output, "mov sp, r7");
    }

    for ir in f.ir {
        let lhs = ir.lhs.unwrap();
        let rhs = ir.rhs.unwrap_or(0);
        match ir.op {
            Imm => emit!(output, "loadn {}, #{}", REGS[lhs], rhs as u16),
            Mov => emit!(output, "mov {}, {}", REGS[lhs], REGS[rhs]),
            Return => {
                emit!(output, "mov r7, {}", REGS[lhs]);
                emit!(output, "jmp {}", ret);
            }
            Outchar => emit!(output, "outchar {}, {}", REGS[lhs], REGS[rhs]),
            Inchar => emit!(output, "inchar {}", REGS[lhs]),
            Call(name, nargs, _) => {
                for i in nargs..lhs {
                    emit!(output, "push {}", REGS[i]);
                }
                emit!(output, "call {}", name);
                emit!(output, "mov {}, r7", REGS[lhs]);
                for i in (nargs..lhs).rev() {
                    emit!(output, "pop {}", REGS[i]);
                }
            }
            Label => writeln!(output, "L{}:", lhs).unwrap(),
            LabelAddr(name) => emit!(output, "loadn {}, #{}", REGS[lhs], name),
            Neg => {
                emit!(output, "not {}, {}", REGS[lhs], REGS[lhs]);
                emit!(output, "inc {}", REGS[lhs]);
            }
            EQ => emit_cmp(output, ir, CMPS::EQ),
            NE => emit_cmp(output, ir, CMPS::NE), // TODO
            LT => emit_cmp(output, ir, CMPS::LT),
            LE => emit_cmp(output, ir, CMPS::LE),
            AND => emit!(output, "and {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            OR => emit!(output, "or {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            XOR => emit!(output, "xor {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SHL => emit!(output, "shiftl0 {}, {}", REGS[lhs], REGS[rhs]),
            SHR => emit!(output, "shiftr0 {}, {}", REGS[lhs], REGS[rhs]),
            Mod => emit!(output, "mod {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Jmp => emit!(output, "jmp L{}", lhs),
            If => emit!(output, "jnz L{}", rhs),
            Unless => emit!(output, "jz L{}", rhs),
            Load(_) => emit!(output, "loadi {}, {}", REGS[lhs], REGS[rhs]),
            Store(_) => emit!(output, "storei {}, {}", REGS[lhs], REGS[rhs]),
            StoreArg(_) => {
                emit!(output, "loadn r7, #{}", lhs);
                emit!(output, "sub r7, r0, r7");
                emit!(output, "storei r7, {}", REGS[rhs]);
            }
            Add => emit!(output, "add {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            AddImm => {
                if rhs == 0 {
                    continue;
                }
                if rhs == 1 {
                    emit!(output, "inc {}", REGS[lhs]);
                } else {
                    emit!(output, "loadn {}, #{}", REGS[lhs + 1], rhs as u16);
                    emit!(
                        output,
                        "add {}, {}, {}",
                        REGS[lhs],
                        REGS[lhs],
                        REGS[lhs + 1]
                    );
                }
            }
            Sub => emit!(output, "sub {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            SubImm => {
                if rhs == 0 {
                    continue;
                }
                if rhs == 1 {
                    emit!(output, "dec {}", REGS[lhs]);
                } else {
                    emit!(output, "loadn {}, #{}", REGS[lhs + 1], rhs as u16);
                    emit!(
                        output,
                        "sub {}, {}, {}",
                        REGS[lhs],
                        REGS[lhs],
                        REGS[lhs + 1]
                    );
                }
            }
            Bprel => {
                emit!(output, "loadn {}, #{}", REGS[lhs], rhs);
                emit!(output, "sub {}, r0, {}", REGS[lhs], REGS[lhs]);
            }
            Mul => emit!(output, "mul {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            MulImm => {
                if rhs < 256 && rhs.count_ones() == 1 {
                    emit!(output, "shiftl0 {}, #{}", REGS[lhs], rhs.trailing_zeros());
                } else {
                    emit!(output, "loadn r7, #{}", rhs as u16);
                    emit!(output, "mul {}, {}, r7", REGS[lhs], REGS[lhs]);
                }
            }
            Div => emit!(output, "div {}, {}, {}", REGS[lhs], REGS[lhs], REGS[rhs]),
            Nop | Kill => (),
        }
    }

    writeln!(output, "{}:", ret).unwrap();
    if f.stacksize > 0 {
        emit!(output, "mov sp, r0");
        emit!(output, "pop r0")
    }
    emit!(output, "rts");
}

pub fn gen_asm(output: &mut impl Write, globals: Vec<Var>, fns: Vec<Function>) {
    writeln!(output, "call main").unwrap();
    writeln!(output, "halt").unwrap();

    for f in fns {
        gen(output, f);
    }

    for var in globals {
        if let Scope::Global(data, len, is_extern) = var.scope {
            if is_extern {
                continue;
            }

            if data.len() > 0 {
                writeln!(output, "{} : string \"{}\"", var.name, data).unwrap();
            } else {
                writeln!(output, "{} : var #{}", var.name, len).unwrap();
            }

            continue;
        }
        unreachable!();
    }
}
