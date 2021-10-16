enum NodeType {
    Text,
    List,
    HashMap,
}

pub trait Node {
    fn get_type(&self) -> NodeType;
}

#[derive(Default)]
pub struct Text<'a> {
    pub value: &'a str,
}

impl<'a> Text<'a> {
    fn get_text(&self) -> &'a str {
        self.value
    }
}

impl<'a> Node for Text<'a> {
    fn get_type(&self) -> NodeType {
        NodeType::Text
    }
}

#[derive(Default)]
pub struct List {
    pub values: Vec<Box<dyn Node>>,
}

impl List {
    fn add(&mut self, value: Box<dyn Node>) {
        self.values.push(value)
    }
}

impl Node for List {
    fn get_type(&self) -> NodeType {
        NodeType::List
    }
}

pub struct Map {
    pub key: String,
    pub value: Box<dyn Node>,
}

impl Map {
    fn add(&mut self, key: String, value: Box<dyn Node>) {
        self.key = key;
        self.value = value;
    }
}

impl Node for Map {
    fn get_type(&self) -> NodeType {
        NodeType::HashMap
    }
}
