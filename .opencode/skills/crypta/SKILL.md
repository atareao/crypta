---
name: crypta
description: Flujo seguro para que agentes y automatizaciones usen Crypta sin exponer secretos en stdout, logs o contexto.
---

# crypta — flujo seguro para agentes

Usa esta skill cuando necesites leer, renderizar o verificar secretos con el CLI `crypta` en un contexto de agente. La regla base es simple: **nunca vuelques valores en claro a stdout**.

## Flujo recomendado

1. **Descubre las claves disponibles**

   ```sh
   crypta list
   ```

2. **Materializa una plantilla sin exponer el resultado**

   ```sh
   crypta render <tpl> --out <path>
   ```

   El fichero se crea con permisos `0600`. No se imprime nada por stdout. El destino no puede ser un symlink, un hardlink al store, un fichero no regular ni `secrets.yml`.

3. **Verifica que una clave existe sin leerla**

   ```sh
   crypta lookup --mask <KEY>
   crypta get --mask <KEY>
   ```

   La salida es `🔒 Secreto '<KEY>' existe (<N> caracteres).` con la longitud en caracteres. `get --mask` no toca el portapapeles.

4. **Persiste los cambios**

   ```sh
   crypta sync "mensaje del commit"
   ```

## Regla dura

**NUNCA ejecutes `crypta lookup` ni `crypta get` sin `--mask` en contexto de agente.** Tampoco redirijas la salida de `crypta render` a stdout ni vuelques valores a logs. Si necesitas el valor en claro, hazlo en una sesión humana interactiva, no en una herramienta de agente.

## Requisitos

- `sops` instalado y en `$PATH`.
- `age` (y `age-keygen` para `crypta init`) instalado.
- `SOPS_AGE_KEY_FILE` apuntando a la clave privada Age.

## Errores comunes

| Error | Causa | Solución |
|---|---|---|
| `sops: command not found` | `sops` no está instalado o no está en `$PATH`. | Instálalo y vuelve a intentarlo. |
| `$HOME` ausente o no definido | El proceso no tiene `$HOME`, necesario para localizar `~/.secrets/`. | Exporta `HOME` o ejecuta en un entorno con él definido. |
| `--out` apunta a `secrets.yml` | El destino es el propio store. | Elige otra ruta; el flag lo rechaza por seguridad. |
| `--out` apunta a un symlink o fichero no regular | El destino no es un fichero regular. | Usa una ruta nueva o un fichero regular. |
