{{ project-name }}
{{ '=================================================' | slice: 0 , project-name.size }}

Proyecto en Rust para placa _{{ board }}_.

# Template para placa STM32F103 blue pill

Usando el crate **stm32f1xx-hal** 
Referencia: [https://github.com/stm32-rs/stm32f1xx-hal/](https://github.com/stm32-rs/stm32f1xx-hal/tree/master)

## Instructiones para construir el proyecto
1. Verifique que tiene instaladas las herramientas requeridas para esta placa:

- Rust con su correspondiente toolchain
- [itmdump](https://crates.io/crates/itm)
- openocd
- arm-none-eadbi-gdb
- [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)
- minicom

    1. Instalación de  itmdump
    ```bash
    cargo install itm
    ```

    2. Instalación de cargo-binutils 
    ```bash
    rustup component add llvm-tools-preview
    cargo install cargo-binutils
    ```

    3. Instalación de paquetes requeridos (Ubuntu 18.04 o mayor)
    ```bash
    sudo apt-get install gdb-multiarch minicom openocd
    ```

    4. Instalación de paquetes adicionales
    ```bash
    sudo apt-get install bluez rfkill
    ```

    Referencia completa en [https://docs.rust-embedded.org/](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html)

2. Verificaciones

    1. Verificar que se detecta el adaptador ST-LINK
    ```bash
    lsusb | grep -i stm
    ```
    Debe aparecer el adaptador ST-LINK como un device USB (Bus 999 Device 888)

    2. Verificar permisos
    ```bash
    ls -la /dev/bus/usb/003/004
    ```
    Deberían ser crw-rw-rw- 

    Guía completa en [https://docs.rust-embedded.org/](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/verify.html)

    3. Verificar que la placa esté en modo programación
    **BOOT0 -> HIGH**
    Cambiar de posición los jumpers de configuración

3. Establecer la coneción con OpenOCD
    ```bash
    openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
    ```

4. Decirle a gdb que es seguro cargar .gdbinit desde el directorio de trabajo.
    ```bash
    echo "set auto-load safe-path $(pwd)" >> ~/.gdbinit
    ```

5. Construir el binario (para STM32F103)
    ```bash
    rustup target add thumbv7m-none-eabi
    cargo build --target thumbv7m-none-eabi
    ```

6. Flashear la placa
    ```bash
    gdb-multiarch -q -ex "target remote :3333" target/thumbv7m-none-eabi/debug/<nombre de mi proyecto>
    ```

    Si el proceso fue exitoso, tendremos um propmt (gdb) esperando 

    ```bash
    (gdb) load
    ```  

    Referencia completa y troubleshooting [https://docs.rust-embedded.org/](https://docs.rust-embedded.org/discovery/f3discovery/05-led-roulette/flash-it.html)

7. Arrancar el programa
**BOOT0 -> LOW**
Cambiar de posición los jumpers de configuración


## Licencia
Con licencia en virtud de

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

a su elección.

## Contribución
A menos que indique explícitamente lo contrario, cualquier contribución que usted envíe intencionadamente para su inclusión en la obra, tal y como se define en la licencia Apache-2.0, tendrá doble licencia, tal y como se ha indicado anteriormente, sin términos ni condiciones adicionales.