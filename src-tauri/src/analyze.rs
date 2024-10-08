use crate::globals::{NodeType, TokenType, TreeNode};
use crate::symTab::{insert, lookup, print};

// Estructura para manejar el tipo de error en los nodos
pub fn type_error(t: &TreeNode, message: &str) {
    println!(
        "Type error en la línea {}: {}",
        t.value.clone().unwrap_or_default(),
        message
    );
}

// Función para recorrer el árbol de sintaxis abstracta en preorden y postorden
fn traverse(
    t: Option<&TreeNode>,
    pre_proc: Option<&dyn Fn(&TreeNode)>,
    post_proc: Option<&dyn Fn(&TreeNode)>,
) {
    if let Some(node) = t {
        if let Some(pre) = pre_proc {
            pre(node);
        }

        for child in &node.children {
            traverse(Some(child), pre_proc, post_proc);
        }

        if let Some(post) = post_proc {
            post(node);
        }
    }
}

// Función para insertar nodos en la tabla de símbolos
fn insert_node(t: &TreeNode, symbol_table: &mut SymbolTable) {
    match t.node_type {
        // Maneja declaraciones de variables (integer y double)
        NodeType::IntStatement | NodeType::DoubleStatement => {
            if let Some(ref name) = t.value {
                let lineno = t.token.unwrap() as usize;


                // Inserta en la tabla de símbolos si no está
                let loc = symbol_table.lookup(name).unwrap_or_else(|| {
                    let new_loc = symbol_table.next_location();
                    symbol_table.insert(name, t.token, "0", lineno, new_loc);
                    new_loc
                });

                // Actualiza el símbolo con la línea de uso
                symbol_table.insert(name, t.token, "0", lineno, loc);
            }
        }

        // para asignaciones
        NodeType::Assignment=> {
            if let Some(ref name) = t.value {
                let lineno = t.token.unwrap() as usize;

                // Inserta en la tabla de símbolos si no está
                let loc = symbol_table.lookup(name).unwrap_or_else(|| {
                    let new_loc = symbol_table.next_location();
                    symbol_table.insert(name, t.token, t.value, lineno, new_loc);
                    new_loc
                });

                //en el caso de las asignaciones si no encuentra la variable a la cual le estamos asignando valor, entonces marcamos error y no asignamos

            }
        }
    }
}

// Procedimiento que construye la tabla de símbolos recorriendo el AST en preorden
pub fn build_symtab(syntax_tree: &TreeNode, symbol_table: &mut SymbolTable) {
    traverse(
        Some(syntax_tree),
        Some(&|node| insert_node(node, symbol_table)),
        None,
    );
    symbol_table.print();
}

fn check_node(t: &TreeNode) {
    match t.node_type {
        NodeType::Expression => {
            if let Some(token) = &t.token {
                match token {
                    TokenType::PLUS
                    | TokenType::MINUS
                    | TokenType::TIMES
                    | TokenType::DIVIDE
                    | TokenType::MODULO
                    | TokenType::POWER => {
                        let left_type = &t.children[0].node_type;
                        let right_type = &t.children[1].node_type;

                        // Verificar que los operandos sean expresiones válidas
                        if left_type != &NodeType::Expression || right_type != &NodeType::Expression
                        {
                            type_error(t, "Operador aplicado a no números");
                        }

                        // Verificación de operadores relacionales
                        if matches!(
                            t.token,
                            Some(TokenType::EQ)
                                | Some(TokenType::NEQ)
                                | Some(TokenType::LT)
                                | Some(TokenType::LTE)
                                | Some(TokenType::GT)
                                | Some(TokenType::GTE)
                        ) {
                            // Si es un operador relacional, establecemos el tipo de nodo como Error
                            t.borrow_mut().node_type = NodeType::Error;
                        }
                    }
                    _ => {}
                }
            }
        }
        NodeType::IfStatement => {
            // Verificación de que la condición del if sea una expresión válida
            if t.children[0].node_type != NodeType::Expression {
                type_error(&t.children[0], "La condición del 'if' no es booleana");
            }
        }
        NodeType::Assignment => {
            // Verificación de que la asignación sea de un valor válido
            if t.children[0].node_type != NodeType::Expression {
                type_error(&t.children[0], "Asignación de valor no válido");
            }
        }
        NodeType::WriteStatement => {
            // Verificación de que el valor a escribir sea válido
            if t.children[0].node_type != NodeType::Expression {
                type_error(&t.children[0], "Escritura de valor no válido");
            }
        }
        NodeType::DoWhileStatement => {
            // Verificación de que la condición del do-while sea booleana
            if t.children[1].node_type != NodeType::Expression {
                type_error(&t.children[1], "La condición del 'do-while' no es booleana");
            }
        }
        NodeType::RepeatUntilStatement => {
            // Verificación de que la condición del repeat-until sea booleana
            if t.children[1].node_type != NodeType::Expression {
                type_error(
                    &t.children[1],
                    "La condición del 'repeat-until' no es booleana",
                );
            }
        }
        _ => {}
    }
}
// Procedimiento para realizar la verificación de tipos
pub fn type_check(syntax_tree: Option<&TreeNode>) {
    traverse(syntax_tree, None, Some(&check_node));
}
