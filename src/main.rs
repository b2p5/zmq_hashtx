//
// Ejemplo de suscripción al nodo de Bitcoin usando ZeroMQ - hashtx
//  

use std::thread;
use std::time::Duration;
use zmq::Context;
use rustc_hex::ToHex;
extern crate hex;



fn mempool_subscriber() -> Result<(), Box<dyn std::error::Error>> {
    
    // Crear un contexto zmq
    let context = Context::new();

    // Crear un socket de tipo SUB
    let subscriber = match context.socket(zmq::SUB) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error al crear el socket: {:?}", e);
            return Err(e.into());
        }
    };

    // Conectar al nodo Bitcoin  (puerto 28332)
    if let Err(e) = subscriber.connect("tcp://127.0.0.1:28332") {
        eprintln!("Error al conectar al nodo Bitcoin: {:?}", e);
        return Err(e.into());
    }

    // Suscribirse a eventos relacionados con la mempool
    if let Err(e) = subscriber.set_subscribe(b"hashtx") {
        eprintln!("Error al suscribirse: {:?}", e);
        return Err(e.into());
    }

    // Recibir y procesar mensajes
    loop {
        
        // El primer mensaje contiene el canal de suscripción, por lo que lo descartamos
        let _ = match subscriber.recv_bytes(0) {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Error al recibir el canal: {:?}", e);
                continue;
            }
        };
        

        match subscriber.recv_bytes(0) {
            Ok(tx_hex) => {
                // Convert bytes to a hexadecimal string
                let hex_string = tx_hex.to_hex::<String>();

               
                process_hex_string(&hex_string);

                Some(hex_string)
            },
            Err(e) => {
                println!("Failed to receive bytes: {}", e);
                None
            },
        };
        
        
        // Pausar para evitar un consumo excesivo de recursos
        thread::sleep(Duration::from_secs(0));
    
    }   // Fin del loop

}


fn process_hex_string(hex_string: &str) {
    // Longitud de la cadena hexadecimal
   let len = hex_string.len();

   match len {
       64 => {
           println!("Hash: {:?}", hex_string);
       },
       12 => {
           println!("Topic: {:?}", "hashtx");
       },
       8 => {
           let bytes = hex::decode(&hex_string).expect("Error al decodificar la cadena hexadecimal");
           let text_string = String::from_utf8_lossy(&bytes);
           println!("Sequence number: {} -> {:?}", &hex_string , text_string);
           println!("\n");
       },
       _ => {
           println!("Unrecognized hex string length: {}", len);
       }
   }
}

fn main() {
    // Iniciar el suscriptor en el hilo principal
    match mempool_subscriber() {
        Ok(_) => {
            // La función se ejecutó con éxito
            println!("Terminado"); 
        },
        Err(e) => {
            // Hubo un error al ejecutar la función, puedes manejarlo aquí
            eprintln!("Error al ejecutar mempool_subscriber: {:?}", e);
        },
    }
}
