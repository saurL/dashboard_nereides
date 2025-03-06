use log::error;
use socketcan::{CanSocket, Socket};
use tauri::{AppHandle, Manager};
// small change to test ff  gg
pub struct App{
    can_socket: Option<CanSocket>,
    app_handle: AppHandle
}

impl App{

    pub fn new(app_handle:AppHandle) -> App{
        let mut socket = None;
        if let Ok(Can_socket) = CanSocket::open("can0"){
            socket = Some(Can_socket);
        }
        else {
            error!("Impossible d'ouvrir le bus CAN");
        }
        
        App{can_socket: socket,app_handle}
        
    }
}