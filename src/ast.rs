use std::fmt;

// PartialEq: Genera una implementación de comparación ==/!= entre dos Expr, 
// comparando recursivamente variante por variante y campo por campo. 
// Sin esto, expr1 == expr2 no compilaría.
#[derive(Debug, PartialEq)]
// Debug: Le pide al compilador que genere una implementación de `fmt::Debug` para `Expr`, 
// así puedes hacer println!("{:?}", expr) sin escribir tú el código de formateo. 
// Es un `impl` como el i`mpl fmt::Display for Expr` que aparece abajo, 
// pero autogenerado en vez de escrito a mano — por eso Expr puede imprimirse 
// tanto con {} (nuestro Display manual: Sub(Sub(3, 2), 1)) 
// como con {:?} (el Debug derivado, salida más literal tipo
//            Op(Op(Number(3), Sub, Number(1)), Sub, Number(1)) 
// mostrando la estructura real del enum).

pub enum Expr {
    Number(i32), /// hojas
    Op(Box<Expr>, OpCode, Box<Expr>), // nodos internos:
    // Box<Expr> es un puntero a un Expr, para evitar que el enum sea recursivo y tenga tamaño infinito   
    // Un operador y dos hijos (subárboles).
    // Por ejemplo, 3-2-1 se parsea (en buildingast.lalrpop) como:
    //              Op(Op(Number(3), Sub, Number(2)), Sub, Number(1))
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
}

// impl se usa para implementar funcionalidad sobre un tipo (una struct o enum) que ya se declaró antes 
// por separado. En Rust, a diferencia de otros lenguajes OO, los métodos no van dentro de la definición 
// del tipo: primero declaras los datos (enum Expr { ... }), y luego, en un bloque impl aparte, 
// defines qué puede hacer ese tipo. La sintáxis es impl Trait for Type { ... }. Significa:
//  "el tipo Expr implementa el trait std::fmt::Display". Aquí el trait es `fmt::Display`, 
// que es un trait de la librería estándar que permite convertir un valor a una cadena de texto.
// Un trait es como una interfaz. fmt::Display es un trait que requiere implementar el método 
// fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::Op(lhs, op, rhs) => write!(f, "{op}({lhs}, {rhs})"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            OpCode::Mul => "Mul",
            OpCode::Div => "Div",
            OpCode::Add => "Add",
            OpCode::Sub => "Sub",
        };
        write!(f, "{symbol}")
    }
}