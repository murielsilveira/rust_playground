pub fn run() {
    let provider = get_provider("internal").unwrap();
    import_stuff(&provider);

    let external: Box<dyn ProviderRealtime> = Box::new(ExternalProvider);
    import_stuff_realtime(&external);
    import_stuff_realtime(&(&external as &Box<dyn ProviderRealtime>));
    import_stuff(&(external as Box<dyn Provider>));

    let internal: Box<dyn ProviderHistorical> = Box::new(InternalProvider);
    import_stuff_historical(&internal);
    import_stuff(&(internal as Box<dyn Provider>));

    asdf();
}

trait Provider {
    fn do_something(&self) -> String;
}

trait ProviderRealtime: Provider {
    fn do_something_realtime(&self);
}

trait ProviderHistorical: Provider {
    fn do_something_historical(&self);
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

impl ProviderRealtime for ExternalProvider {
    fn do_something_realtime(&self) {
        println!("External Realtime");
    }
}

impl ProviderHistorical for InternalProvider {
    fn do_something_historical(&self) {
        println!("Internal Historical");
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

fn import_stuff_realtime(provider: &Box<dyn ProviderRealtime>) {
    provider.do_something_realtime()
}

fn import_stuff_historical(provider: &Box<dyn ProviderHistorical>) {
    provider.do_something_historical()
}

trait BaseTrait {
    fn base_function(&self);
}

trait SubTrait: BaseTrait {
    fn sub_function(&self);
}

struct MyStruct;

impl BaseTrait for MyStruct {
    fn base_function(&self) {
        println!("Base function of MyStruct");
    }
}

impl SubTrait for MyStruct {
    fn sub_function(&self) {
        println!("Sub function of MyStruct");
    }
}

fn handle_base(obj: Box<dyn BaseTrait>) {
    obj.base_function();
}

fn handle_sub(obj: Box<dyn SubTrait>) {
    obj.sub_function();
}

fn asdf() {
    let base: Box<dyn BaseTrait> = Box::new(MyStruct);
    let sub1: Box<dyn SubTrait> = Box::new(MyStruct);
    let sub2: Box<dyn SubTrait> = Box::new(MyStruct);

    handle_base(base);
    // handle_sub(base); // err
    handle_base(sub1);
    handle_sub(sub2);
}
