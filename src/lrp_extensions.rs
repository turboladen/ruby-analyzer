use lib_ruby_parser::{nodes, Node};

pub(super) trait NameFromNode {
    fn name_from_node(&self) -> String;
}

impl NameFromNode for nodes::Class {
    fn name_from_node(&self) -> String {
        match &*self.name {
            Node::Const(nodes::Const { name, .. }) => name.clone(),
            _ => "{{expression}}".to_string(),
        }
    }
}

impl NameFromNode for nodes::Module {
    fn name_from_node(&self) -> String {
        match &*self.name {
            Node::Const(nodes::Const { name, .. }) => name.clone(),
            _ => "{{expression}}".to_string(),
        }
    }
}

pub(super) trait OptionNameFromNode {
    fn option_name_from_node(&self) -> Option<String>;
}

impl OptionNameFromNode for nodes::MatchRest {
    fn option_name_from_node(&self) -> Option<String> {
        match self.name.as_deref()? {
            Node::MatchVar(match_var) => Some(match_var.name.clone()),
            _ => None,
        }
    }
}
