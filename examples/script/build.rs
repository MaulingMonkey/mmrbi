use mmrbi::ResultExt;
use mmrbi::cargo::script;

fn main() {
    let env = script::Env::get().or_die();
    script::out::warning(format!("crate run successfully: {:#?}", env));
}
