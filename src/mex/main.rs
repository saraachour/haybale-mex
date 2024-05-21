use haybale::*;
use haybale::function_hooks::IsCall;
use haybale::backend::{DefaultBackend, Backend};
use log::{debug, error, log_enabled, info, Level};

// todo, if arguments are concrete, Just concretely execute. If arguments are symbolic,
// symbolically execute (in printf's case, treat returned value as purely symbolic int.)
fn hook_printf<'p, B: Backend>(
    state: &mut State<'p, B>,
    call: &'p dyn IsCall,
) -> Result<ReturnValue<B::BV>> {
    debug!("name: {:?}\n", call.get_called_func());
    debug!("args: {:?}\n", call.get_arguments());
    debug!("ret-attrs: {:?}\n", call.get_return_attrs());
    debug!("fn-attrs: {:?}\n", call.get_fn_attrs());
    // returns number of characters debuged
    return Ok(ReturnValue::ReturnVoid);
}

fn main(){
    let _ = env_logger::builder().is_test(true).try_init();
    let project = Project::from_bc_path("cacti.bc").expect("bytecode not found");
    let mut config = Config::<DefaultBackend>::default();
    //config.function_hooks
    //    .add("printf", &hook_printf);
    let mut em = symex_function("main", &project, config, None).expect("constructed env");
    let result = em.next().expect("expected at least one path.");
    print!("path: {:?}\n", result);

}
