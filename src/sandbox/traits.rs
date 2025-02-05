pub fn run() {
    let provider = get_provider("internal").unwrap();
    import_stuff(&provider);
}

trait Provider {
    fn do_something(&self) -> String;
}

struct ExternalProvider;

struct InternalProvider;

impl Provider for ExternalProvider {
    fn do_something(&self) -> String {
        println!("External");
        String::from("External")
    }
}

impl Provider for InternalProvider {
    fn do_something(&self) -> String {
        println!("Internal");
        String::from("Internal")
    }
}

fn get_provider(provider_key: &str) -> Result<Box<dyn Provider>, String> {
    match provider_key {
        "external" => Ok(Box::new(ExternalProvider)),
        "internal" => Ok(Box::new(InternalProvider)),
        _ => Err(format!("Invalid provider [{provider_key}].")),
    }
}

fn import_stuff(provider: &Box<dyn Provider>) -> String {
    provider.do_something()
}
