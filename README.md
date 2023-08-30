# Lenguaje de programacion Pana
Este lenguaje de programacion esta inspirado en el libro *Writing a interpreter in Go*. El objetivo de este mismo es simplemente recreativo, educativo y experimental. No es un proyecto serio.

# Compilacion
## Prerequisitos
1. Instalar rust.\
https://www.rust-lang.org/tools/install\
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. Instalar git.
3. Tener internet.

## Correr el programa
```cargo run <comandos opcionales>```

## compilar
```cargo build```

# Comandos
#### Ejecutar un archivo .pana (futuro)
```pana archivo.pana```

#### Arte
```pana pana```

#### Modo interactivo
En el modo interactivo tienes los comandos de: `salir`, `limpiar`, `pana`.

```pana```

# Sintaxis
#### Variables
```
var a = 20;
var b = verdad == falso;
a = b;  a copia el valor de b, no hay referencias
```
#### Condicionales
```
var a = 20;
si a == 20 {
    var b = a * 2;
}
sino {
    var c = verdad != falso;
}
```
#### Funciones 
```
var sumar = fn(x, y) {
    var extra = 2;
    x + y + extra;
}

fn resta(x, y) {
    x - y;
}

var total = sumar(1, 2) * resta(4, 5);
```