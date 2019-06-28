use inkwell::values::*;
use z3::ast::{Ast, BV};

mod iterators;
pub use iterators::*;

mod state;
use state::State;

mod symex;
use symex::{symex_function, symex_again};

mod utils;
use utils::get_value_name;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum IntOfSomeWidth {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl IntOfSomeWidth {
    pub fn unwrap_to_i8(self) -> i8 {
        match self {
            IntOfSomeWidth::I8(i) => i,
            _ => panic!("unwrap_to_i8 on {:?}", self),
        }
    }

    pub fn unwrap_to_i16(self) -> i16 {
        match self {
            IntOfSomeWidth::I16(i) => i,
            _ => panic!("unwrap_to_i16 on {:?}", self),
        }
    }

    pub fn unwrap_to_i32(self) -> i32 {
        match self {
            IntOfSomeWidth::I32(i) => i,
            _ => panic!("unwrap_to_i32 on {:?}", self),
        }
    }

    pub fn unwrap_to_i64(self) -> i64 {
        match self {
            IntOfSomeWidth::I64(i) => i,
            _ => panic!("unwrap_to_i64 on {:?}", self),
        }
    }
}

// Given a function, find values of its inputs such that it returns zero
// Assumes function takes (some number of) integer arguments and returns an integer
// Returns None if there are no values of the inputs such that the function returns zero
pub fn find_zero_of_func(func: FunctionValue) -> Option<Vec<IntOfSomeWidth>> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let mut state = State::new(&ctx);

    let params: Vec<BasicValueEnum> = ParamsIterator::new(func).collect();
    for &param in params.iter() {
        assert!(param.is_int_value());
        let width = param.as_int_value().get_type().get_bit_width();
        let z3param = ctx.named_bitvector_const(&get_value_name(param), width);
        state.add_bv_var(param, z3param);
    }

    let returnwidth = func.get_type()
        .get_return_type()
        .expect("Expected function to have return type")
        .into_int_type()
        .get_bit_width();
    let zero = BV::from_u64(&ctx, 0, returnwidth);

    let mut optionz3rval = Some(symex_function(&mut state, func));
    loop {
        let z3rval = optionz3rval.clone().expect("optionz3rval should always be Some at this point in the loop");
        state.assert(&z3rval._eq(&zero));
        if state.check() { break; }
        optionz3rval = symex_again(&mut state);
        if optionz3rval.is_none() { break; }
    }

    if optionz3rval.is_some() {
        // in this case state.check() must have passed
        let model = state.get_model();
        let z3params = params.iter().map(|&p| state.lookup_bv_var(p));
        Some(z3params.map(|p| {
            let param_as_i64 = model.eval(p).unwrap().as_i64().unwrap();
            match p.get_size() {
                8 => IntOfSomeWidth::I8(param_as_i64 as i8),
                16 => IntOfSomeWidth::I16(param_as_i64 as i16),
                32 => IntOfSomeWidth::I32(param_as_i64 as i32),
                64 => IntOfSomeWidth::I64(param_as_i64 as i64),
                s => unimplemented!("Parameter with bitwidth {}", s),
            }
        }).collect())
    } else {
        None
    }
}
