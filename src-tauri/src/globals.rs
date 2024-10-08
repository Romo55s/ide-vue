use ::lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Escáner /////////////////////////////////////////////////////////////////////////////////////////

// Enum para representar los estados en el DFA del escáner
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StateType {
    Start,
    InAssign,
    InComment,
    InMultiComment,
    InNum,
    InReal,
    InId,
    Done,
    EndFile,
}

// Enum para representar los tipos de tokens
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TokenType {
    // Tokens de control
    ENDFILE, //cierre de archivo
    ERROR,   //error - no hay tokens que coincidan

    // Palabras reservadas
    IF,
    ELSE,
    DO,
    WHILE,
    REPEAT,
    UNTIL,
    READ,
    WRITE,
    INTEGER,
    FLOAT,
    DOUBLE,
    MAIN,
    AND,
    OR,
    RETURN,
    CIN,
    COUT,

    // Tokens de múltiples caracteres
    ID,
    NumInt,
    NumReal,

    // Operadores aritméticos
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    MODULO,
    POWER,

    // Operadores relacionales
    EQ,  // igualdad
    NEQ, // diferente
    LT,  // menor que
    LTE, // menor o igual que
    GT,  // mayor que
    GTE, // mayor o igual que

    // Símbolos especiales
    LPAREN,    // paréntesis izquierdo
    RPAREN,    // paréntesis derecho
    LBRACE,    // llave izquierda
    RBRACE,    // llave derecha
    COMMA,     // coma
    COLON,     // dos puntos
    SEMICOLON, // punto y coma
    ASSIGN,    // asignación

    //Incrementador
    INCREMENT,

    //Decrementador
    DECREMENT,

    // Símbolo de comentario múltiple no cerrado
    InMultipleComment,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Parser /////////////////////////////////////////////////////////////////////////////////////////

// Enum para representar los tipos de nodos del árbol de sintaxis abstracta
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NodeType {
    MainRoot,
    IntStatement,
    DoubleStatement,
    FloatStatement,
    Statement,
    Expression,
    Term,
    Factor,
    Assignment,
    IfStatement,
    ElseStatement,
    WhileStatement,
    WriteStatement,
    ReadStatement,
    DoWhileStatement,
    RepeatUntilStatement,
    SwitchStatement,
    CaseStatement,
    DefaultStatement,
    MainFunction,
    ReturnStatement,
    CinStatement,
    CoutStatement,
    Increment,
    Decrement,
    Error,
}

// Estructura para representar un nodo del árbol de sintaxis abstracta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub node_type: NodeType,
    pub token: Option<TokenType>,
    pub value: Option<String>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(node_type: NodeType) -> Self {
        TreeNode {
            node_type,
            token: None,
            value: None,
            children: Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// Errores /////////////////////////////////////////////////////////////////////////////////////////
lazy_static! {
    pub static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn log_error(error: String) {
    let mut errors = ERRORS.lock().unwrap();
    if !errors.contains(&error) {
        errors.push(error);
    }
}
