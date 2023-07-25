fn main() {
    // String literals são imutáveis
    // Vamos utilizar um outro tipo com o mesmo nome, que tem seu valor allocado na heap

    let s = String::from("hello");
    
    // essa cópia clona somente o tipo String (que contém um ponteiro pro valor)
    // um indicador de tamanho (atual) e de capacidade (que o OS fornece)
    let s2 = s; 

    // fazendo uma cópia (inclusive do conteúdo da heap)
    let s3 = s2.clone();
    println!("{}",s3);


    /* IMPORTANTE: alguns tipos de dados não exigem esse nível de cuidado 
       sobretudo por terem tamanhos fixos em tempo de compilação.
       Dessa forma, uma cópia de inteiros (por exemplo) é feita de forma "comum". 
    */
    let x = 3;
    let y = x;
    
    println!("{}:{}","A cópia funciona bem", y);

    teste_slices();
}

fn teste_slices(){

    let test = String::from("hello world");
    let world = &test[5..];

    println!("A palavra é: {}",world);


}
