# solc-ast

solc-ast provides rust bindings for the [solidity AST](https://solidity-ast.netlify.app/) and visitors. The visitors were built to be 1-1 compatible with the [visitors](https://github.com/ethereum/solidity/blob/develop/libsolidity/ast/ASTVisitor.h) from solc.

Note: The AST structs are almost entirely from [camden-smallwood](https://github.com/camden-smallwood)'s [solidity-rs](https://github.com/camden-smallwood/solidity-rs). The main deviation is in the Visitor implementation.

Note: AST for inline assembly in incomplete.
