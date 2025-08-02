# Raspberry Pi Zero LCD Simulation

Esta aplicación Rust permite simular una pantalla LCD ST7789 sin necesidad del hardware real de Raspberry Pi Zero.

## 🎯 Características

### Modo Simulación (Por defecto)
- ✅ Simula la pantalla LCD en la consola con caracteres ASCII
- ✅ Muestra "Hello World!" y datos de sensores simulados
- ✅ API web para monitoreo remoto
- ✅ Control de LED simulado
- ✅ No requiere hardware GPIO/SPI

### Modo Hardware
- 🔧 Controla pantalla LCD ST7789 real (240x240)
- 🔧 GPIO real para LED y control de pantalla
- 🔧 Comunicación SPI con la pantalla

## 🚀 Uso Rápido

### 1. Ejecutar en Modo Simulación
```bash
# Compilar y ejecutar (modo simulación por defecto)
cargo run

# O usar el script incluido
./run_simulation.sh
```

### 2. Ver la Simulación de Pantalla
La aplicación mostrará en la consola algo como:
```
🖥️  LCD Display Content:
╔════════════════════════════════════════════════╗
║Hello World!                                    ║
║Pi Zero Monitor                                 ║
║Temp: 23.4C                                     ║
║Humidity: 65.2%                                 ║
║                                                ║
║Uptime: 15s                                     ║
║LED                                             ║
╚════════════════════════════════════════════════╝
📊 Status: LED=ON, Temp=23.4°C
```

### 3. Probar la API Web
```bash
# Estado del sistema
curl http://localhost:3030/status

# Datos de sensores
curl http://localhost:3030/sensor

# Contenido de la pantalla (solo en simulación)
curl http://localhost:3030/display

# Controlar LED
curl -X POST -H 'Content-Type: application/json' \
     -d '{"state": true}' \
     http://localhost:3030/led
```

## 🔧 Configuración

### Cambiar a Modo Hardware
```bash
# Para usar con hardware real de Raspberry Pi
cargo run --features hardware --no-default-features
```

### Dependencias del Proyecto
```toml
[features]
default = ["simulation"]
hardware = ["rppal", "st7789"]
simulation = []
```

## 📋 Endpoints de la API

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/` | GET | Mensaje de bienvenida |
| `/status` | GET | Estado completo del sistema |
| `/sensor` | GET | Últimos datos de sensores |
| `/display` | GET | Contenido de la pantalla (simulación) |
| `/led` | POST | Controlar estado del LED |

## 🖥️ Ejemplo de Respuesta API

### GET /status
```json
{
  "uptime_seconds": 45,
  "led_status": true,
  "last_sensor_reading": {
    "temperature": 23.4,
    "humidity": 65.2,
    "timestamp": 1709856123
  },
  "display_content": "╔════════════════╗\n║Hello World!    ║\n..."
}
```

### GET /display (Solo en simulación)
```json
{
  "display_content": "╔════════════════╗\n║Hello World!    ║\n║Pi Zero Monitor ║\n...",
  "mode": "simulation"
}
```

## 🔌 Conexiones Hardware (Solo modo hardware)

Para usar con hardware real:

| Componente | Pin GPIO | Descripción |
|------------|----------|-------------|
| LED | 18 | LED de estado |
| LCD DC | 24 | Data/Command |
| LCD Reset | 25 | Reset |
| LCD SPI | SPI0 | MOSI, SCLK, CE0 |

## 🛠️ Desarrollo

### Estructura del Código
- `MockDisplay`: Simula la pantalla con caracteres ASCII
- `Display` trait: Abstracción para hardware/simulación
- Compilación condicional con `#[cfg(feature = "...")]`
- API web unificada para ambos modos

### Personalizar la Simulación
Edita la función `update_display_content()` en `src/main.rs` para cambiar lo que se muestra en la pantalla simulada.

## 🎨 Personalización

### Cambiar el Mensaje
Modifica la línea en `update_display_content()`:
```rust
display.draw_text("Hello World!", 10, 30, Rgb565::WHITE)?;
```

### Ajustar Frecuencia de Actualización
Cambia el intervalo en `display_update_task()`:
```rust
let mut interval = time::interval(Duration::from_secs(2)); // Cada 2 segundos
```

## 🐛 Solución de Problemas

### Error de Compilación Rust
Si encuentras errores de proxy de rustup, intenta:
```bash
# Resetear configuración de rustup
rustup default stable
rustup update
```

### Puerto 3030 en Uso
Cambia el puerto en `main()`:
```rust
warp::serve(routes).run(([0, 0, 0, 0], 8080)).await; // Puerto 8080
```

## 📚 Recursos Adicionales

- [Documentación ST7789](https://docs.rs/st7789/)
- [Embedded Graphics](https://docs.rs/embedded-graphics/)
- [RPPAL GPIO](https://docs.rs/rppal/)
