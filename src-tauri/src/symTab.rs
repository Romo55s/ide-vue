use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ptr;
use std::rc::Rc;

const SIZE: usize = 211;
const SHIFT: u32 = 4;

/* La función hash */
fn hash(key: &str) -> usize {
    let mut temp = 0;
    for c in key.chars() {
        temp = ((temp << SHIFT) + c as usize) % SIZE;
    }
    temp
}

/* La lista de números de línea del código fuente
 * en la que se referencia una variable
 */
#[derive(Clone)]
struct LineList {
    lineno: i32,
    next: Option<Rc<RefCell<LineList>>>,
}

impl LineList {
    fn new(lineno: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(LineList { lineno, next: None }))
    }
}

/* El registro en la lista de buckets para
 * cada variable, incluyendo nombre,
 * la ubicación de memoria asignada, y
 * la lista de números de línea en la que
 * aparece en el código fuente
 */
#[derive(Clone)]
struct BucketList {
    name: String,
    lines: Rc<RefCell<LineList>>,
    memloc: i32,
    next: Option<Rc<RefCell<BucketList>>>,
}

impl BucketList {
    fn new(name: String, lineno: i32, memloc: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(BucketList {
            name,
            lines: LineList::new(lineno),
            memloc,
            next: None,
        }))
    }
}

/* La tabla hash */
struct SymbolTable {
    table: Vec<Option<Rc<RefCell<BucketList>>>>,
}

impl SymbolTable {
    pub fn st_insert(&mut self, name: &str, lineno: i32, loc: i32) {
        let h = hash(name);
        let mut l = self.table[h].clone(); // Cloning the Option<Rc<RefCell<BucketList>>>

        // Search the bucket list for the name
        while let Some(ref bucket) = l {
            if bucket.borrow().name == name {
                // The name is already in the table, we just add a new line number
                let mut lines = bucket.borrow().lines.clone(); // Clone the Rc<RefCell<LineList>>

                // Traverse the line list to append the new line number
                while lines.borrow().next.is_some() {
                    let next = lines.borrow().next.clone().unwrap();
                    lines = next;
                }
                lines.borrow_mut().next = Some(LineList::new(lineno));
                return;
            }
            l = bucket.borrow().next.clone();
        }

        // If not found, insert new bucket
        let new_bucket = BucketList::new(name.to_string(), lineno, loc);
        new_bucket.borrow_mut().next = self.table[h].clone(); // Insert at the head of the list
        self.table[h] = Some(new_bucket);
    }
    /* Función st_lookup devuelve la ubicación de memoria
     * de una variable o -1 si no se encuentra
     */
    pub fn st_lookup(&self, name: &str) -> i32 {
        let h = hash(name);
        let mut l = self.table[h].clone();

        while let Some(ref bucket) = l {
            if bucket.borrow().name == name {
                return bucket.borrow().memloc;
            }
            l = bucket.borrow().next.clone();
        }

        -1
    }

    /* Procedimiento printSymTab imprime una lista formateada
     * del contenido de la tabla de símbolos
     */
    pub fn print_symtab(&self) {
        println!(
            "{:14}  {:8}  {}",
            "Variable Name", "Location", "Line Numbers"
        );
        println!(
            "{:14}  {:8}  {}",
            "-------------", "--------", "------------"
        );

        for i in 0..SIZE {
            if let Some(ref bucket) = self.table[i] {
                let mut b = Some(bucket.clone());
                while let Some(ref current) = b {
                    let mut lines = current.borrow().lines.clone();
                    print!(
                        "{:14}  {:8}  ",
                        current.borrow().name,
                        current.borrow().memloc
                    );
                    while let Some(ref line) = lines {
                        print!("{:4} ", line.borrow().lineno);
                        lines = line.borrow().next.clone();
                    }
                    println!();
                    b = current.borrow().next.clone();
                }
            }
        }
    }
}
