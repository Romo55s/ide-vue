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
    fn new() -> Self {
        SymbolTable {
            table: vec![None; SIZE],
        }
    }

    /* Procedimiento st_insert inserta números de línea y ubicaciones
     * de memoria en la tabla de símbolos.
     * loc = ubicación de memoria que se inserta solo la
     * primera vez, de lo contrario se ignora
     */
    pub fn st_insert(&mut self, name: &str, lineno: i32, loc: i32) {
        let h = hash(name);
        let mut l = self.table[h].clone();

        while let Some(ref bucket) = l {
            if bucket.borrow().name == name {
                break;
            }
            l = bucket.borrow().next.clone();
        }

        if l.is_none() {
            let new_bucket = BucketList::new(name.to_string(), lineno, loc);
            new_bucket.borrow_mut().next = self.table[h].clone();
            self.table[h] = Some(new_bucket);
        } else {
            let bucket = l.unwrap();
            let mut lines = bucket.borrow().lines.clone();
            while lines.borrow().next.is_some() {
                lines = lines.borrow().next.clone().unwrap();
            }
            lines.borrow_mut().next = Some(LineList::new(lineno));
        }
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
