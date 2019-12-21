#[allow(dead_code)]

use std::io;
use std::io::{stdin};
use termion::input::TermRead;
use termion::event::*;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Alignment, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget, SelectableList, Text, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    
    //Lista degli elementi nella lista
    let list = vec!["test 1 ", "test 2 ", "test 3 ", "test 4 ", "test 5 ", "test 6 ", "test 7 ",
                    "test 8 ", "test 9 ", "test 10", "test 11", "test 12", "test 13", "Test 14",
                    "test 15", "test 16", "test 17", "test 18", "test 19", "test 20", "Test 21",
                    "test 22", "test 23", "test 24", "test 25", "test 26", "test 27", "Test 28",
                   ];
    let mut selected = Some(list.len());
    
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let mut events = stdin().events();
    let mut scroll: u16 = 0;

    loop{
        terminal.draw(|mut f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();
            Block::default().borders(Borders::NONE).render(&mut f, size);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
                ].as_ref())
                .split(f.size());
            let blklist = Block::default()
                            .borders(Borders::ALL)
                            .title("List")
                            .title_style(Style::default().fg(Color::Yellow));
            SelectableList::default()
                .block(blklist)
                .items(&list)
                .select(selected)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::White).fg(Color::Red).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
            
            let s = "veeeeeeery loooooooooooooooooooooooooooooong liiiiiiiiiiiiiiiiiiiiiiiiiiiiiiine. ";
            let mut long_line = s.repeat(usize::from(size.width)/s.len() + 4);  
            long_line.push('\n');
            let text = [
                Text::raw("This is a line \n"),
                Text::styled("This is a line styled \n", Style::default().fg(Color::Yellow)),
                Text::styled("This is a longher line \n", Style::default()
                                                            .modifier(Modifier::CROSSED_OUT)),
                Text::styled(&long_line, Style::default().bg(Color::White).fg(Color::Yellow)
                                                .modifier(Modifier::BOLD))
            ];

            let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Ratio(1,3),
                                Constraint::Ratio(1,3),
                                Constraint::Ratio(1,3),
                            ].as_ref())
                            .split(chunks[1]);
            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));
            Paragraph::new(text.iter())
                .block(block.clone().title("P1 no wrap"))
                .alignment(Alignment::Left)
                .render(&mut f, chunks[0]);
            Paragraph::new(text.iter())
                .block(block.clone().title("P2 wrap scroll"))
                .alignment(Alignment::Center)
                .wrap(true)
                .scroll(scroll)
                .render(&mut f, chunks[1]);
            Paragraph::new(text.iter())
                .block(block.clone().title("P3 wrap no scroll"))
                .alignment(Alignment::Right)
                .wrap(true)
                .render(&mut f, chunks[2]);
            
            //block list vvvvvv
            //Block::default()
            //    .borders(Borders::ALL)
            //    .title("List")
            //    .title_style(Style::default().fg(Color::Yellow))
            //    .render(&mut f, chunks[0]);
            //Block::default()
            //    .borders(Borders::ALL)
            //    .title("wnd 1")
            //    .title_style(Style::default()
            //                    .bg(Color::Red)
            //                    .fg(Color::White)
            //                    .modifier(Modifier::BOLD)
            //                )
            //    .render(&mut f, chunks[1]);

            
        })?;

        scroll +=1;
        scroll %=10;
        
        //####################################################################
        //gestore degli eventi
        //####################################################################
        match events.next().unwrap().unwrap() {
            Event::Key(keyevent) =>{
                match keyevent{
                    Key::Char('q')=>break,
                    Key::Right =>{selected = Some(list.len())}
                    Key::Left =>{selected = Some(0);}
                    Key::Down =>{
                        if selected == Some(list.len()){
                            selected = Some(0);
                        }else{
                            let mut i: usize = selected.unwrap();
                            i=(i+1)%(list.len());
                            selected = Some(i);
                        }
                    }
                    Key::Up =>{
                        if selected == Some(list.len()){
                            selected = Some(9);
                        }else{
                            let mut i: usize = selected.unwrap();
                            if i == 0 {i = i + list.len();}
                            i = i-1;
                            selected = Some(i);
                        }
                    }
                    _ =>{}
                }
            }
            Event::Mouse(mousevent) =>{
                match mousevent{
                    MouseEvent::Press(_,a,b) |
                    MouseEvent::Release(a,b) |
                    MouseEvent::Hold(a,b) =>{}
                }
            }
            _ => {}
        }
    }    
    Ok(())
}
