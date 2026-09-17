use kvaser_can_wrapper::{initialize, open, send, recv, close};
use std::{thread, time::Duration};

fn main() {
    initialize();

    let gonderen = open(0);
    let alan = open(1);

    println!("Gonderen handle: {}", gonderen);
    println!("Alan handle: {}", alan);

    if gonderen < 0 || alan < 0 {
        println!("HATA: kanallar acilamadi - virtualcan yuklu mu kontrol et.");
        return;
    }

    thread::sleep(Duration::from_millis(500));

    let mesaj = [0x11, 0x22, 0x33, 0x44];
    let basarili = send(gonderen, 100, &mesaj);
    println!("Gonderme sonucu: {}", basarili);

    let mut id: i32 = 0;
    let mut buffer = [0u8; 8];
    let mut len: usize = 0;

    if recv(alan, &mut id, &mut buffer, &mut len, 2000) {
        println!("Mesaj alindi! id={}, veri={:?}", id, &buffer[..len]);
    } else {
        println!("HATA: mesaj alinamadi (zaman asimi).");
    }

    close(gonderen);
    close(alan);
}
