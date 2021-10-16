pub enum NodeType {
    Text,
    List,
    HashMap,
}

pub trait Node {
    fn get_type(&self) -> NodeType;
}

pub trait TextElement<'a>: Node {
    fn get_text(&self) -> &'a str;
}

pub trait ListElement: Node {
    fn get_lists(&self) -> &Vec<Box<dyn Node>>;
}

pub trait MapElement<'a>: Node {
    fn get_key(&self) -> &'a str;
    fn get_value(&self) -> &Box<dyn Node>;
}

pub struct Text<'a> {
    pub value: &'a str,
}

impl<'a> Text<'a> {
    fn new(value: &'a str) -> Self {
        Self { value: value }
    }
}

impl<'a> TextElement<'a> for Text<'a> {
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

impl ListElement for List {
    fn get_lists(&self) -> &Vec<Box<dyn Node>> {
        &self.values
    }
}

impl Node for List {
    fn get_type(&self) -> NodeType {
        NodeType::List
    }
}

pub struct Map<'a> {
    pub key: &'a str,
    pub value: Box<dyn Node>,
}

impl<'a> Map<'a> {
    fn new(key: &'a str, value: Box<dyn Node>) -> Self {
        Self {
            key: key,
            value: value,
        }
    }
}

impl<'a> MapElement<'a> for Map<'a> {
    fn get_key(&self) -> &'a str {
        self.key
    }

    fn get_value(&self) -> &Box<dyn Node> {
        &self.value
    }
}

impl<'a> Node for Map<'a> {
    fn get_type(&self) -> NodeType {
        NodeType::HashMap
    }
}

#[cfg(test)]
mod value_tests {
    use crate::value::{List, ListElement, Map, Node, NodeType, Text};

    #[test]
    fn works() {
        //
        // {
        //     "nya": [
        //         "hoge",
        //         "fuga"
        //     ]
        // }
        //
        let text = Text::new("hoge");
        let text2 = Text::new("fuga");

        let mut list: List = Default::default();

        list.add(Box::new(text));
        list.add(Box::new(text2));

        let map = Map::new("nya", Box::new(list));

        assert_eq!(map.key, "nya");
        assert_eq!(map.get_type(), NodeType::HashMap);

        let buf = map.value;

        assert_eq!(buf.get_type(), NodeType::List);

        let element = (buf as Box<dyn ListElement>);
    }
}
