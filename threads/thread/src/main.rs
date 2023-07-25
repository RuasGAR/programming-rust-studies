use std::thread;
use std::time::Duration;

/* fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // o retorno do spawn é um joinHandle. Podemos utilizar isso pra forçar a thread principal
    // a esperar o fim das restantes.
}

fn other_main(){
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi, im number {} from the spawned thread",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Botar esse join aqui fará com que toda a main thread espere as que foram criadas antes
    // e depois ela volta à execução. Dessa vez, não haverá alterância, porque a main thread
    // estará bloqueada, esperando as outras terminarem.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi, i'm {} main thread",i);
    }
} */


// MOTIVAÇÃO: queremos usar dados de um thread em outra, e é necessário transferir ownership
// O livro fala de utilzar um conceito de move closures, seja lá o que for.

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}