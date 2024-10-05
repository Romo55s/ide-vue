use std::cell::RefCell;
use std::rc::Rc;
use crate::globals::{NodeType, TokenType, TreeNode};
use crate::symTab::{insert, lookup, print};

// Estado inicial para la ubicación de memoria de las variables
static mut LOCATION: i32 = 0;

// Estructura para manejar el tipo de error en los nodos
pub fn type_error(t: &TreeNode, message: &str) {
    println!("Type error en la línea {}: {}", t.value.clone().unwrap_or_default(), message);
}

// Procedimiento para insertar identificadores en la tabla de símbolos
fn insert_node(t: Rc<RefCell<TreeNode>>) {
    let node_type = t.borrow().node_type.clone();
    
    match node_type {
        NodeType::Assignment | 
        NodeType::ReadStatement |
        NodeType::WriteStatement => {
            let name = t.borrow().value.clone().unwrap();
            let lineno = t.borrow().value.clone().unwrap().parse::<i32>().unwrap();
            unsafe {
                if lookup(&name) == -1 {
                    // No está en la tabla, se trata como una nueva definición
                    insert(&name, lineno, LOCATION);
                    LOCATION += 1;
                } else {
                    // Ya está en la tabla, solo se agrega la línea de uso
                    insert(&name, lineno, 0);
                }
            }
        }
        NodeType::Expression => {
            if let Some(TokenType::ID) = t.borrow().token {
                let name = t.borrow().value.clone().unwrap();
                let lineno = t.borrow().value.clone().unwrap().parse::<i32>().unwrap();
                unsafe {
                    if lookup(&name) == -1 {
                        // No está en la tabla, se trata como una nueva definición
                        insert(&name, lineno, LOCATION);
                        LOCATION += 1;
                    } else {
                        // Ya está en la tabla, solo se agrega la línea de uso
                        insert(&name, lineno, 0);
                    }
                }
            }
        }
        _ => {}
    }
}

// Función para recorrer el árbol de sintaxis abstracta en preorden y postorden
fn traverse(
    t: Option<Rc<RefCell<TreeNode>>>,
    pre_proc: Option<&dyn Fn(Rc<RefCell<TreeNode>>)>,
    post_proc: Option<&dyn Fn(Rc<RefCell<TreeNode>>)>,
) {
    if let Some(node) = t {
        if let Some(pre) = pre_proc {
            pre(node.clone());
        }
        for child in node.borrow().children.iter() {
            traverse(Some(Rc::new(RefCell::new(child.clone()))), pre_proc, post_proc);
        }
        if let Some(post) = post_proc {
            post(node);
        }
    }
}

// Procedimiento que construye la tabla de símbolos recorriendo el AST en preorden
pub fn build_symtab(syntax_tree: Option<Rc<RefCell<TreeNode>>>) {
    traverse(syntax_tree, Some(&insert_node), None);
    print();
}

// Procedimiento para verificar el tipo de un nodo específico
fn check_node(t: Rc<RefCell<TreeNode>>) {
    match t.borrow().node_type {
        NodeType::Expression => {
            if let Some(TokenType::PLUS) |
                Some(TokenType::MINUS) | 
                Some(TokenType::TIMES) | 
                Some(TokenType::DIVIDE) |
                Some(TokenType::MODULO) |
                Some(TokenType::POWER) = t.borrow().token {
                
                let left_type = t.borrow().children[0].borrow().node_type.clone();
                let right_type = t.borrow().children[1].borrow().node_type.clone();
                
                if left_type != NodeType::Expression || right_type != NodeType::Expression {
                    type_error(&t.borrow(), "Operador aplicado a no números");
                }

                if matches!(t.borrow().token,
                Some(TokenType::EQ) | 
                Some(TokenType::NEQ) | 
                Some(TokenType::LT) | 
                Some(TokenType::LTE) | 
                Some(TokenType::GT) | 
                Some(TokenType::GTE)) {
                    t.borrow_mut().node_type = NodeType::Error;
                }
            }
        }
        NodeType::IfStatement => {
            if t.borrow().children[0].borrow().node_type != NodeType::Expression {
                type_error(&t.borrow().children[0], "La condición del 'if' no es booleana");
            }
        }
        NodeType::Assignment => {
            if t.borrow().children[0].borrow().node_type != NodeType::Expression {
                type_error(&t.borrow().children[0], "Asignación de valor no válido");
            }
        }
        NodeType::WriteStatement => {
            if t.borrow().children[0].borrow().node_type != NodeType::Expression {
                type_error(&t.borrow().children[0], "Escritura de valor no válido");
            }
        }
        NodeType::DoWhileStatement => {
            if t.borrow().children[1].borrow().node_type != NodeType::Expression {
                type_error(&t.borrow().children[1], "La condición del 'do-while' no es booleana");
            }
        }
        NodeType::RepeatUntilStatement => {
            if t.borrow().children[1].borrow().node_type != NodeType::Expression {
                type_error(&t.borrow().children[1], "La condición del 'repeat-until' no es booleana");
            }
        }
        _ => {}
    }
}

// Procedimiento para realizar la verificación de tipos
pub fn type_check(syntax_tree: Option<Rc<RefCell<TreeNode>>>) {
    traverse(syntax_tree, None, Some(&check_node));
}