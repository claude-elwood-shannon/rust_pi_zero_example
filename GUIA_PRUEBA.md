# 🚀 Guía Completa para Probar la Aplicación Rust Pi Zero

## 📋 Requisitos Previos

### 1. Instalar Rust
```bash
# En Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# En Windows
# Descargar desde: https://rustup.rs/
```

### 2. Verificar instalación
```bash
rustc --version
cargo --version
```

## 📥 Descargar el Proyecto

### Opción A: Clonar desde GitHub
```bash
git clone https://github.com/claude-elwood-shannon/rust_pi_zero_example.git
cd rust_pi_zero_example
```

### Opción B: Descargar ZIP
1. Ir a: https://github.com/claude-elwood-shannon/rust_pi_zero_example
2. Click en "Code" → "Download ZIP"
3. Extraer y entrar al directorio

## 🏃‍♂️ Ejecutar la Aplicación

### Método 1: Script Automático (Recomendado)
```bash
# Dar permisos de ejecución
chmod +x run_simulation.sh

# Ejecutar
./run_simulation.sh
```

### Método 2: Comando Directo
```bash
cargo run --target x86_64-unknown-linux-gnu
```

### Método 3: Solo Compilar
```bash
cargo build --target x86_64-unknown-linux-gnu
```

## 🖥️ Lo que Verás

### Pantalla LCD Simulada
```
╔════════════════════════════════════════════════╗
║ Hello World!                                   ║
║                                                ║
║ Pi Zero Monitor                                ║
║ Temp: 25.3C                                    ║
║                                                ║
║ Humidity: 65.2%                                ║
║                                                ║
║                                                ║
║ Uptime: 120s                                   ║
║ LED                                            ║
║                                                ║
║                                                ║
║                                                ║
╚════════════════════════════════════════════════╝
```

### Datos en Consola
```
📊 Status: LED=OFF, Temp=25.3°C
```

## 🌐 Probar la API Web

### 1. Verificar que el servidor esté corriendo
```bash
curl http://localhost:3030/
# Respuesta: "Raspberry Pi Zero Rust Server is running!"
```

### 2. Obtener estado completo del sistema
```bash
curl http://localhost:3030/status
```
**Respuesta JSON:**
```json
{
  "uptime_seconds": 139,
  "led_status": true,
  "last_sensor_reading": {
    "temperature": 34.835,
    "humidity": 51.440002,
    "timestamp": 1754179702
  },
  "display_content": "Hello World!..."
}
```

### 3. Leer sensores
```bash
curl http://localhost:3030/sensor
```
**Respuesta:**
```json
{
  "temperature": 23.36,
  "humidity": 71.56,
  "timestamp": 1754179727
}
```

### 4. Ver contenido de la pantalla
```bash
curl http://localhost:3030/display
```
**Respuesta:**
```json
{
  "display_content": "Hello World!...",
  "mode": "simulation"
}
```

### 5. Controlar el LED
```bash
# Encender LED
curl -X POST -H 'Content-Type: application/json' -d '{"state": true}' http://localhost:3030/led

# Apagar LED
curl -X POST -H 'Content-Type: application/json' -d '{"state": false}' http://localhost:3030/led
```
**Respuesta:**
```json
{
  "led_state": true,
  "success": true
}
```

## 🔧 Comandos Útiles

### Detener la aplicación
```bash
# Ctrl+C en la terminal donde corre
# O desde otra terminal:
pkill -f rust_pi_zero_example
```

### Ver logs detallados
```bash
RUST_LOG=debug cargo run --target x86_64-unknown-linux-gnu
```

### Compilar para Raspberry Pi (cross-compilation)
```bash
# Instalar target ARM
rustup target add arm-unknown-linux-gnueabihf

# Compilar para Pi Zero
cargo build --target arm-unknown-linux-gnueabihf --features hardware --no-default-features
```

## 🎯 Características que Puedes Observar

### ✅ Pantalla LCD Virtual
- **"Hello World!"** en la primera línea
- Temperatura y humedad simuladas
- Contador de tiempo de funcionamiento
- Alertas de temperatura alta (>30°C)
- Actualización cada 2 segundos

### ✅ Sensores Simulados
- Temperatura: 18°C - 35°C
- Humedad: 30% - 80%
- Valores realistas y variables

### ✅ LED Virtual
- Estado ON/OFF
- Control via API
- Indicador visual en pantalla

### ✅ API REST Completa
- 5 endpoints funcionales
- Respuestas JSON estructuradas
- CORS habilitado
- Puerto 3030

## 🐛 Solución de Problemas

### Error: "failed to select a version for st7789"
```bash
# Ya está solucionado en el código, pero si aparece:
cargo update
```

### Error: "linker not found"
```bash
# En Ubuntu/Debian:
sudo apt install build-essential

# En macOS:
xcode-select --install
```

### Puerto 3030 ocupado
```bash
# Cambiar puerto en src/main.rs línea con .serve()
# O matar proceso que usa el puerto:
sudo lsof -ti:3030 | xargs kill -9
```

### Permisos en Linux
```bash
chmod +x run_simulation.sh
```

## 📱 Probar desde el Navegador

Abre tu navegador y visita:
- http://localhost:3030/ - Mensaje de bienvenida
- http://localhost:3030/status - Estado JSON completo
- http://localhost:3030/sensor - Datos de sensores
- http://localhost:3030/display - Contenido de pantalla

## 🎉 ¡Listo!

Si ves el mensaje **"Hello World!"** en la pantalla ASCII y puedes acceder a la API, ¡la aplicación está funcionando perfectamente!

La aplicación simula un monitor IoT completo para Raspberry Pi Zero con:
- Pantalla LCD con "Hello World!"
- Sensores de temperatura y humedad
- Control de LED
- API web para monitoreo remoto
- Logs en tiempo real

---
**Repositorio:** https://github.com/claude-elwood-shannon/rust_pi_zero_example
**Documentación completa:** README.md y README_SIMULATION.md
