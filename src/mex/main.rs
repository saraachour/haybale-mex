
use haybale::{backend::DefaultBackend, symex_function, Config, Project};

fn main(){
    let project = Project::from_bc_path("cacti.bc").expect("bytecode not found");
    let em = symex_function("main", &project, Config::<DefaultBackend>::default(), None);
}
