# Lenguaje de programacion Pana
Este lenguaje de programacion esta inspirado en el libro *Writing a interpreter in Go*. El objetivo de este mismo es simplemente recreativo, educativo y experimental. No es un proyecto serio. Sin embargo, apunta a ser un lenguaje capaz de ser usado para proyectos educativos y ser capaz de producir un producto. 

# Compilacion
### Prerequisitos
1. Instalar rust.

    https://www.rust-lang.org/tools/instal

    ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

2. Instalar git.
3. Tener internet.

### Correr el programa
```cargo run <comandos opcionales>```

### compilar
```cargo build --release```

# Comandos
### Ejecutar un archivo .pana o cualquier archivo, no descriminamos la extension.
```pana archivo.pana```

### Arte
```pana pana```

### Modo interactivo
En el modo interactivo tienes los comandos de: `salir`, `limpiar` y `pana`.

```pana```

# Sintaxis
### Variables
```
var a = 20;
var b = verdad == falso;
var c = [0, 1, "hola", {"juan": 20}];
a = b;
```

### Condicionales
```
var a = 20;
si a == 20 {
    var b = a * 2;
}
sino {
    var c = verdad != falso;
}
```
### Funciones 
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
### Comentarios
```
|Hola esto es un comentario.
|Lo mismo pero en otra linea.
var a = ""; |Ahora digo que hace esta variable, nada.
```
# Tipo de datos
```
| Numerico
var a = 12;

| Logico
var b = verdad;

| Cadena
var c = "Hola mundo";

| Nulo
var d = nulo;

| Lista
var e = [1, 2 "hola", [fn(x) { x * 2}, d]];
e[0] = 10

| Diccionario
var g = {verdad: [{1: 10}, "xd", falso], "hola": "mundo"}
g["hola"] = nulo;
```
# Referencia y copias
Los tipos de datos: `Numericos`, `logicos`, `nulo` y `cadena`. No pueden ser referenciados, se copian con cada asignacion.
```
var a = 10;
var b = a; | b copia el valor de a
```
Los tipos de datos: `Lista` y `Diccionario` pueden son referenciados.
```
var a = [0, 1, 2]
var b = a; | b referencia a
b[1] = "hola mundo"
imprime(a[1]) | -> "hola mundo"

```

# Operaciones
```
falso == 0                  |-> verdad
falso != verdad             |-> verdad
"hola" == "chao"            |-> falso
4 < 0                       |-> falso
9 > 8                       |-> verdad
1 >= 1                      |-> verdad 
0 <= 1                      |-> verdad 
nulo != 2                   |-> verdad
[1, 2, [3, 4]] != [1, 2]    |-> verdad
[1, 2, 3] > [1, 2]          |-> verdad
1 + 2                       |-> 3
2 - 4                       |-> -2
4 * 4                       |-> 16
2 / 2                       |-> 0
verdad - 1                  |-> 0
"hola" + " " + "mundo"      |-> "hola mundo"
[1, 2] + [3, 4]             |-> [1, 2, 3, 4]
2 == [0, 2][1]              |-> 
"hola" * 2                  |-> "holahola"
[1, 2, [3, 4]] * 2          |-> [1, 2, [3, 4], 1, 2, [3, 4]]
```

# Funciones internas
#### Longitud
```
longitud("hola")                                    | -> 4
longitud([1, 2, 3])                                 | -> 3
longitud({"nombre": "Sebas", "apellido": "Lopez"})  | -> 2
```
### Tipo de dato
```
tipo("hola") | -> "cadena"
```
### Imprimir en consola
```
imprimir("hola ", "mundo") | -> "hola mundo"
```
### Leer consola
```
leer("Ingrese su nombre: ") | -> "Sebastian"
```
### Cadena
```
cadena(10) | -> "10"
```

# Futuro
### Version 0.1
- ✅ Varibles.
- ✅️ tipo de datos: entero, nulo, vacio, logico, lista, diccionario y cadena (string).
- ✅ Condicionales.
- ✅ Funciones y funciones anonimas.
- ✅ Funciones internas basicas (imprimir, tipo, longitud, y leer)
- ✅ Operaciones
- ️✅️ Comentarios
### Version 0.1.1
- ✅ ️️Funcion interna nueva: cadena.
- ✅ ️️Asignacion a objetos dentro de una lista y diccionario.
- ✅ ️️Referencias a listas y diccionarios.

### Version 0.2
- ⬜️ Mejora en los mensajes de error con columna y linea
### Version 0.2.1
- ⬜️ Cambiar el tipo de dato entero a numerico
### Version 0.2.2
- ⬜️ Leer los strings dentro del parser y no el lexer

### Version 0.3
- ⬜ bucle for range: para i en rango(10).
- ⬜ bucle while: mientras i < 10.
- ⬜ Keyword break: romper.

### Version 0.4
- ⬜ Operador de acceso de miembros: **.**
- ⬜ Acceso a funciones miembro de los tipos de datos:
    - ⬜ lista: agregar(), eliminar(), buscar(), concatenar(), indice(), insertar(), separar(), invertir(), ordenar(), limpiar(), vacio().
    - ⬜ diccionario: eliminar(), llaves(), valores(), limpiar().
    - ⬜ cadena: caracteres(), concatenar(), buscar(), eliminar(), es_alfa, es_numerico, inicia_con(), insertat(), invertir(), mayusculas(), minusculas(), reemplazar(), recortar(), separar(), subcadena(), vacio().

### Version 0.5
- ⬜ Soporte para la sintaxis de modulos.
- ⬜ Modulos internos:
    - ⬜ archivo: todo lo relacionado a manejo de ficheros.
    - ⬜ sis: todo lo relacionado a syscalls.
    - ⬜ mate: todo lo relacionado a matematicas.
- ⬜ Importar codigo **Pana** externo con modulos