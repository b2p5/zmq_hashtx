
# Ejemplo de utilización la biblioteca ZeroMQ con hashtx 

1. **Crear un contexto zmq**: El programa comienza creando un nuevo contexto ZeroMQ. Este contexto se utiliza para crear sockets ZeroMQ.

2. **Crear un socket de tipo SUB**: El programa crea un socket SUB utilizando el contexto ZeroMQ. Si hay un error al crear el socket, el programa imprime el error y lo devuelve.

3. **Conectar al nodo Bitcoin**: El socket SUB se conecta a un nodo de Bitcoin que se ejecuta en localhost en el puerto 28332. Si hay un error al conectar, el programa imprime el error y lo devuelve. 
En el archivo ***bitcoin.conf*** se ha de incluir el comando: 
zmqpubrawtx=tcp://127.0.0.1:28332

4. **Suscribirse a eventos relacionados con la mempool**: El socket SUB se suscribe a los eventos "hashtx", que representan las notificionrs sobre todas las transacciones.

5. **Recibir y procesar mensajes**: El programa entra en un bucle donde recibe y procesa continuamente mensajes. El primer mensaje que recibe es el canal de suscripción, que descarta. El segundo mensaje que recibe es la transacción en formato hexadecimal, la cual se pasa a string. Si hay un error al recibir cualquiera de los mensajes, el programa imprime el error y continúa con la siguiente iteración del bucle o devuelve el error.

6. **Pausa**: El programa hace una pausa de 3 segundos para evitar el consumo excesivo de recursos.



*Resultado del script:*

Hash: "2b7c9160f0ae3c5af8b76db60e4ec1e50313a689776a392d60a3d1a7f0817bb8"

Topic: "hashtx"

Sequence number: 57160d00 -> "W\u{16}\r\0"


