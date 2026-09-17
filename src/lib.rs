// bindgen'in urettigi FFI kodu (canlib.h'den otomatik cevrilmis)
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::c_void;

/// Kutuphaneyi baslatir - PROGRAMIN EN BASINDA, bir kez cagrilmali.
pub fn initialize() {
    unsafe {
        canInitializeLibrary();
    }
}

/// Bir CAN kanalini acar, bus parametrelerini ayarlar ve aktif hale getirir.
/// channel: 0, 1, 2... (virtualcan'da genelde 0 ve 1 birbirine bagli)
/// Basarili olursa gecerli bir handle (>= 0) doner, hata olursa negatif deger doner.
pub fn open(channel: i32) -> i32 {
    unsafe {
        let handle = canOpenChannel(channel, canOPEN_ACCEPT_VIRTUAL as i32);

        if handle < 0 {
            return handle;
        }

        // Bus parametrelerini ayarla: 500 kbit/s, standart CAN zamanlama degerleri.
        canSetBusParams(handle, 500_000, 4, 3, 1, 1, 0);

        canBusOn(handle);

        handle
    }
}

/// Bir CAN mesaji gonderir.
/// handle: open() ile alinan deger
/// id: CAN mesaj ID'si
/// payload: gonderilecek veri (en fazla 8 byte, klasik CAN icin)
/// len: payload'un uzunlugu
pub fn send(handle: i32, id: i32, payload: &[u8]) -> bool {
    unsafe {
        let sonuc = canWrite(
            handle,
            id as std::os::raw::c_long,
            payload.as_ptr() as *mut c_void,
            payload.len() as u32,
            0,
        );

        sonuc == canStatus_canOK
    }
}
/// Bir CAN mesaji bekler/okur (timeout_ms kadar bekler, sonra vazgecer).
/// handle: open() ile alinan deger
/// id: okunan mesajin ID'sini buraya YAZAR (cikis parametresi)
/// payload: cagiranin ONCEDEN ayirdigi buffer - fonksiyon buraya YAZAR/// len: payload'un KAPASITESI olarak gelir, gercekte okunan veri
///      uzunlugu olarak GERI DONER (cikis parametresi)
/// Donus degeri: basarili mi (true) yoksa zaman asimi/hata mi (false)
pub fn recv(handle: i32, id: &mut i32, payload: &mut [u8], len: &mut usize, timeout_ms: i64) -> bool {
    unsafe {
        let mut c_id: std::os::raw::c_long = 0;
        let mut dlc: u32 = 0;
        let mut flag: u32 = 0;
        let mut time: u64 = 0;

        let sonuc = canReadWait(
            handle,
            &mut c_id,
            payload.as_mut_ptr() as *mut c_void,
            &mut dlc,
            &mut flag,
            &mut time,
            timeout_ms as u64,
        );

        if sonuc == canStatus_canOK {
            *id = c_id as i32;
            *len = dlc as usize;
            true
        } else {
            *len = 0;
            false
        }
    }
}
/// Bir kanali kapatir - bus'i pasif hale getirir, handle'i serbest birakir.
pub fn close(handle: i32) {
    unsafe {
        canBusOff(handle);
        canClose(handle);
    }
}
