use std::rc::Rc;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use crate::ast::StringNode::StringNode;
use crate::ast::IfNode::IfNode;
use crate::ast::TrimNode::TrimNode;
use crate::ast::ForEachNode::ForEachNode;
use crate::ast::ChooseNode::ChooseNode;
use crate::ast::OtherwiseNode::OtherwiseNode;
use crate::ast::WhenNode::WhenNode;
use crate::ast::BindNode::BindNode;
use crate::ast::IncludeNode::IncludeNode;
use crate::ast::SetNode::SetNode;

use crate::ast::SelectNode::SelectNode;
use crate::ast::DeleteNode::DeleteNode;
use crate::ast::UpdateNode::UpdateNode;
use crate::ast::InsertNode::InsertNode;
use crate::ast::InsertTempleteNode::InsertTempleteNode;
use crate::ast::UpdateTempleteNode::UpdateTempleteNode;
use crate::ast::DeleteTempleteNode::DeleteTempleteNode;
use crate::ast::SelectTempleteNode::SelectTempleteNode;

#[derive(Clone)]
pub enum NodeType {
    Null,
    NString(StringNode),
    NIf(IfNode),
    NTrim(TrimNode),
    NForEach(ForEachNode),
    NChoose(ChooseNode),
    NOtherwise(OtherwiseNode),
    NWhen(WhenNode),
    NBind(BindNode),
    NInclude(IncludeNode),
    NSet(SetNode),

    //CRUD
    NInsertNode(InsertNode),
    NUpdateNode(UpdateNode),
    NDeleteNode(DeleteNode),
    NSelectNode(SelectNode),

    NInsertTempleteNode(InsertTempleteNode),
    NUpdateTempleteNode(UpdateTempleteNode),
    NDeleteTempleteNode(DeleteTempleteNode),
    NSelectTempleteNode(SelectTempleteNode),
}

impl <'a>SqlNode for NodeType {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        match self {
            NodeType::NSelectNode(node) => return node.eval(env),
            NodeType::NDeleteNode(node) => return node.eval(env),
            NodeType::NUpdateNode(node) => return node.eval(env),
            NodeType::NInsertNode(node) => return node.eval(env),

            NodeType::NSelectTempleteNode(node) => return node.eval(env),
            NodeType::NDeleteTempleteNode(node) => return node.eval(env),
            NodeType::NUpdateTempleteNode(node) => return node.eval(env),
            NodeType::NInsertTempleteNode(node) => return node.eval(env),

            NodeType::Null => return Result::Ok(String::new()),
            NodeType::NString(stringNode) => return stringNode.eval(env),
            NodeType::NIf(ifNode) => return ifNode.eval(env),
            NodeType::NTrim(trimNode) => return trimNode.eval(env),
            NodeType::NForEach(forEachNode) => return forEachNode.eval(env),
            NodeType::NChoose(chooseNode) => return chooseNode.eval(env),
            NodeType::NOtherwise(otherwiseNode) => return otherwiseNode.eval(env),
            NodeType::NWhen(whenNode) => return whenNode.eval(env),
            NodeType::NBind(bindNode) => return bindNode.eval(env),
            NodeType::NInclude(includeNode) => return includeNode.eval(env),
            NodeType::NSet(setNode) => return setNode.eval(env),
            _ => Result::Err(String::from("NodeType not exist!")),
        }
    }

    fn print(&self) -> String {
        match self {
            NodeType::NSelectNode(node) => return node.print(),
            NodeType::NUpdateNode(node) => return node.print(),
            NodeType::NInsertNode(node) => return node.print(),
            NodeType::NDeleteNode(node) => return node.print(),

            NodeType::NSelectTempleteNode(node) => return node.print(),
            NodeType::NUpdateTempleteNode(node) => return node.print(),
            NodeType::NInsertTempleteNode(node) => return node.print(),
            NodeType::NDeleteTempleteNode(node) => return node.print(),


            NodeType::Null => return "null".to_string(),
            NodeType::NString(stringNode) => return stringNode.print(),
            NodeType::NIf(ifNode) => return ifNode.print(),
            NodeType::NTrim(trimNode) => return trimNode.print(),
            NodeType::NForEach(forEachNode) => return forEachNode.print(),
            NodeType::NChoose(chooseNode) => return chooseNode.print(),
            NodeType::NOtherwise(otherwiseNode) => return otherwiseNode.print(),
            NodeType::NWhen(whenNode) => return whenNode.print(),
            NodeType::NBind(bindNode) => return bindNode.print(),
            NodeType::NInclude(includeNode) => return includeNode.print(),
            NodeType::NSet(setNode) => return setNode.print(),
            _ => String::from("NodeType not exist!"),
        }
    }
}



