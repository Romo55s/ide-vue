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
    pub _type: String,
    pub value: String,
    pub lines: Vec<LineList>, // Lista de líneas donde se usa el símbolo
    pub memloc: usize,        // Ubicación en memoria
}

// Implementación de la tabla de símbolos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolTable {
    table: Vec<Option<Vec<BucketList>>>, // Cada posición es una lista de buckets (para encadenamiento)
    next_loc: usize,                     // Siguiente ubicación de memoria
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: vec![None; SIZE], // Inicializa la tabla con None en cada posición
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

    // Inserta un símbolo en la tabla con manejo de colisiones por encadenamiento
    pub fn insert(&mut self, name: &str, _type: &str, value: &str, lineno: usize, loc: usize) {
        let h = self.hash(name);
        let bucket_list = &mut self.table[h];

        // Si no hay ningún bucket en esta posición, crea una nueva lista
        if bucket_list.is_none() {
            let new_bucket = BucketList {
                name: name.to_string(),
                _type: _type.to_string(),
                value: value.to_string(),
                lines: vec![LineList { lineno }], // Inicializa con la línea
                memloc: loc,
            };
            *bucket_list = Some(vec![new_bucket]); // Crea una lista de buckets
        } else {
            let bucket_vec = bucket_list.as_mut().unwrap();

            // Busca si el símbolo ya está en la lista de buckets
            if let Some(bucket) = bucket_vec.iter_mut().find(|b| b.name == name) {
                // Si ya existe, actualiza la lista de líneas
                if !bucket.lines.iter().any(|line| line.lineno == lineno) {
                    bucket.lines.push(LineList { lineno });
                }
            } else {
                // Si no existe, añade un nuevo bucket a la lista
                let new_bucket = BucketList {
                    name: name.to_string(),
                    _type: _type.to_string(),
                    value: value.to_string(),
                    lines: vec![LineList { lineno }],
                    memloc: loc,
                };
                bucket_vec.push(new_bucket); // Añade el nuevo bucket a la lista
            }
        }
    }

    // Busca la ubicación de memoria de un símbolo
    pub fn lookup(&self, name: &str) -> Option<&BucketList> {
        let h = self.hash(name);
        let bucket_list = &self.table[h];

        // Si hay una lista de buckets en esa posición, buscar en ella
        if let Some(bucket_vec) = bucket_list {
            for bucket in bucket_vec {
                if bucket.name == name {
                    return Some(bucket); // Retorna el bucket encontrado
                }
            }
        }
        None // Si no se encuentra, retorna None
    }

    // Obtiene la siguiente ubicación de memoria y la incrementa
    pub fn next_location(&mut self) -> usize {
        let loc = self.next_loc;
        self.next_loc += 1;
        loc
    }

    // Imprime la tabla de símbolos
    pub fn print(&self) {
        println!("Variable Name     Type            Value           Location    Line Numbers");
        println!("--------------    --------------  --------------  --------    -----------");
        for bucket_list in &self.table {
            if let Some(bucket_vec) = bucket_list {
                for bucket in bucket_vec {
                    print!(
                        "{:<15} {:<15} {:<15} {:<10} ",
                        bucket.name, bucket._type, bucket.value, bucket.memloc
                    );
                    for line in &bucket.lines {
                        print!("{} ", line.lineno);
                    }
                    println!();
                }
            }
        }
    }
}
