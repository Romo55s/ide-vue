use serde::{Deserialize, Serialize};

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
    pub tipo: String,
    pub lines: Vec<LineList>, // Lista de líneas donde se usa el símbolo
    pub memloc: usize,        // Ubicación en memoria
}

// Implementación de la tabla de símbolos
pub struct SymbolTable {
    table: Vec<Option<BucketList>>, // Cambiado a Vec para una mejor gestión
    next_loc: usize,                // Siguiente ubicación de memoria
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: vec![None; SIZE], // Inicializa la tabla con None
            next_loc: 0,             // Inicializa la ubicación en 0
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
    pub fn insert(&mut self, name: &str, tipo: &str, lineno: usize, loc: usize) {
        let h = self.hash(name);
        let bucket = &mut self.table[h];

        // Si el símbolo no está en la tabla
        if bucket.is_none() {
            let new_bucket = BucketList {
                name: name.to_string(),
                tipo: tipo.to_string(),
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
                // Manejo de colisiones simple, añadiendo nuevos buckets en el mismo índice
                // Aquí puedes implementar un mejor manejo de colisiones si lo deseas
                // Por simplicidad, solo agregamos la línea al bucket existente
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

    // Obtiene la siguiente ubicación de memoria y la incrementa
    pub fn next_location(&mut self) -> usize {
        let loc = self.next_loc;
        self.next_loc += 1;
        loc
    }

    // Imprime la tabla de símbolos
    pub fn print(&self) {
        println!("Variable Name     Tipo        Location   Line Numbers");
        println!("---------------   ----------   --------   ------------");
        for bucket in &self.table {
            if let Some(bucket) = bucket {
                print!(
                    "{:<15} {:<15} {:<10} ",
                    bucket.name, bucket.tipo, bucket.memloc
                );
                for line in &bucket.lines {
                    print!("{} ", line.lineno);
                }
                println!();
            }
        }
    }
}
