
// Constantes têm sempre o mesmo valor, atribuído em compilação
// Sempre devem ser "anotadas" com o tipo
const MAX_POINTS:u32 = 1000;


fn main() {
    let mut x = 5;
    println!("The value of x is:{}",x);

    x = 6;
    println!("The value of x is:{}",x);

    /*  Mutabilidade é uma escolha: depende do que faz sentido
        Devemos considerar o tamanho das alocações, o tempo de processamento
        para o eu translado em memória, seja ela RAM ou de disco, etc;
    */
    other();
}

// Ilustrando o conceito de Shadowing
fn other() {

    let x = 1;

    // Precisamos 're'-declarar uma variável para ela ocultar a anterior
    let x = x+1;

    // Os tipos delas podem ser diferentes, o que facilita muito quando queremos converter um valor
    // sem ficar renomeando a torto e a direito
    let x = "muito louco";

    println!("Olha que loucura: {}", x);

    test_for();
}

fn listas_tuplas(){

    // O tamanho de tuplas é fixo desde o início, e as type annotations são opcionais
    let tup:(i32,f64,u8) = (500,6.4,1);

}

fn test_for (){
    for el in (1..6).rev() {
        println!("CRAZY COUNTDOWN {}",el)
    }
}