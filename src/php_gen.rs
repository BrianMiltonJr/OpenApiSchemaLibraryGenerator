pub enum PHPAccessModifier {
    PUBLIC,
    PRIVATE,
    PROTECTED,
}

pub struct PHPFunction {
    is_static: bool,
    name: String,
    arguments: Vec<String>,
    access_modifier: PHPAccessModifier,
    instructions: Vec<String>
}

impl PHPFunction {
    pub fn new(
        is_static: bool,
        name: String,
        arguments: Vec<String>,
        access_modifier: PHPAccessModifier,
        instructions: Vec<String>
    ) -> PHPFunction {
        PHPFunction {
            is_static: is_static,
            name: name,
            arguments: arguments,
            access_modifier: access_modifier,
            instructions: instructions,
        }
    }

    pub fn to_string(&mut self) -> String {
        let modifier = match self.access_modifier {
            PHPAccessModifier::PUBLIC => "public",
            PHPAccessModifier::PRIVATE => "private",
            PHPAccessModifier::PROTECTED => "protected",
        };

        let is_static = match self.is_static {
            true => " static ",
            false => " ",
        };

        let mut function_signature = format!("{}{}function {}(", modifier, is_static, self.name);

        let arguments_length = self.arguments.len();
        for (i, ele) in self.arguments.iter_mut().enumerate() {
            function_signature.push_str("$");
            function_signature.push_str(ele.as_str());
            if i != arguments_length - 1 {
                function_signature.push_str(", ");
            }
        }

        function_signature.push_str(") {\r\n");

        for ele in self.instructions.iter_mut() {
            function_signature.push_str("        ");
            function_signature.push_str(ele.as_str());
            function_signature.push_str(";\r\n");
        }

        function_signature.push_str("    }\r\n");

        return function_signature;
    }
}

pub struct PHPClassProperty {
    access_modifier: PHPAccessModifier,
    name: String,
    typing: Option<String>,
}

impl PHPClassProperty {
    pub fn new(access: PHPAccessModifier, name: String, typing: Option<String>) -> Self {
        Self {
            access_modifier: access,
            name: name,
            typing: typing,
        }
    }

    pub fn to_string(&mut self) -> String {
        let modifier = match self.access_modifier {
            PHPAccessModifier::PUBLIC => "public",
            PHPAccessModifier::PRIVATE => "private",
            PHPAccessModifier::PROTECTED => "protected",
        };

        let typing = match &self.typing {
            Some(string) => " ".to_owned() + string + &" ".to_owned(),
            None => " ".to_owned(),
        };
        let property_signature = format!("{}{}${};", modifier, typing, self.name);

        return property_signature;
    }
}

pub struct PHPClass {
    namespace: String,
    class_name: String,
    imports: Vec<String>,
    properties: Vec<PHPClassProperty>,
    methods: Vec<PHPFunction>,
}

impl PHPClass {
    pub fn new(namespace: String, class_name: String) -> Self {
        Self {
            namespace: namespace,
            class_name: class_name,
            imports: vec![],
            properties: vec![],
            methods: vec![],
        }
    }

    pub fn get_name(self) -> String {
        return self.class_name;
    }

    pub fn add_import(&mut self, import: String) {
        self.imports.push(import);
    }

    pub fn add_property(&mut self, property: PHPClassProperty) {
        self.properties.push(property);
    }

    pub fn add_method(&mut self, method: PHPFunction) {
        self.methods.push(method);
    }

    pub fn to_string(&mut self) -> String {
        let mut string = String::from("<?php\r\n\r\n");

        string.push_str("namespace ");
        string.push_str(&self.namespace);
        string.push_str(";\r\n\r\n");

        for import in &self.imports {
            string.push_str("use ");
            string.push_str(import.as_str());
            string.push_str(";\r\n");
        }

        string.push_str("\r\nclass ");
        string.push_str(&self.class_name);
        string.push_str(" {\r\n");

        for property in self.properties.iter_mut() {
            string.push_str("    ");
            string.push_str(property.to_string().as_str());
            string.push_str("\r\n");
        }

        string.push_str("\r\n");
        
        for method in self.methods.iter_mut() {
            string.push_str("    ");
            string.push_str(method.to_string().as_str());
            string.push_str("\r\n");
        }

        string.push_str("}");

        return string;
    }
}