extern crate termion;

use termion::screen::AlternateScreen;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::{thread, time};

fn loading(){
    let mill = 100;
    for i in 0..10{
        
        print!("{}\\",termion::cursor::Goto(1,1));
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(mill));
    
        print!("{}|",termion::cursor::Goto(1,1));
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(mill));
    
        print!("{}/",termion::cursor::Goto(1,1));
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(mill));
    
        print!("{}-", termion::cursor::Goto(1,1));
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(mill));
    
    }

}

fn main() {

    print!("{}", termion::clear::All);
    loading();

}

