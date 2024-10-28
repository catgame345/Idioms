use std::io;

fn main() {
    start();
}

fn start() {
    let mut lang = String::new();

    println!("1. English   2. Español   3. Français   4. Svenska   5. Pyccкй   6. Cymraeg  7. 日本語");

    io::stdin()
      .read_line(&mut lang)
      .expect("Error!");

    let langn: i64 = lang.trim().parse().expect("Not number!");

    language(langn);

    greet("", langn);

    let mut name = String::new();

    match langn {
        1 => println!("\nType your name user: "),
        2 => println!("\nIngrese su nombre usuario: "),
        3 => println!("\nEntrez votre nom d\'utilisateur: "),
        4 => println!("\nAnge ditt användarnamn: "),
        5 => println!("\nBвeдитe cвoe имя пoльзoвaтeля: "),
        6 => println!("\nRhowch eich enw defnyddiwr: "),
        7 => println!("\nユーザー名入力してください: "),
        _ => println!("\nType your name user: "),
    }
    
    io::stdin()
      .read_line(&mut name)
      .expect("Error!");

    if name == "" {
        greet("user", langn);
    } else {
        greet(name.trim(), langn);
    }
}

fn sum(name: &str, lang: i64) {
    let mut xn = String::new();
    let mut yn = String::new();

    match lang {
        1 => println!("\nType a number {}: ", name),
        2 => println!("\nIngrese un número {}: ", name),
        3 => println!("\nEntrez un numéro {}: ", name),
        4 => println!("\nAnge ett nummer {}: ", name),
        5 => println!("\nBвeдитe нoмep {}: ", name),
        6 => println!("\nRhowch rif {}: ", name),
        7 => println!("\n{}番号を入力してください: ", name),
        _ => println!("\nType a number {}: ", name),
    }

    io::stdin()
      .read_line(&mut xn)
      .expect("Error!");

    let x: i64 = xn.trim().parse().expect("Error!");

    match lang {
        1 => println!("\nType a second number {}: ", name),
        2 => println!("\nIngrese un segundo número {}: ", name),
        3 => println!("\nEntrez un deuxième numéro {}: ", name),
        4 => println!("\nAnge ett andra nummer {}: ", name),
        5 => println!("\nBвeдитe втopoй нoмep {}: ", name),
        6 => println!("\nRhowch ail rif {}: ", name),
        7 => println!("\n2番目の番号を入力してください{}: ", name),
        _ => println!("\nType a second number {}: ", name),
    }

    io::stdin()
      .read_line(&mut yn)
      .expect("Error!");

    let y: i64 = yn.trim().parse().expect("Error!");

    let sums: i64 = x + y;

    match lang {
        1 => println!("\nSum of {} and {} is {}.", x, y, sums),
        2 => println!("\nLa suma de {} y {} es {}.", x, y, sums),
        3 => println!("\nLa somme de {} et {} est égale à {}.", x, y, sums),
        4 => println!("\nSumman av {} och {} är lika med {}.", x, y, sums),
        5 => println!("\nCyммa {} и {} paвнa {}.", x, y, sums),
        6 => println!("\nMae swm {} ac {} yn hafal i {}.", x, y, sums),
        7 => println!("\n{}と{}の合計は{}に等しい。", x, y, sums),
        _ => println!("\nSum of {} and {} is {}.", x, y, sums),
    }
    println!("\n");
    start();
}

fn greet(name: &str, lang: i64) {
    if name == "" {
        match lang {
            1 => println!("\nHello, User!"),
            2 => println!("\nHola, Usuario!"),
            3 => println!("\nBonjour, Utilisateur!"),
            4 => println!("\nHej, Användare!"),
            5 => println!("\n3дpaвcтвyйтe, Пoльзoвaтeль!"),
            6 => println!("\nHelo, Defnyddiwr!"),
            7 => println!("\nユーザーさん、こんにちは!"),
            _ => println!("\nHello, User!"),
        }
    } else {
        match lang {
            1 => println!("\nHello, {}!", name),
            2 => println!("\nHola, {}!", name),
            3 => println!("\nBonjour, {}!", name),
            4 => println!("\nHej, {}!", name),
            5 => println!("\n3дpaвcтвyйтe, {}!", name),
            6 => println!("\nHelo, {}!", name),
            7 => println!("\nこんにちは, {}さん!", name),
            _ => println!("\nHello, {}!", name),
        }

        sum(name, lang);
    }
}

fn language(lang: i64) {
    match lang {
        1 => println!("\nLanguage is set to English."),
        2 => println!("\nEl idioma está configurado en Español."),
        3 => println!("\nLa langue est définie sur le Français."),
        4 => println!("\nSpråket är inställt på Svenska."),
        5 => println!("\nЯзык установлен Pyccкй."),
        6 => println!("\nGosodir iaith i'r Cymraeg."),
        7 => println!("\n言語は日本語に設定されています。"),
        _ => println!("Other language"),
    }
}
