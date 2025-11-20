# lab5_graphics

Este laboratorio utiliza un sistema de procedural noise escrito en Rust para generar superficies planetarias, atmósferas y variaciones naturales sin necesidad de texturas externas. Todo se calcula matemáticamente a partir de una posición 3D, lo cual permite crear mundos infinitamente detallados, reproducibles y ligeros.

De los noise elegidos estan:
- Simplex Noise (simplex_noise):
Una variante más moderna del Perlin noise, con menos artefactos direccionales,mejores gradientes y mayor rendimiento en 3D. Es la base para el detalle fino de la superficie del planeta.

- Fractal Brownian Motion — fBm (fbm / fbm_simplex): Combina múltiples capas (octavas) de ruido Simplex para crear detalles más complejos.
Cada capa agrega más frecuencia, menos amplitud y esto genera superficies con riqueza visual, parecidas a terreno real.

- Turbulence (turbulence): Usa el valor absoluto del ruido para generar efectos de nubes densas, patrones duros y superficies rocosas. Produce bordes más agresivos.
  
- Voronoi / Cellular Noise (voronoi): Genera patrones celulares, útil para cráteres, formaciones minerales y estructuras hexagonales simples. Se implementa de forma ligera para no afectar el rendimiento.
  
- Warp Noise (warp_noise): Distorsiona el espacio antes de evaluarlo. Esto crea remolinos, vórtices y patrones fluidos complejos. Es la técnica responsable de muchas de las texturas más “orgánicas”.

## GIF del sistema solar

![](https://github.com/DiegoOF07/lab5_graphics/blob/main/assets/solar_system.gif)
