> [!IMPORTANT]
> Este proyecto solo incluye dibujo básico de líneas por CPU
> No tomarse en serio este proyecto

# Experimental Graphics Engine (EGE)

Este es un proyecto de aprendizaje para explorar los fundamentos de un motor gráfico. EGE (Experimental Graphics Engine) se centra en la implementación de algoritmos y estructuras de datos fundamentales para el renderizado de gráficos.

## Características

El proyecto incluye implementaciones para:

- **Renderizado de Líneas:** Algoritmos como línea aliasing y antialiasing.
- **Clipping:** Técnicas para recortar geometría, incluyendo `parametric.rs`, `region_code.rs` y `selector.rs`.
- **Canvas:** Una representación de un lienzo de dibujo.
- **Tipos y Utilidades:** Definiciones de tipos (`types.rs`) y funciones de utilidad (`utils.rs`) para el motor.
- **Bucle Principal:** La estructura del bucle de ejecución principal del motor (`loop_.rs`, `main.rs`).
- **Gestión de Memoria:** Manejo de búferes de píxeles (`pixel_buffer.rs`).

## Librerías

- **winit:** Manejo de ventanas y eventos.
- **softbuffer:** Controlar el búfer de píxeles y renderizado de imágenes.
## Estructura del Proyecto

- `src/draw/clipping`: Implementaciones de algoritmos de clipping.
- `src/draw/line`: Implementaciones para el renderizado de líneas.
- `src/draw`: Módulos relacionados con el dibujo en general.
- `src/engine`: Componentes centrales del motor, como el bucle principal.
- `src/main.rs`: Punto de entrada de la aplicación.
- `src/types.rs`: Definiciones de tipos de datos.
- `src/utils.rs`: Utilidades varias.
- `Cargo.toml`: Archivo de manifiesto de Cargo.

## Cómo Contribuir

Siendo un proyecto de aprendizaje, las contribuciones son bienvenidas para mejorar las implementaciones existentes o añadir nuevas funcionalidades.
