use eyre::Result;

use crate::ast::*;

pub trait ASTConstVisitor {
    fn visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_array_type_name(&mut self, node: &ArrayTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_assignment(&mut self, node: &Assignment) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_binary_operation(&mut self, node: &BinaryOperation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_conditional(&mut self, node: &Conditional) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_conditional(&mut self, node: &Conditional) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_contract_definition(&mut self, node: &ContractDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_elementary_type_name(&mut self, node: &ElementaryTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_elementary_type_name_expression(
        &mut self,
        node: &ElementaryTypeNameExpression,
    ) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_enum_definition(&mut self, node: &EnumDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_event_definition(&mut self, node: &EventDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_event_definition(&mut self, node: &EventDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_error_definition(&mut self, node: &ErrorDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_call_options(&mut self, node: &FunctionCallOptions) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_identifier(&mut self, node: &Identifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_index_access(&mut self, node: &IndexAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_index_access(&mut self, node: &IndexAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_inheritance_specifier(&mut self, node: &InheritanceSpecifier) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_index_range_access(&mut self, node: &IndexRangeAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_literal(&mut self, node: &Literal) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_literal(&mut self, node: &Literal) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_member_access(&mut self, node: &MemberAccess) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_member_access(&mut self, node: &MemberAccess) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_new_expression(&mut self, node: &NewExpression) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_new_expression(&mut self, node: &NewExpression) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_mapping(&mut self, node: &Mapping) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_mapping(&mut self, node: &Mapping) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_modifier_definition(&mut self, node: &ModifierDefinition) -> Result<()> {
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

    fn visit_struct_definition(&mut self, node: &StructDefinition) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_struct_definition(&mut self, node: &StructDefinition) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_structured_documentation(&mut self, node: &StructuredDocumentation) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_tuple_expression(&mut self, node: &TupleExpression) -> Result<()> {
        self.end_visit_node(node)
    }

    fn visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_unary_operation(&mut self, node: &UnaryOperation) -> Result<()> {
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

    fn visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<bool> {
        self.visit_node(node)
    }
    fn end_visit_using_for_directive(&mut self, node: &UsingForDirective) -> Result<()> {
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
