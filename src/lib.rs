use scrypto::prelude::*;

blueprint! {
    struct HelloOrchestrator {
        helloes: HashMap<String, ComponentAddress>
    }

    impl HelloOrchestrator {
        pub fn new() -> ComponentAddress {

            Self {
                helloes: HashMap::new()
            }
            .instantiate()
            .globalize()
        }

        pub fn register(&mut self, component_to_register: String, name: String) {

            let component_address: ComponentAddress = component_to_register.parse().unwrap();

            self.helloes.insert(name, component_address);
        }

        pub fn hello_for(&mut self, name: String) {
            let component_address: &ComponentAddress = self.helloes.get(&name).unwrap();
            let component: &Component = borrow_component!(*component_address);
            let return_value = component.call::<String>("say_hello", args![]);

            debug!("Hello in {}: {}", name, return_value)
        }
    }
}

blueprint! {

    struct HelloFrench {

    }

    impl HelloFrench {
        pub fn say_hello(&self) -> String {
            String::from("bonjour")
        }

        pub fn new() -> ComponentAddress {
            Self {}
            .instantiate()
            .globalize()
        }
    }

}

blueprint! {

    struct HelloEnglish {

    }

    impl HelloEnglish {
        pub fn say_hello(&self) -> String {
            String::from("hello")
        }

        pub fn new() -> ComponentAddress {
            Self {}
            .instantiate()
            .globalize()
        }
    }

}
