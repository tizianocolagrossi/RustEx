use std::io;
use std::io::*;
use std::fs::File;
use std::collections::HashSet;
use std::convert::TryFrom;

struct World{
    grid: HashSet<Resident>,
    size: i32,
}

#[derive(Hash, Eq, PartialEq)]
struct Resident(i32, i32);

impl World{
    fn print(&self){
        for riga in 0..self.size{
            for colonna in 0..self.size*2{
                print!("{}",
                    if self.grid.contains(&Resident(riga, colonna)){"%"}
                    else {"."}
                );
            }
            println!();
        }
        println!();
    }

    fn ngbhood(&self, residente: &Resident)->i32{
        let mut vicini = 0;
        let riga = residente.0;
        let colonna = residente.1;

        if self.grid.contains(&Resident(riga-1, colonna-1)){vicini+=1;}
        if self.grid.contains(&Resident(riga-1, colonna)){vicini+=1;}
        if self.grid.contains(&Resident(riga-1, colonna+1)){vicini+=1;}

        if self.grid.contains(&Resident(riga, colonna-1)){vicini+=1;}
        if self.grid.contains(&Resident(riga, colonna+1)){vicini+=1;}
        
        if self.grid.contains(&Resident(riga+1, colonna-1)){vicini+=1;}
        if self.grid.contains(&Resident(riga+1, colonna)){vicini+=1;}
        if self.grid.contains(&Resident(riga+1, colonna+1)){vicini+=1;}
        vicini
    }

    fn new(size: i32)->World{
        //generare un valore random
        let mut f = File::open("/dev/urandom").unwrap();
        let mut buf = [0u8; 32 ];
        let mut counter: usize = 0;
        let mut grid = HashSet::new();
        f.read_exact(&mut buf).unwrap(); //TODO gestione errore
        

        for colonna in 0..size*2{
            for riga in 0..size{
                if counter == 31 {
                    f.read_exact(&mut buf).unwrap(); //TODO gestione errore
                    counter = 0;
                }
                if buf[counter]%2 == 0 {
                    grid.insert(Resident(riga, colonna));
                }
                counter+=1;
            }
        }
        World{grid, size}
    }

    fn next_day(&mut self){
        let mut grid = HashSet::new();
        for riga in 0..self.size{
            for colonna in 0..self.size*2{
                let vicini = self.ngbhood(&Resident(riga, colonna));
                //controllo se viva o morta
                if self.grid.contains(&Resident(riga,colonna)){
                    //VIVA
                    if vicini == 2 || vicini == 3{
                        grid.insert(Resident(riga, colonna));
                    }
                }
                else {
                    //MORTA
                    if vicini == 3{
                        grid.insert(Resident(riga, colonna));
                    }
                }
            }
        }
        self.grid = grid;
    }
    fn next_iteration(&mut self){
        self.next_day();
        self.print();
    }
}

fn main() {
    let dimensione_mondo: i32 = loop{
        let mut input = String::new();
        print!("Dimensione del mondo ? > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("ERRORE");
        let dimensione_mondo = input.trim().parse::<u32>();
        let dimensione_mondo = match dimensione_mondo{
            Ok(dim) => dim,
            Err(_error)=>{
                println!("Inserire un numero decimale maggiore di 0! ");
                0
            },
        };
        if dimensione_mondo != 0{
            let dimensione_mondo = i32::try_from(dimensione_mondo).unwrap();
            break dimensione_mondo;
        }
    };

    let mut w = World::new(dimensione_mondo);
    w.print();

    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();
    w.next_iteration();

    

}
