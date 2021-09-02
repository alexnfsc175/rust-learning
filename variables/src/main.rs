fn main() {
    const PONTOS_MAXIMOS: u32 = 100_000;
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {}", x);
    println!("Pontos maximos : {}", PONTOS_MAXIMOS);
    let tup: (i32, f64, char, u8) = (500, 6.4, 'ℤ', 1);
    println!("Primeiro valor da tupla : {}", tup.0);
    println!("Segundo valor da tupla : {}", tup.1);
    println!("Terceiro valor da tupla : {}", tup.2);

    let (a, b, c, _) = tup; // desestruturação
    println!("Primeiro valor extraido da tupla : {}", a);
    println!("Segundo valor extraido da tupla : {}", b);
    println!("Terceiro valor extraido da tupla : {}", c);
    let _a = [1, 2, 3, 4, 5];
    let _meses = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];
    outra_funcao(5);
}

fn outra_funcao(x: i32) {
    println!("O valor de x é: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {}", y);

    let x5 = cinco();

    println!("O valor de x é: {}", x5);
}

// não é necessario a palavra return
fn cinco() -> i32 {
    5
}
