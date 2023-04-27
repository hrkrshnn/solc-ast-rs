#[cfg(test)]
mod tests {
    use eyre::Result;
    use solc_ast::{ast::*, visitor::ast_visitor::*};

    fn read_counter() -> Result<SourceUnit> {
        Ok(serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("tests/ast-json/Counter.json")?,
        ))?)
    }

    #[test]
    fn deserialize_counter() -> Result<()> {
        let source_unit = read_counter()?;
        assert_eq!(source_unit.absolute_path, Some("Counter.sol".into()));
        Ok(())
    }

    #[derive(Default, Debug)]
    struct FunctionDefinitionCollector {
        names: Vec<String>,
    }

    impl ASTConstVisitor for FunctionDefinitionCollector {
        fn end_visit_function_definition(&mut self, node: &FunctionDefinition) -> Result<()> {
            self.names.push(node.name.clone());
            Ok(())
        }
    }

    #[test]
    fn functions() -> Result<()> {
        let source_unit = read_counter()?;
        let mut function_definition_collector = FunctionDefinitionCollector::default();
        source_unit.accept(&mut function_definition_collector)?;
        assert_eq!(
            function_definition_collector.names,
            vec![String::from("setNumber"), String::from("increment")]
        );
        Ok(())
    }

    #[derive(Default, Debug)]
    struct VariableNameCollector {
        names: Vec<String>,
    }

    impl ASTConstVisitor for VariableNameCollector {
        fn end_visit_variable_declaration(&mut self, node: &VariableDeclaration) -> Result<()> {
            self.names.push(node.name.clone());
            Ok(())
        }
    }

    #[test]
    fn variables() -> Result<()> {
        let source_unit = read_counter()?;
        let mut variable_name_collector = VariableNameCollector::default();
        source_unit.accept(&mut variable_name_collector)?;
        assert_eq!(
            variable_name_collector.names,
            vec![String::from("number"), String::from("newNumber")]
        );
        Ok(())
    }
}
