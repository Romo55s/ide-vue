use serde::{Serialize, Deserialize};

const SIZE: usize = 211;
const SHIFT: usize = 4;

// Estructura para almacenar números de línea
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineList {
    pub lineno: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketList {
    pub name: String,
    pub lines: Vec<LineList>, // Lista de líneas donde se usa el símbolo
    pub memloc: usize,        // Ubicación en memoria
}

// Implementación de la tabla de símbolos
pub struct SymbolTable {
    table: Vec<Option<BucketList>>, // Cambiado a Vec para una mejor gestión
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: vec![None; SIZE], // Inicializa la tabla con None
        }
    }

    // Función hash
    fn hash(&self, key: &str) -> usize {
        let mut temp = 0;
        for c in key.chars() {
            temp = ((temp << SHIFT) + c as usize) % SIZE;
        }
        temp
    }

    // Inserta un símbolo en la tabla
    pub fn insert(&mut self, name: &str, lineno: usize, loc: usize) {
        let h = self.hash(name);
        let bucket = &mut self.table[h];

        // Si el símbolo no está en la tabla
        if bucket.is_none() {
            let new_bucket = BucketList {
                name: name.to_string(),
                lines: vec![LineList { lineno }], // Inicializa con la línea
                memloc: loc,
            };
            *bucket = Some(new_bucket);
        } else {
            let current = bucket.as_mut().unwrap();
            // Verifica si el símbolo ya existe
            if current.name == name {
                // Comprueba si la línea ya está registrada
                if !current.lines.iter().any(|line| line.lineno == lineno) {
                    current.lines.push(LineList { lineno });
                }
            } else {
                // Si el símbolo no coincide, crea un nuevo bucket
                let _new_bucket = BucketList {
                    name: name.to_string(),
                    lines: vec![LineList { lineno }],
                    memloc: loc,
                };
                // Manejo de colisiones simple, añadiendo nuevos buckets en el mismo índice
                current.lines.push(LineList { lineno });
            }
        }
    }

    // Busca la ubicación de memoria de un símbolo
    pub fn lookup(&self, name: &str) -> Option<usize> {
        let h = self.hash(name);
        let current = &self.table[h];

        if let Some(bucket) = current {
            if bucket.name == name {
                return Some(bucket.memloc);
            }
        }
        None
    }

    // Imprime la tabla de símbolos
    pub fn print(&self) {
        println!("Variable Name    Location   Line Numbers");
        println!("---------------   --------   ------------");
        for bucket in &self.table {
            if let Some(bucket) = bucket {
                print!("{} ", bucket.name);
                print!("{:<10} ", bucket.memloc);
                for line in &bucket.lines {
                    print!("{} ", line.lineno);
                }
                println!();
            }
        }
    }
}