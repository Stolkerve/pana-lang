# Lenguaje de programacion Pana
Este lenguaje de programacion esta inspirado en el libro *Writing a interpreter in Go*. El objetivo de este mismo es simplemente recreativo, educativo y experimental. No es un proyecto serio.

## Comandos
#### Ejecutar un archivo .pana (futuro)
```pana archivo.pana```

#### Arte
```pana pana```

#### Modo interactivo
En el modo interactivo tienes los comandos de: `salir`, `limpiar`, `pana`.

```pana```

## Sintaxis
#### Variables
```
var a = 20
var b = verdad == falso
```
Todavia no soporta la sintaxis de volver a asignar un valor
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
    x + y + extra
}

fn resta(x, y) {
    x - y
}

var total = sumar(1, 2) * resta(4, 5)
```