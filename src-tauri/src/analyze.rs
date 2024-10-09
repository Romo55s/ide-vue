static mut TYPE_ERRORS: Vec<String> = Vec::new(); //arreglo global para almacenar los errores

use crate::globals::{NodeType, TokenType, TreeNode};
use crate::symTab::SymbolTable;


// Estructura para manejar el tipo de error en los nodos
pub fn type_error(t: &TreeNode, message: &str) {
    let error_message = format!(
        "Type error en la línea {}: {}",
        t.value.clone().unwrap_or_default(),
        message
    );
    unsafe {
        TYPE_ERRORS.push(error_message);
    }
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

fn traverse_mut(
    t: Option<&mut TreeNode>,
    pre_proc: Option<&dyn Fn(&mut TreeNode)>,
    post_proc: Option<&dyn Fn(&mut TreeNode)>,
) {
    if let Some(node) = t {
        if let Some(pre) = pre_proc {
            pre(node);
        }

        for child in &mut node.children {
            traverse_mut(Some(child), pre_proc, post_proc);
        }

        if let Some(post) = post_proc {
            post(node);
        }
    }
}



// Función para insertar nodos en la tabla de símbolos y verificar declaración
fn insert_node(t: &mut TreeNode, mut symbol_table:SymbolTable) {
    match t.node_type {
        // Manejo de declaraciones de variables (integer y double)
        NodeType::IntStatement | NodeType::DoubleStatement | NodeType::FloatStatement => {
            if let Some(ref name) = t.value {
                let lineno = t.token.clone().unwrap() as usize;

                // Inserta en la tabla de símbolos si no está
                if symbol_table.lookup(name).is_none() {
                    let new_loc = symbol_table.next_location();
                    let token_type = match t.token {
                        Some(ref token) => format!("{:?}", token), // Convertir a String o hacer otro procesamiento
                        None => "None".to_string(),
                    };
                    
                    symbol_table.insert(name, &token_type , "0", lineno, new_loc);
                } else {
                    type_error(t, &format!("La variable '{}' ya está declarada", name)); //opcional
                }
            }
        }

        // Manejo de asignaciones
        NodeType::Assignment => {
            if let Some(ref name) = t.value {
                let lineno = t.token.clone().unwrap() as usize;

                // Verificar que la variable esté declarada antes de usarla
                if let Some(loc) = symbol_table.lookup(name) {
                    let token_type = match t.token {
                        Some(ref token) => format!("{:?}", token), // Convertir a String o hacer otro procesamiento
                        None => "None".to_string(),
                    };
                    let token_value = match t.value {
                        Some(ref value) => format!("{:?}", value), // Convertir a String o hacer otro procesamiento
                        None => "None".to_string(),
                    };

                    symbol_table.insert(name, &token_type , &token_value, lineno, loc.memloc);
                } else {
                    type_error(t, &format!("Variable '{}' usada sin declarar", name));
                }
            }
        }
        _ => {}
    }
}

// Procedimiento que construye la tabla de símbolos recorriendo el AST en preorden
pub fn build_symtab(syntax_tree: &mut TreeNode, symbol_table: SymbolTable) {
    traverse_mut(
        Some(syntax_tree),
        Some(&mut |node| insert_node(node, symbol_table.clone())),
        None,
    );
    symbol_table.print();
}

// Inferencia de tipos en las expresiones
fn infer_type(t: &TreeNode) -> NodeType {
    match t.node_type {
        NodeType::Expression => {
            if let Some(token) = &t.token {
                // Inferir tipos de los operandos
                let left_type = infer_type(&t.children[0]);
                let right_type = infer_type(&t.children[1]);

                // Inferencia de tipo basado en operandos
                match (left_type, right_type) {
                    (NodeType::IntStatement, NodeType::IntStatement) => NodeType::IntStatement,
                    (NodeType::DoubleStatement, NodeType::DoubleStatement) => NodeType::DoubleStatement,
                    (NodeType::IntStatement, NodeType::DoubleStatement) => NodeType::DoubleStatement,
                    (NodeType::DoubleStatement, NodeType::IntStatement) => NodeType::DoubleStatement,
                    (NodeType::FloatStatement, NodeType::FloatStatement) => NodeType::FloatStatement,
                    (NodeType::FloatStatement, NodeType::DoubleStatement) => NodeType::FloatStatement,
                    (NodeType::DoubleStatement, NodeType::FloatStatement) => NodeType::FloatStatement,
                    (NodeType::FloatStatement, NodeType::IntStatement) => NodeType::FloatStatement,
                    (NodeType::IntStatement, NodeType::FloatStatement) => NodeType::FloatStatement,
                    _ => NodeType::Error, // Tipos incompatibles
                }
            } else {
                NodeType::Error
            }
        }
        NodeType::IntStatement | NodeType::DoubleStatement | NodeType:: FloatStatement => t.node_type.clone(),
        _ => NodeType::Error, // Otros tipos no soportados
    }
}


fn eval_constant_expr(t: &TreeNode, symbol_table: SymbolTable) -> Option<f64> {
    if t.node_type == NodeType::Expression {
        if let Some(token) = &t.token {
            match token {
                TokenType::PLUS => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    if left.fract() == 0.0 && right.fract() == 0.0 {
                        // Si ambos son enteros, retornar como entero
                        Some((left as i32 + right as i32) as f64)
                    } else {
                        // Si uno es flotante, retornar como flotante
                        Some(left + right)
                    }
                }
                TokenType::MINUS => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    if left.fract() == 0.0 && right.fract() == 0.0 {
                        Some((left as i32 - right as i32) as f64)
                    } else {
                        Some(left - right)
                    }
                }
                TokenType::TIMES => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    if left.fract() == 0.0 && right.fract() == 0.0 {
                        Some((left as i32 * right as i32) as f64)
                    } else {
                        Some(left * right)
                    }
                }
                TokenType::DIVIDE => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    if right != 0.0 {
                        Some(left / right)
                    } else {
                        type_error(t, "División por cero");
                        None
                    }
                }
                TokenType::MODULO => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    if left.fract() == 0.0 && right.fract() == 0.0 {
                        Some((left as i32 % right as i32) as f64)
                    } else {
                        Some(left % right)
                    }
                }
                TokenType::POWER => {
                    let left = eval_constant_expr(&t.children[0], symbol_table.clone())?;
                    let right = eval_constant_expr(&t.children[1], symbol_table.clone())?;
                    Some(left.powf(right))
                }
                _ => None,
            }
        } else {
            None
        }
    } else if t.node_type == NodeType::IntStatement {
        // Si es un número entero, retornarlo como i32
        t.value.as_ref().and_then(|v| v.parse::<i32>().ok()).map(|val| val as f64)
    } else if t.node_type == NodeType::DoubleStatement {
        // Si es un número flotante, retornarlo como f64
        t.value.as_ref().and_then(|v| v.parse::<f64>().ok())
    } else if let Some(ref name) = t.value {
        // Si es una variable, buscar el valor en la tabla de símbolos
        if let Some(entry) = symbol_table.lookup(name) {
            // Intentar como flotante primero, si falla intentar como entero
            entry.value.parse::<f64>().or_else(|_| entry.value.parse::<i32>().map(|val| val as f64)).ok()
        } else {
            None
        }
    } else {
        None
    }
}


// Procedimiento para realizar la verificación de tipos y evaluación de expresiones
fn check_node(t: &mut TreeNode, symbol_table: SymbolTable) {
    match t.node_type {
        NodeType::Expression => {
            let inferred_type = infer_type(t);
            if inferred_type == NodeType::Error {
                type_error(t, "Tipos incompatibles en la expresión");
            }

            // Intentar evaluar la expresión si es constante
            if let Some(result) = eval_constant_expr(t, symbol_table.clone()) {
                t.value = Some(result.to_string());
            }
        }
        NodeType::Assignment => {
            if let Some(ref name) = t.value {
                // Verificación de que la variable esté declarada
                if symbol_table.lookup(name).is_none() {
                    type_error(t, &format!("Variable '{}' no está declarada", name));
                }
            }
        }
        _ => {}
    }
}

// Procedimiento para realizar la verificación de tipos y evaluación de expresiones
pub fn type_check(syntax_tree: Option<&mut TreeNode>, symbol_table: SymbolTable) {
    traverse_mut(syntax_tree, None, Some(&mut |node: &mut TreeNode| check_node(node, symbol_table.clone())));
}

pub fn analyze_syntax_tree(syntax_tree: &mut TreeNode, symbol_table: SymbolTable) -> (TreeNode, Vec<String>) {
    // Paso 1: Construcción de la tabla de símbolos
    build_symtab(syntax_tree, symbol_table.clone());

    // Paso 2: Verificación de tipos y anotaciones
    type_check(Some(syntax_tree), symbol_table.clone());

    // Retornar el árbol anotado y los errores encontrados
    unsafe {
        (syntax_tree.clone(), TYPE_ERRORS.clone())
    }
}
