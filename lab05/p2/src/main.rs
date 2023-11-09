struct Canvas {
    latime: usize,
    lungime: usize,
    puncte: [[char; 50]; 10],
}

fn dimensiune_desen(mut d: Canvas, x: usize, y: usize) -> Canvas {
    d.latime = x;
    d.lungime = y;
    d
}

fn pictare(d: &mut Canvas, x: usize, y: usize, caracter: char) -> bool {
    if x < d.latime && y < d.lungime {
        d.puncte[x][y] = caracter;
        true
    } else {
        false
    }
}

fn afisare_desen(d: &Canvas) {
    for i in 0..d.latime {
        for j in 0..d.lungime {
            print!("{}", d.puncte[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut desen = Canvas {
        latime: 0,
        lungime: 0,
        puncte: [[' '; 50]; 10],
    };

    desen = dimensiune_desen(desen, 5, 5);

    for i in 1..4 {
        pictare(&mut desen, i, 1, 'l');
        pictare(&mut desen, i, 3, 'l');
        pictare(&mut desen, 1, i, 'l');
        pictare(&mut desen, 3, i, 'l');
    }

    afisare_desen(&desen);
}
