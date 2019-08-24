use std::fs::File;
use std::io::prelude::*;

use super::*;

pub fn compile(module: &Module) -> std::io::Result<()> {
    let mut file = File::create("wasm/_.wasm")?;

    let magic = magic();
    file.write_all(&magic);

    let version = version();
    file.write_all(&version);

    let typesec = typesec(module);
    file.write_all(&typesec);

    let importsec = importsec(module);
    file.write_all(&importsec);

    let funcsec = funcsec(module);
    file.write_all(&funcsec);

    let tablesec = tablesec(module);
    file.write_all(&tablesec);

    let memsec = memsec(module);
    file.write_all(&memsec);

    let globalsec = globalsec(module);
    file.write_all(&globalsec);

    let exportsec = exportsec(module);
    file.write_all(&exportsec);

    let startsec = startsec(module);
    file.write_all(&startsec);

    let elemsec = elemsec(module);
    file.write_all(&elemsec);
    
    let codesec = codesec(module);
    file.write_all(&codesec);

    let datasec = datasec(module);
    file.write_all(&datasec);

    Ok(())
}

fn magic() -> Vec<u8> { b"\0asm".to_vec() }
fn version() -> Vec<u8> { [0x01, 0x00, 0x00, 0x00].to_vec() }

fn typesec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];

    for tp in &module.types {
        // function type
        contents.push(0x60);

        // valtype param
        let mut params = vec![];
        for vt in &tp.0 {
            params.push(valtype_to_byte(vt));
        }
        contents.extend(vec_(params));

        let mut results = vec![];
        // valtype result
        for vt in &tp.1 {
            results.push(valtype_to_byte(vt));
        }
        contents.extend(vec_(results));
    }

    sect(1, &contents)
}

fn importsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];

    let mut results = vec![];
    for imp in &module.imports {
        contents.extend(imp.0.clone().into_bytes());
        contents.extend(imp.1.clone().into_bytes());
        contents.extend(importdesc_to_bytes());
    }
    contents.push(0x60);  // function type

    sect(2, &contents)
}

fn funcsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    for func in &module.funcs {
        contents.extend(&func.0.to_le_bytes());
    }

    sect(3, &contents)
}

fn tablesec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    for table in &module.tables {
        contents.extend(&tabletype_to_bytes(&table.0));
    }

    sect(4, &contents)
}

fn memsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    for mem in &module.mems {
        contents.extend(&memtype_to_bytes(&mem.0));
    }

    sect(5, &contents)
}

fn globalsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    for global in &module.globals {
        contents.extend(&globaltype_to_bytes(&global.0));
        contents.extend(expr_to_bytes(global.1));
    }

    sect(6, &contents)
}

fn exportsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    for exp in &module.exports {
        contents.extend(&exp.0.clone().into_bytes());
        contents.extend(exportdesc_to_bytes(exp.1));
    }

    sect(7, &contents)
}

fn startsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];
    if let Some(startidx) = module.start {
        contents.extend(&startidx.to_le_bytes())
    }
    sect(8, &contents)
}

fn elemsec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];

    let mut elems = vec![];
    for elem in &module.elems {
        elsms.extend(&elem.table.to_le_bytes());
        elsms.extend(expr_to_bytes(&elem.offset));
        let mut inits = vec![];
        for ini in elem.init {
            inits.extend(&ini.to_le_bytes());
        }
        elems.extend(vec_(inits));
    }
    contents.extend(vec_(elems));

    sect(9, &contents)
}

fn codesec(module: &Module) -> Vec<u8> {
        // contents
    let mut contents = vec![];
    let mut code = vec![];
    for func in &module.funcs {
        code.extend(vec_(funccode(func)));
    }
    contents.extend(vec_(code));

    sect(10, &contents)
}

fn datasec(module: &Module) -> Vec<u8> {
    // contents
    let mut contents = vec![];    
    let mut data = vec![];
    for dat in &module.data {
        data.extend(&dat.data.to_le_bytes());
        data.extend(expr_to_bytes(&elem.offset));
        data.extend(vec_(dat.init.clone().into_bytes()));
    }
    contents.extend(vec_(data));
    sect(11, &contents)
}

fn funccode(func: &Func) -> Vec<u8> {
    let mut ret = vec![];
    let mut locals = vec![];
    for local in func.1 {
        locals.extend(1u32.to_le_bytes());  // TODO: not compressed 
        locals.push(valtype_to_byte(local));
    }
    ret.extend(vec_(locals));

    ret.extend(expr_to_bytes(func.2))
    ret
}

fn exportdesc_to_bytes(desc: ExportDesc) -> Vec<u8> {
    match desc {
        ExportDesc::Func(typeidx) => {
            let mut ret = vec![0x00];
            ret.extend(&typeidx.to_le_bytes());
            ret
        },
        ExportDesc::Table(tableidx) => {
            let mut ret = vec![0x01];
            ret.extend(&tableidx.to_le_bytes());
            ret
        },
        ExportDesc::Mem(memidx) => {
            let mut ret = vec![0x02];
            ret.extend(&memidx.to_le_bytes());
            ret
        },
        ExportDesc::Global(globalidx) => {
            let mut ret = vec![0x03];
            ret.extend(&globalidx.to_le_bytes());
            ret
        },
    }
}

fn expr_to_bytes(expr: Expr) -> Vec<u8> {
    let mut ret = vec![];
    for instr in expr.0 {
        ret.extend(instr_to_bytes(instr));
    }
    ret.push(0x0B);
    ret
}

fn instr_to_bytes(instr: Instr) -> Vec<u8> {
    match instr {
        Instr::Unreachable => vec![0x00],
        Instr::Nop => vec![0x01],
        Instr::Block => vec![0x02],
        Instr::Loop => vec![0x03],
        Instr::If => vec![0x04],
        Instr::Else => vec![0x05],

        Instr::End => vec![0x0B],

        Instr::Br => vec![0x0C],
        Instr::BrIf => vec![0x0D],
        Instr::BrTable => vec![0x0E],
        Instr::Return => vec![0x0F],
        Instr::Call => vec![0x10],
        Instr::Indirect => vec![0x11],

        Instr::Drop => vec![0x1A],
        Instr::Select => vec![0x1B],

        Instr::LocalGet => vec![0x20],
        Instr::LocalSet => vec![0x21],
        Instr::LocalTee => vec![0x22],
        Instr::GlobalGet => vec![0x23],
        Instr::GlobalSet => vec![0x24],

        Instr::Load(ValType::I32) => vec![0x28],
        Instr::Load(ValType::I64) => vec![0x29],
        Instr::Load(ValType::F32) => vec![0x2A],
        Instr::Load(ValType::F64) => vec![0x2B],
        Instr::ILoad8(ValSize::V32, ValSign::S) => vec![0x2C],
        Instr::ILoad8(ValSize::V32, ValSign::U) => vec![0x2D],
        Instr::ILoad16(ValSize::V32, ValSign::S) => vec![0x2E],
        Instr::ILoad16(ValSize::V32, ValSign::U) => vec![0x2F],

        Instr::ILoad8(ValSize::V64, ValSign::S) => vec![0x30],
        Instr::ILoad8(ValSize::V64, ValSign::U) => vec![0x31],
        Instr::ILoad16(ValSize::V64, ValSign::S) => vec![0x32],
        Instr::ILoad16(ValSize::V64, ValSign::U) => vec![0x33],
        Instr::ILoad32(ValSize::V64, ValSign::S) => vec![0x34],
        Instr::ILoad32(ValSize::V64, ValSign::U) => vec![0x35],

        Instr::Store(ValType::I32) => vec![0x36],
        Instr::Store(ValType::I64) => vec![0x37],
        Instr::Store(ValType::F32) => vec![0x38],
        Instr::Store(ValType::F64) => vec![0x39],

        Instr::IStore8(ValSize::V32) => vec![0x3A],
        Instr::IStore16(ValSize::V32) => vec![0x3B],
        Instr::IStore8(ValSize::V64) => vec![0x3C],
        Instr::IStore16(ValSize::V64) => vec![0x3D],
        Instr::IStore32(ValSize::V64) => vec![0x3E],
        Instr::MemorySize => vec![0x3F, 0x00],
        Instr::MemoryGrow => vec![0x40, 0x00],

        Instr::I32Const => vec![0x41],
        Instr::I64Const => vec![0x42],
        Instr::F32Const => vec![0x43],
        Instr::F64Const => vec![0x44],

        Instr::ITestOp(vs, ITestOp::Eqz) => {
            match vs {
                ValSize::V32 => vec![0x45],
                ValSize::V64 => vec![0x50],
            }
        },
        Instr::ITRelOp(vs, irelop) => {
            match vs {
                ValSize::V32 => {
                    match irelop {
                        IRelOp::Eq => vec![0x46],
                        IRelOp::Ne => vec![0x47],
                        IRelOp::Lt(ValSign::S) => vec![0x48],
                        IRelOp::Lt(ValSign::U) => vec![0x49],
                        IRelOp::Gt(ValSign::S) => vec![0x4A],
                        IRelOp::Gt(ValSign::U) => vec![0x4B],
                        IRelOp::Le(ValSign::S) => vec![0x4C],
                        IRelOp::Le(ValSign::U) => vec![0x4D],
                        IRelOp::Ge(ValSign::S) => vec![0x4E],
                        IRelOp::Ge(ValSign::U) => vec![0x4F],
                    }
                },
                ValSize::V64 => {
                    match irelop {
                        IRelOp::Eq => vec![0x51],
                        IRelOp::Ne => vec![0x52],
                        IRelOp::Lt(ValSign::S) => vec![0x53],
                        IRelOp::Lt(ValSign::U) => vec![0x54],
                        IRelOp::Gt(ValSign::S) => vec![0x55],
                        IRelOp::Gt(ValSign::U) => vec![0x56],
                        IRelOp::Le(ValSign::S) => vec![0x57],
                        IRelOp::Le(ValSign::U) => vec![0x58],
                        IRelOp::Ge(ValSign::S) => vec![0x59],
                        IRelOp::Ge(ValSign::U) => vec![0x5A],
                    }
                },
            }
        },
        Instr::FRelOp(vs, frelop) => {
            ValSize::V32 => {
                match freop {
                    FRelOp::Eq => vec![0x5B],
                    FRelOp::Ne => vec![0x5C],
                    FRelOp::Lt => vec![0x5D],
                    FRelOp::Gt => vec![0x5E],
                    FRelOp::Le => vec![0x5F],
                    FRelOp::Ge => vec![0x60],
                }
            },
            ValSize::V32 => {
                match freop {
                    FRelOp::Eq => vec![0x61],
                    FRelOp::Ne => vec![0x62],
                    FRelOp::Lt => vec![0x63],
                    FRelOp::Gt => vec![0x64],
                    FRelOp::Le => vec![0x65],
                    FRelOp::Ge => vec![0x66],
                }
            },
        },
        Instr::IUnOp(vs, iunop) => {
            ValSize::V32 => {
                match ireop {
                    IUnOp::Clz => vec![0x67],
                    IUnOp::Ctz => vec![0x68],
                    IUnOp::Popcnt => vec![0x69],
                }
            },
            ValSize::V64 => {
                match ireop {
                    IUnOp::Clz => vec![0x79],
                    IUnOp::Ctz => vec![0x7A],
                    IUnOp::Popcnt => vec![0x7B],
                }
            },
        },
        Instr::IBinOp(vs, ibinop) => {
            ValSize::V32 => {
                match ibinop {
                    IBinOp::Add => vec![0x6A],
                    IBinOp::Sub => vec![0x6B],
                    IBinOp::Mul => vec![0x6C],
                    IBinOp::Div(ValSign::S) => vec![0x6D],
                    IBinOp::Div(ValSign::U) => vec![0x6E],
                    IBinOp::Rem(ValSign::S) => vec![0x6F],
                    IBinOp::Rem(ValSign::U) => vec![0x70],
                    IBinOp::And => vec![0x71],
                    IBinOp::Or => vec![0x72],
                    IBinOp::Xor => vec![0x73],
                    IBinOp::Shl => vec![0x74],
                    IBinOp::Shr(ValSign::S) => vec![0x75],
                    IBinOp::Shr(ValSign::U) => vec![0x76],
                    IBinOp::Rotl => vec![0x77],
                    IBinOp::Rotr => vec![0x78],
                }
            },
            ValSize::V64 => {
                match ibinop {
                    IBinOp::Add => vec![0x7C],
                    IBinOp::Sub => vec![0x7D],
                    IBinOp::Mul => vec![0x7E],
                    IBinOp::Div(ValSign::S) => vec![0x7F],
                    IBinOp::Div(ValSign::U) => vec![0x80],
                    IBinOp::Rem(ValSign::S) => vec![0x81],
                    IBinOp::Rem(ValSign::U) => vec![0x82],
                    IBinOp::And => vec![0x83],
                    IBinOp::Or => vec![0x84],
                    IBinOp::Xor => vec![0x85],
                    IBinOp::Shl => vec![0x86],
                    IBinOp::Shr(ValSign::S) => vec![0x87],
                    IBinOp::Shr(ValSign::U) => vec![0x88],
                    IBinOp::Rotl => vec![0x89],
                    IBinOp::Rotr => vec![0x8A],
                }
            },
        },
        Instr::FUnOp(vs, funop) => {
            ValSize::V32 => {
                match funop {
                    FUnOp::Abs => vec![0x8B],
                    FUnOp::Neg => vec![0x8C],
                    FUnOp::Ceil => vec![0x8D],
                    FUnOp::Floor => vec![0x8E],
                    FUnOp::Trunc => vec![0x8F],
                    FUnOp::Nearest => vec![0x90],
                    FUnOp::Sqrt => vec![0x91],
                }
            },
            ValSize::V64 => {
                match funop {
                    FUnOp::Abs => vec![0x99],
                    FUnOp::Neg => vec![0x9A],
                    FUnOp::Ceil => vec![0x9B],
                    FUnOp::Floor => vec![0x9C],
                    FUnOp::Trunc => vec![0x9D],
                    FUnOp::Nearest => vec![0x9E],
                    FUnOp::Sqrt => vec![0x9F],
                }
            },
        },
        Instr::FBinOp(vs, fbinop) => {
            ValSize::V32 => {
                match fbinop {
                    FBinOp::Add => vec![0x92],
                    FBinOp::Sub => vec![0x93],
                    FBinOp::Mul => vec![0x94],
                    FBinOp::Div => vec![0x95],
                    FBinOp::Min => vec![0x96],
                    FBinOp::Max => vec![0x97],
                    FBinOp::Copysign => vec![0x98],
                }
            },
            ValSize::V64 => {
                match fbinop {
                    FBinOp::Add => vec![0xA0],
                    FBinOp::Sub => vec![0xA1],
                    FBinOp::Mul => vec![0xA2],
                    FBinOp::Div => vec![0xA3],
                    FBinOp::Min => vec![0xA4],
                    FBinOp::Max => vec![0xA5],
                    FBinOp::Copysign => vec![0xA6],
                }
            },
        },
        Instr::CvtOp(cvtop) => {
            match {
                CvtOp::I32WrapFromI64 => vec![0xA7],
                CvtOp::ITruncFromF => {
                    match ITruncFromF {
                        ValSize::V32 => {
                            match ITruncFromF {
                                ValSize::V32 => {
                                    match vs {
                                        ValSign::S => vec![0xA8],
                                        ValSign::U => vec![0xA9],
                                    }
                                },
                                ValSize::V64 => {
                                    match vs {
                                        ValSign::S => vec![0xAA],
                                        ValSign::U => vec![0xAB],
                                    }
                                },
                            }
                        },
                        ValSize::V64 => {
                            match ITruncFromF {
                                ValSize::V32 => {
                                    match vs {
                                        ValSign::S => vec![0xAE],
                                        ValSign::U => vec![0xAF],
                                    }
                                },
                                ValSize::V64 => {
                                    match vs {
                                        ValSign::S => vec![0xB0],
                                        ValSign::U => vec![0xB1],
                                    }
                                },
                            }
                        },
                    }
                },

                CvtOp::I64ExtendFromI32(ValSign::S) => vec![0xAC],
                CvtOp::I64ExtendFromI32(ValSign::U) => vec![0xAD],

                CvtOp::FConvertFromI => {
                    match ITruncFromF {
                        ValSize::V32 => {
                            match ITruncFromF {
                                ValSize::V32 => {
                                    match vs {
                                        ValSign::S => vec![0xB2],
                                        ValSign::U => vec![0xB3],
                                    }
                                },
                                ValSize::V64 => {
                                    match vs {
                                        ValSign::S => vec![0xB4],
                                        ValSign::U => vec![0xB5],
                                    }
                                },
                            }
                        },
                        ValSize::V64 => {
                            match ITruncFromF {
                                ValSize::V32 => {
                                    match vs {
                                        ValSign::S => vec![0xB7],
                                        ValSign::U => vec![0xB8],
                                    }
                                },
                                ValSize::V64 => {
                                    match vs {
                                        ValSign::S => vec![0xB9],
                                        ValSign::U => vec![0xBA],
                                    }
                                },
                            }
                        },
                    }
                },

                CvtOp::F32DemoteFromF64 => vec![0xB6],

                CvtOp::F64PromoteFromF32 => vec![0xBB],
                
                CvtOp::IReinterpretFromF(ValSize::V32) => vec![0xBC],
                CvtOp::IReinterpretFromF(ValSize::V64) => vec![0xBD],
                CvtOp::FReinterpretFromI(ValSize::V32) => vec![0xBE],
                CvtOp::FReinterpretFromI(ValSize::V64) => vec![0xBF],
            }
        }
    }
}

fn importdesc_to_bytes(desc: ImportDesc) -> Vec<u8> {
    match desc {
        ImportDesc::Func(typeidx) => {
            let mut ret = vec![0x00];
            ret.extend(&typeidx.to_le_bytes());
            ret
        },
        ImportDesc::Table(tabletype) => {
            let mut ret = vec![0x01];
            ret.extend(&tabletype_to_bytes(&tabletype));
            ret
        },
        ImportDesc::Mem(memtype) => {
            let mut ret = vec![0x02];
            ret.extend(&memtype_to_bytes(&memtype));
            ret
        },
        ImportDesc::Global(globaltype) => {
            let mut ret = vec![0x03];
            ret.extend(&globaltype_to_bytes(globaltype));
            ret
        },
    }
}

fn globaltype_to_bytes(gt: GlobalType) -> Vec<u8> {
    let mut ret = vec![];
    ret.push(valtype_to_byte(&gt.1));
    ret.push(mutablity_to_byte(gt.0));
    ret
}

fn mutablity_to_byte(mutablity: Mutablity) -> u8 {
    match mutablity {
        Mutablity::Const => 0x00,
        Mutablity::Var => 0x01,
    }
}

fn memtype_to_bytes(mt: &MemType) -> Vec<u8> {
    limits_to_bytes(&mt.0)
}

fn tabletype_to_bytes(tt: &TableType) -> Vec<u8> {
    let mut ret = vec![0x70];  // funcref
    ret.extend(limits_to_bytes(&tt.limits));
    ret
}

fn limits_to_bytes(lim: &Limits) -> Vec<u8> {
    let mut res = vec![];
    if let Some(max) = lim.max {
        res.push(0x01);
        res.extend(&lim.min.to_le_bytes());
        res.extend(&max.to_le_bytes());
    } else {
        res.push(0x00);
        res.extend(&lim.min.to_le_bytes());
    }
    res
}

fn valtype_to_byte(vt: &ValType) -> u8 {
    match vt {
        ValType::I32 => 0x7F,
        ValType::I64 => 0x7E,
        ValType::F32 => 0x7D,
        ValType::F64 => 0x7C,
    }
}

fn sect(id: u8, cont: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    res.push(id);
    res.extend(&cont.len().to_le_bytes());
    res.extend(cont);
    res
}

fn vec_(seq: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    if seq.len() > 0 {
        res.extend(&seq.len().to_le_bytes());
        res.extend(seq);
    }
    res
}

#[test]
fn test() {
    let module = Module::default();
    compile(&module);
}