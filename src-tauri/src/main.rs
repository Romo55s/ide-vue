// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write; // Importa el trait Write
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
// Enum para representar los tipos de tokens
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TokenType {
    // Tokens de control
    ENDFILE, //cierre de archivo
    ERROR, //error - no hay tokens que coincidan

    // Palabras reservadas
    IF,
    ELSE,
    DO,
    WHILE,
    SWITCH,
    CASE,
    END,
    REPEAT,
    UNTIL,
    READ,
    WRITE,
    INTEGER,
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
    EQ,   // igualdad
    NEQ,  // diferente
    LT,   // menor que
    LTE,  // menor o igual que
    GT,   // mayor que
    GTE,  // mayor o igual que

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

// Enum para representar los estados en el DFA del escáner
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum StateType {
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum NodeType {
    MainRoot,
    IntStatement,
    DoubleStatement,
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
    MainFunction,
    ReturnStatement,
    CinStatement,
    CoutStatement,
    Increment,
    Decrement,
    Error
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNode {
    node_type: NodeType,
    token: Option<TokenType>,
    value: Option<String>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(node_type: NodeType) -> Self {
        TreeNode {
            node_type,
            token: None,
            value: None,
            children: Vec::new(),
        }
    }
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn log_error(error: String) {
    let mut errors = ERRORS.lock().unwrap();
    if !errors.contains(&error) {
        errors.push(error);
    }
}

// Función para obtener el siguiente carácter no en blanco de la línea actual
fn get_next_char(line: &str, linepos: &mut usize, bufsize: usize) -> char {
    if *linepos >= bufsize {
        '\0' // Devuelve un carácter nulo al final de la línea
    } else {
        let c = line.chars().nth(*linepos).unwrap_or('\0'); // Usa unwrap_or para devolver un carácter nulo si el índice está fuera de rango
        *linepos += 1;
        c
    }
}

// Función para retroceder un carácter en la línea actual
fn unget_next_char(linepos: &mut usize) {
    if *linepos > 0 {
        *linepos -= 1;
    }
}

// Función para buscar palabras reservadas y devolver su TokenType correspondiente
fn reserved_lookup(s: &str) -> TokenType {
    match s {
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "do" => TokenType::DO,
        "while" => TokenType::WHILE,
        "switch" => TokenType::SWITCH,
        "case" => TokenType::CASE,
        "end" => TokenType::END,
        "repeat" => TokenType::REPEAT,
        "until" => TokenType::UNTIL,
        "read" => TokenType::READ,
        "write" => TokenType::WRITE,
        "int" => TokenType::INTEGER,
        "double" => TokenType::DOUBLE,
        "main" => TokenType::MAIN,
        "return" => TokenType::RETURN,
        "/*" => TokenType::InMultipleComment,
        "cin" => TokenType::CIN,
        "cout" => TokenType::COUT,
        _ => TokenType::ID,
    }
}

// Función para realizar el análisis léxico y devolver los tokens
fn get_token(content: &str) -> (Vec<(TokenType, String, usize, usize)>, Vec<(TokenType, String, usize, usize)>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lineno = 1;
    let mut state = StateType::Start;
    let mut token_string = String::new();
    let mut linepos = 0;
    let bufsize = content.len();
    let mut column_number = 0;
    while linepos <= bufsize {
        let c = get_next_char(content, &mut linepos, bufsize);
        match state {
            StateType::Start => {
                if c == '\n' {
                    lineno += 1;
                    column_number = 1;
                }
                if c.is_whitespace() {
                    // Ignorar espacios en blanco
                    column_number +=1;
                } else if c.is_ascii_alphabetic() || c == '_' {
                    state = StateType::InId;
                    token_string.push(c);
                    column_number +=1;
                } else if c.is_digit(10) {
                    state = StateType::InNum;
                    token_string.push(c);
                    column_number +=1;
                } else if c == '/' {
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = StateType::InComment;
                            lineno += 1;
                        }
                    } else if next_char == '*' {
                        lineno += 1;
                        let next_char = get_next_char(content, &mut linepos, bufsize);
                        if next_char == '\n' {
                            lineno += 1;
                        } else {
                            unget_next_char(&mut linepos);
                            state = StateType::InMultiComment;
                            lineno += 1;
                        }
                    } else {
                        tokens.push((TokenType::DIVIDE, "/".to_string(), lineno, column_number - 1));
                        unget_next_char(&mut linepos)
                    }
                } else {
                    match c {
                        '=' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::EQ, "==".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::ASSIGN, "=".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '!' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::NEQ, "!=".to_string(), lineno, column_number - 1));
                            } else {
                                errors.push((TokenType::ERROR, "!".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '<' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::LTE, "<=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::LT, "<".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '>' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '=' {
                                tokens.push((TokenType::GTE, ">=".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::GT, ">".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '+' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '+' {
                                tokens.push((TokenType::INCREMENT, "++".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::PLUS, "+".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '-' => {
                            let next_char = get_next_char(content, &mut linepos, bufsize);
                            if next_char == '-' {
                                tokens.push((TokenType::DECREMENT, "--".to_string(), lineno, column_number - 1));
                            } else {
                                tokens.push((TokenType::MINUS, "-".to_string(), lineno, column_number - 1));
                                unget_next_char(&mut linepos);

                            }
                        }
                        '*' => tokens.push((TokenType::TIMES, "*".to_string(), lineno, column_number)),
                        '%' => tokens.push((TokenType::MODULO, "%".to_string(), lineno, column_number)),
                        '^' => tokens.push((TokenType::POWER, "^".to_string(), lineno, column_number)),
                        '(' => tokens.push((TokenType::LPAREN, "(".to_string(), lineno, column_number)),
                        ')' => tokens.push((TokenType::RPAREN, ")".to_string(), lineno, column_number)),
                        '{' => tokens.push((TokenType::LBRACE, "{".to_string(), lineno, column_number)),
                        '}' => tokens.push((TokenType::RBRACE, "}".to_string(), lineno, column_number)),
                        ',' => tokens.push((TokenType::COMMA, ",".to_string(), lineno, column_number)),
                        ';' => tokens.push((TokenType::SEMICOLON, ";".to_string(), lineno, column_number)),
                        '&' => tokens.push((TokenType::AND, "&".to_string(), lineno, column_number)),
                        '|' => tokens.push((TokenType::OR, "|".to_string(), lineno, column_number)),
                        ':' => tokens.push((TokenType::COLON, ":".to_string(), lineno, column_number)),
                        '\0' => {
                            state = StateType::EndFile;
                        }
                        _ => errors.push((TokenType::ERROR, c.to_string(), lineno, column_number - 1)),
                    }
                }
            }
            StateType::InId => {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token_string.push(c);
                } else {
                    tokens.push((reserved_lookup(&token_string), token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InNum => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if c == '.' {
                    state = StateType::InReal;
                    token_string.push(c);
                } else {
                    tokens.push((TokenType::NumInt, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InReal => {
                if c.is_digit(10) {
                    token_string.push(c);
                } else if token_string.ends_with('.') {
                    errors.push((TokenType::ERROR, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); //retornar un carácter
                } else {
                    tokens.push((TokenType::NumReal, token_string.clone(), lineno, (column_number - 1)));
                    token_string.clear();
                    state = StateType::Start;
                    unget_next_char(&mut linepos); // Retornar un carácter
                }
            }
            StateType::InComment => {
                if c == '\n' || c == '\0' {
                    state = StateType::Start;
                    column_number = 1;
                }
            }
            StateType::InMultiComment => {
                if c == '*' {
                    lineno += 1;
                    let next_char = get_next_char(content, &mut linepos, bufsize);
                    if next_char == '/' {
                        state = StateType::Start;
                        lineno += 1;
                    } else {
                        unget_next_char(&mut linepos)
                    }
                } else if c == '\0' {
                    tokens.push((TokenType::InMultipleComment, "/*".to_string(), lineno, column_number - 1));
                    println!("Error: '/*' Multiline comment not closed.");
                    state = StateType::EndFile;
                }
            }
            StateType::EndFile => {
                tokens.push((TokenType::ENDFILE, "\0".to_string(), lineno, column_number - 1));
                break; // Salir del bucle while
            }
            _ => (),
        }
    }
    (tokens, errors)
}
#[tauri::command]
fn lexic(content: String) -> Result<(Vec<(TokenType, String, usize, usize)>, Vec<(TokenType, String, usize, usize)>), String> {
    Ok(get_token(&content))
}

fn match_token(tokens: &[(TokenType, String, usize, usize)], expected: TokenType, current_token: &mut usize) -> Result<(), String> {
    if *current_token < tokens.len() && tokens[*current_token].0 == expected {
        *current_token += 1;
        Ok(())
    } else {
        println!("token in match: {:?}", tokens.get(*current_token));
        Err(format!("Error de sintaxis: se esperaba {:?} en la posición {:?}", expected, tokens.get(*current_token)))
    }
}

fn parse_program(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize, errors: &mut Vec<String>) -> Result<TreeNode, String> {
    let mut root = TreeNode::new(NodeType::MainRoot);
    while *current_token < tokens.len() && tokens[*current_token].0 != TokenType::ENDFILE {
        match parse_statement(tokens, current_token) {
            Ok(statement_node) => root.children.push(statement_node),
            Err(err) => errors.push(err.to_string()), // Convertir el error en una cadena antes de agregarlo al vector
        }
    }

    Ok(root)
}
fn parse_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    match tokens.get(*current_token) {
        Some((TokenType::ID, _, _, _)) => {
            if let Some((TokenType::INCREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_increment_statement(tokens, current_token);
            } else if let Some((TokenType::DECREMENT, _, _, _)) = tokens.get(*current_token + 1) {
                return parse_decrement_statement(tokens, current_token);
            }
        }
        _ => {}
    }

   // match tokens.get(*current_token) {
    //    Some((TokenType::NumInt, _, _, _)) => {
      //      println!("token with error: {:?}", tokens.get(*current_token));
     //       return Err("Error de sintaxis: se esperaba un identificador con asignación antes".to_string());
     //   }
     //   _ => {}
   // }
    
    match tokens.get(*current_token) {
        Some((TokenType::COLON, _, _, _)) => {
            *current_token+=1;
            return Err("Error de sintaxis: token fuera de un case ':'".to_string());
        }
        _ => {}
    }


    match tokens.get(*current_token) {
        Some((TokenType::IF, _, _, _)) => return parse_if_statement(tokens, current_token),
        Some((TokenType::WHILE, _, _, _)) => return parse_while_statement(tokens, current_token),
        Some((TokenType::WRITE, _, _, _)) => return parse_write_statement(tokens, current_token),
        Some((TokenType::READ, _, _, _)) => return parse_read_statement(tokens, current_token),
        Some((TokenType::DO, _, _, _)) => return parse_do_while_statement(tokens, current_token),
        Some((TokenType::REPEAT, _, _, _)) => return parse_repeat_until_statement(tokens, current_token),
        Some((TokenType::SWITCH, _, _, _)) => return parse_switch_statement(tokens, current_token),
        Some((TokenType::RETURN, _, _, _)) => return parse_return_statement(tokens, current_token),
        Some((TokenType::CIN, _, _, _)) => return parse_cin_statement(tokens, current_token),
        Some((TokenType::COUT, _, _, _)) => return parse_cout_statement(tokens, current_token),
        Some((TokenType::MAIN, _, _, _)) => return parse_main_function(tokens, current_token),
        Some((TokenType::INTEGER, _, _, _)) => return parse_int_variable_declaration(tokens, current_token),
        Some((TokenType::DOUBLE, _, _, _)) => return parse_double_variable_declaration(tokens, current_token),
        Some((TokenType::ID, _, _, _)) => {
            let assignment_node = parse_assignment(tokens, current_token)?;
            if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
                *current_token += 1;
                return Ok(assignment_node);
            } else {
                return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
            }
        }
        _ => if is_part_of_expression(tokens, current_token) {
            println!("token: {:?}",tokens.get(*current_token));
            return Err(format!("Error de sintaxis: se esperaba una asignación a un identificador antes de la posición {:?}", tokens.get(*current_token)));
        } else {
            return Err(format!("Error de sintaxis: token inesperado {:?}", tokens.get(*current_token)));
        },
    }
}

fn is_part_of_expression(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> bool {
    if parse_expression(tokens, current_token).is_ok() {
        return true;
    }
    false
}


fn parse_int_variable_declaration(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::IntStatement);

    // Parsear la palabra clave 'int'
    match_token(tokens, TokenType::INTEGER, current_token)?;

    // Parsear los identificadores
    loop {
        match tokens.get(*current_token) {
            Some((TokenType::ID, id, _, _)) => {
                node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_double_variable_declaration(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::DoubleStatement);

    match_token(tokens, TokenType::DOUBLE, current_token)?;
    loop {
        match tokens.get(*current_token) {
            Some((TokenType::ID, id, _, _)) => {
                node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(TokenType::ID),
                    value: Some(id.clone()),
                    children: Vec::new(),
                });
                *current_token += 1;
                if let Some((TokenType::COMMA, _, _, _)) = tokens.get(*current_token) {
                    *current_token += 1; // Avanzar si hay una coma
                } else {
                    break; // Salir del bucle si no hay más identificadores
                }
            }
            _ => return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token))),
        }
    }

    // Verificar si hay un punto y coma al final
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1; // Avanzar si hay un punto y coma
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}


fn parse_if_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::IfStatement);
    match_token(tokens, TokenType::IF, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if let Some((TokenType::ELSE, _, _, _)) = tokens.get(*current_token) {
        let else_node = parse_else_statement(tokens, current_token);
        match else_node {
            Ok(else_node) => {
                node.children.push(else_node);
            }
            Err(err) => {
                log_error(err.to_string());
            }
        }
    }
    Ok(node)
}

fn parse_else_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ElseStatement);
    match_token(tokens, TokenType::ELSE, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}


fn parse_do_while_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::DoWhileStatement);
    match_token(tokens, TokenType::DO, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::WHILE, current_token) {
        log_error(err.to_string());
    }
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}


fn parse_while_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::WhileStatement);
    match_token(tokens, TokenType::WHILE, current_token)?;
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_repeat_until_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::RepeatUntilStatement);
    match_token(tokens, TokenType::REPEAT, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::UNTIL, current_token) {
        log_error(err.to_string());
    }
    let condition_node = parse_expression(tokens, current_token)?;
    node.children.push(condition_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_switch_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::SwitchStatement);
    match_token(tokens, TokenType::SWITCH, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let mut case_found = false;
    let mut end_found = false;
    while let Some((token_type, _, _, _)) = tokens.get(*current_token) {
        match token_type {
            TokenType::CASE => {
                case_found = true;
                let case_node = parse_case_statement(tokens, current_token)?;
                node.children.push(case_node);
            }
            TokenType::END => {
                end_found = true;
                *current_token += 1;
                break;
            }
            _ => return Err(format!("Error de sintaxis: token inesperado {:?}", tokens.get(*current_token))),
        }
    }
    if !case_found {
        return Err("Error de sintaxis: se esperaba la palabra reservada 'CASE' en el bloque switch".to_string());
    }
    if !end_found {
        return Err("Error de sintaxis: se esperaba la palabra reservada 'END' en el bloque switch".to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_case_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::CaseStatement);
    match_token(tokens, TokenType::CASE, current_token)?;
    let value_node = parse_expression(tokens, current_token)?;
    node.children.push(value_node);
    if  let Err(err) = match_token(tokens, TokenType::COLON, current_token) {
        log_error(err.to_string());
    }
    while let Some((token_type, _, _, _)) = tokens.get(*current_token) {
        if token_type == &TokenType::END || token_type == &TokenType::CASE {
            break;
        } else {
                let statement_node = parse_statement(tokens, current_token);
                match statement_node {
                    Ok(statement_node) => {
                        node.children.push(statement_node);
                    }
                    Err(err) => {
                        log_error(err.to_string());
                    }
                }
        }
    }
    Ok(node)
}


fn parse_main_function(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::MainFunction);
    match_token(tokens, TokenType::MAIN, current_token)?;
    if  let Err(err) = match_token(tokens, TokenType::LPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::RPAREN, current_token) {
        log_error(err.to_string());
    }
    if  let Err(err) = match_token(tokens, TokenType::LBRACE, current_token) {
        log_error(err.to_string());
    }
    let statement_node = parse_statement(tokens, current_token);
    match statement_node {
        Ok(statement_node) => {
            node.children.push(statement_node);
        }
        Err(err) => {
            log_error(err.to_string());
        }
    }
    if  let Err(err) = match_token(tokens, TokenType::RBRACE, current_token) {
        log_error(err.to_string());
    }
    Ok(node)
}

fn parse_write_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::WriteStatement);
    match_token(tokens, TokenType::WRITE, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_read_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ReadStatement);
    match_token(tokens, TokenType::READ, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_return_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::ReturnStatement);
    match_token(tokens, TokenType::RETURN, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_cin_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::CinStatement);
    match_token(tokens, TokenType::CIN, current_token)?;
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)));
    }
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token))
    }
}

fn parse_cout_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::CoutStatement);
    match_token(tokens, TokenType::COUT, current_token)?;
    let expression_node = parse_expression(tokens, current_token)?;
    node.children.push(expression_node);
    if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
        *current_token += 1;
    } else {
        return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
    }
    Ok(node)
}

fn parse_increment_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Increment);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}

fn parse_decrement_statement(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Decrement);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 2;
        if let Some((TokenType::SEMICOLON, _, _, _)) = tokens.get(*current_token) {
            *current_token += 1;
        } else {
            return Err(format!("Error de sintaxis: se esperaba ';' en la posición {:?}", *current_token));
        }
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}


fn parse_expression(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = parse_term(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            TokenType::PLUS | TokenType::MINUS | TokenType::LT | TokenType::LTE | TokenType::GT | TokenType::GTE | TokenType::EQ | TokenType::NEQ | TokenType::AND | TokenType::OR => {
                *current_token += 1;
                let term_node = parse_term(tokens, current_token)?;
                let mut expression_node = TreeNode::new(NodeType::Expression);
                expression_node.children.push(node);
                expression_node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(token.clone()),
                    value: Some(value.clone()),
                    children: Vec::new(),
                });
                expression_node.children.push(term_node);
                node = expression_node;
            }
            _ => break,
        }
    }
    Ok(node)
}

fn parse_term(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = parse_factor(tokens, current_token)?;
    while let Some((token, value, _, _)) = tokens.get(*current_token) {
        match token {
            TokenType::TIMES | TokenType::DIVIDE | TokenType::MODULO | TokenType::POWER => {
                *current_token += 1;
                let factor_node = parse_factor(tokens, current_token)?;
                let mut term_node = TreeNode::new(NodeType::Term);
                term_node.children.push(node);
                term_node.children.push(TreeNode {
                    node_type: NodeType::Factor,
                    token: Some(token.clone()),
                    value: Some(value.clone()),
                    children: Vec::new(),
                });
                term_node.children.push(factor_node);
                node = term_node;
            }
            _ => break,
        }
    }
    Ok(node)
}

fn parse_factor(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    if let Some((token, value, _, _)) = tokens.get(*current_token) {
        let mut node = TreeNode::new(NodeType::Factor);
        match token {
            TokenType::NumInt | TokenType::NumReal | TokenType::ID => {
                node.token = Some(token.clone());
                node.value = Some(value.clone());
                *current_token += 1;
                Ok(node)
            }
            TokenType::LPAREN => {
                *current_token += 1;
                let expression_node = parse_expression(tokens, current_token)?;
                if  let Err(err) = match_token(tokens, TokenType::RPAREN, current_token) {
                    log_error(err.to_string());
                }
                node.children.push(expression_node);
                Ok(node)
            }
            _ => Err(format!("Error de sintaxis: token inesperado {:?}", tokens.get(*current_token))),
        }
    } else {
        Err(format!("Error de sintaxis: token inesperado en la posición {:?}", tokens.get(*current_token)))
    }
}

fn parse_assignment(tokens: &[(TokenType, String, usize, usize)], current_token: &mut usize) -> Result<TreeNode, String> {
    let mut node = TreeNode::new(NodeType::Assignment);
    if let Some((TokenType::ID, id, _, _)) = tokens.get(*current_token) {
        node.children.push(TreeNode {
            node_type: NodeType::Factor,
            token: Some(TokenType::ID),
            value: Some(id.clone()),
            children: Vec::new(),
        });
        *current_token += 1;
        if  let Err(err) = match_token(tokens, TokenType::ASSIGN, current_token) {
            log_error(err.to_string());
        }
        let expression_node = parse_expression(tokens, current_token)?;
        node.children.push(expression_node);
        Ok(node)
    } else {
        Err(format!("Error de sintaxis: se esperaba un identificador en la posición {:?}", tokens.get(*current_token)))
    }
}

#[tauri::command]
fn parse(tokens: Vec<(TokenType, String, usize, usize)>) -> Result<(TreeNode, Vec<String>), String> {
    let mut errors_str: Vec<String> = Vec::new();
    let mut current_token = 0;
    let syntax_tree = match parse_program(&tokens, &mut current_token, &mut errors_str) {
        Ok(tree) => tree,
        Err(err) => {
            log_error(err.to_string());
            TreeNode::new(NodeType::Error)
        }
    };
    let errors_from_global = {
        let mut errors = ERRORS.lock().unwrap();
        let errors_clone = errors.clone();
        errors.clear(); // Clear errors after retrieval
        errors_clone
    };

    // Combine local and global errors
    errors_str.extend(errors_from_global.iter().cloned());
    Ok((syntax_tree, errors_str))
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_file, remove_file, lexic, parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(path: String, contents: String) -> Result<(), String> {
    match save_file_or_save_as(&path, &contents) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Error al guardar el archivo: {}", e)),
    }
}

#[tauri::command]
async fn remove_file(path: String) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

fn save_file_or_save_as(path: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
  }
