use socketcan::{CanSocket, CanFrame};
use tauri::Manager;
use std::thread;
use std::time::Duration;

#[tauri::command]
fn start_can_listener(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let socket = CanSocket::open("can0").expect("Impossible d'ouvrir le bus CAN");

        loop {
            match socket.read_frame() {
                Ok(frame) => {
                    let id = frame.id();
                    let data = frame.data().to_vec();

                    // Envoyer les données au frontend via un event Tauri
                    app_handle.emit_all("can_message", (id, data)).unwrap();
                }
                Err(e) => eprintln!("Erreur de lecture CAN : {}", e),
            }

            // Petite pause pour éviter une boucle infinie trop rapide
            thread::sleep(Duration::from_millis(10));
        }
    });
}
