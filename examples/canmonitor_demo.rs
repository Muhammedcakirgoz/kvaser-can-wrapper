use kvaser_can_wrapper::{initialize, open, send, close};
use std::{thread, time::Duration};

fn main() {
    initialize();

    let gonderen = open(0);
    println!("Gonderen handle: {}", gonderen);

    if gonderen < 0 {
        println!("HATA: kanal acilamadi.");
        return;
    }

    thread::sleep(Duration::from_millis(500));

    let mesaj = [0x11, 0x22, 0x33, 0x44];
    let basarili = send(gonderen, 100, &mesaj);
    println!("Gonderme sonucu: {}", basarili);

    close(gonderen);
}
