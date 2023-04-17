use eyre::Result;

use crate::ast::*;

pub trait ASTConstVisitor {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_literal(&mut self, node: &Literal) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_mapping(&mut self, node: &Mapping) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_override_specifier(&mut self, node: &OverrideSpecifier) -> Result<()> {
        self.end_visit_node(node)
    }


    fn visit_parameter_list(&mut self, node: &ParameterList) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_parameter_list(&mut self, node: &ParameterList) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_user_defined_type_name(&mut self, node: &UserDefinedTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_user_defined_value_type_definition(
        &mut self,
        node: &UserDefinedValueTypeDefinition,
    ) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_node(&mut self, _node: &impl Node) -> Result<bool> {
        Ok(true)
    }
    fn end_visit_node(&mut self, _node: &impl Node) -> Result<()> {
        Ok(())
    }
}

pub trait Node {
    fn accept(&self, _visitor: &mut impl ASTConstVisitor) -> Result<()> {
        Ok(())
    }
    // fn listAccept(
}

pub fn list_accept(list: &Vec<impl Node>, visitor: &mut impl ASTConstVisitor) -> Result<()> {
    for elem in list {
        elem.accept(visitor)?;
    }
    Ok(())
}
