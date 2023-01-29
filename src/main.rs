mod hotel;
mod menu;

use hotel::Hotel;
use menu::Menu;

fn main() {
    let mut menu = Menu::Main;
    let mut _hotel = Hotel::new();

    'engine: loop {
        match menu {
            Menu::Main => {
                println!("This is main menu.");
                println!("1. Goes to Side1");
                println!("2. Goes to Side2");
                println!("3. Goes to Side3");
                println!("4. Goes to End");
                println!("L always goes to End");
                menu.switch4(Menu::Side1, Menu::Side2, Menu::Side3, Menu::End);
            }
            Menu::Side1 => {
                println!("This is Side menu 1.");
                println!("1. Goes to Side2");
                println!("2. Goes to Main");
                println!("3. Goes to End");
                println!("L always goes to End");
                menu.switch3(Menu::Side2, Menu::Main, Menu::End);
            }
            Menu::Side2 => {
                println!("This is Side menu 2.");
                println!("1. Goes to Side3");
                println!("2. Goes to Main");
                println!("L always goes to End");
                menu.switch2(Menu::Side3, Menu::Main);
            }
            Menu::Side3 => {
                println!("This is Side menu 3.");
                println!("1. Goes to Main");
                println!("L always goes to End");
                menu.switch1(Menu::Main);
            }
            Menu::End => {
                // engine = false;
                println!("This is the end.");
                break 'engine;
            }
        }
    }
}