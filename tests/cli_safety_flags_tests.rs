//! Escenarios del change `cli-safety-flags`.
//!
//! Cubren los flags `render --out` y `lookup/get --mask`, las protecciones
//! del fichero de salida y la ausencia de fugas de secretos por stdout/stderr.

mod common;

use common::TestEnv;

const SECRET: &str = "s3cr3t-value";

fn mask_message(key: &str, length: usize) -> String {
    format!("🔒 Secreto '{key}' existe ({length} caracteres).\n")
}

/// 1. `render --out <PATH>` escribe el resultado en el fichero y deja stdout
///    vacío. El valor no debe aparecer por stderr.
#[test]
fn render_out_escribe_fichero_y_deja_stdout_vacio() {
    let env = TestEnv::new("DB_HOST: localhost\nDB_PASSWORD: s3cr3t-value\n");
    let template = env.write_template(
        "template.j2",
        "Host: {{ DB_HOST }}; Password: {{ DB_PASSWORD }}",
    );
    let out_path = env.home.join("rendered.conf");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "render --out debe salir con 0. stderr: {stderr}"
    );
    assert!(
        stdout.is_empty(),
        "render --out no debe escribir nada en stdout. stdout: {stdout}"
    );
    assert!(
        !stderr.contains(SECRET),
        "el valor no debe filtrarse por stderr. stderr: {stderr}"
    );

    let written = std::fs::read_to_string(&out_path)
        .expect("render --out debe haber creado el fichero de salida");
    assert_eq!(
        written, "Host: localhost; Password: s3cr3t-value",
        "el fichero debe contener la plantilla renderizada"
    );
}

/// 2. `render --out` con una ruta no escribible: exit != 0 y error en español
///    sin filtrar el valor.
#[test]
fn render_out_con_ruta_no_escribible_falla_sin_filtrar_el_valor() {
    let env = TestEnv::new("DB_PASSWORD: s3cr3t-value\n");
    let template = env.write_template("template.j2", "Password: {{ DB_PASSWORD }}");

    // Un directorio existente como destino: escribir en él falla siempre
    // (incluso como root), así que no depende de permisos del usuario.
    let target_dir = env.home.join("destino-directorio");
    std::fs::create_dir(&target_dir).expect("no se pudo crear el directorio destino");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&target_dir)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "render --out sobre una ruta no escribible debe fallar"
    );
    assert!(
        stderr.contains("escribir"),
        "el error debe ser un mensaje de escritura en español. stderr: {stderr}"
    );
    assert!(
        !stderr.contains(SECRET),
        "el error no debe filtrar el valor. stderr: {stderr}"
    );
    assert!(
        !stderr.contains("unexpected argument"),
        "el fallo no debe ser de clap rechazando el flag. stderr: {stderr}"
    );
}

/// 3. `lookup --mask` muestra existencia y longitud exacta, nunca el valor.
#[test]
fn lookup_mask_muestra_metadatos_y_no_el_valor() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["lookup", "API_TOKEN", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta lookup --mask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "lookup --mask debe salir con 0. stderr: {stderr}"
    );
    assert_eq!(
        stdout,
        mask_message("API_TOKEN", 12),
        "lookup --mask debe mostrar la clave y la longitud exacta"
    );
    assert!(
        !stdout.contains(SECRET) && !stderr.contains(SECRET),
        "lookup --mask no debe filtrar el valor. stdout: {stdout} stderr: {stderr}"
    );
}

/// 4. `get --mask` no copia al portapapeles ni imprime el valor.
#[test]
fn get_mask_no_copia_ni_imprime_el_valor() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    // Sin servidor gráfico accesible, cualquier intento de abrir el
    // portapapeles fallaría y haría que el comando saliera con error. Que
    // `get --mask` termine con 0 demuestra que no lo toca.
    let output = env
        .command()
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .args(["get", "API_TOKEN", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta get --mask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "get --mask debe salir con 0 sin tocar el portapapeles. stderr: {stderr}"
    );
    assert_eq!(
        stdout,
        mask_message("API_TOKEN", 12),
        "get --mask debe mostrar la clave y la longitud exacta"
    );
    assert!(
        !stdout.contains(SECRET) && !stderr.contains(SECRET),
        "get --mask no debe filtrar el valor. stdout: {stdout} stderr: {stderr}"
    );
    assert!(
        !stdout.contains("portapapeles") && !stderr.contains("portapapeles"),
        "get --mask no debe intentar copiar al portapapeles. stdout: {stdout} stderr: {stderr}"
    );
}

/// 5. `lookup --mask` con clave inexistente: exit != 0 con error en español.
#[test]
fn lookup_mask_con_clave_inexistente_falla() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["lookup", "NO_EXISTE", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta lookup --mask");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "lookup --mask de una clave inexistente debe fallar"
    );
    assert!(
        stderr.contains("NO_EXISTE"),
        "el error debe mencionar la clave. stderr: {stderr}"
    );
    assert!(
        stderr.contains("no existe"),
        "el error debe indicar que la clave no existe. stderr: {stderr}"
    );
    assert!(
        !stderr.contains("unexpected argument"),
        "el fallo no debe ser de clap rechazando el flag. stderr: {stderr}"
    );
}

/// 6. `get --mask` con clave inexistente: exit != 0 con error en español.
#[test]
fn get_mask_con_clave_inexistente_falla() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["get", "NO_EXISTE", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta get --mask");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "get --mask de una clave inexistente debe fallar"
    );
    assert!(
        stderr.contains("NO_EXISTE") && stderr.contains("no existe"),
        "el error debe indicar que la clave no existe. stderr: {stderr}"
    );
}

/// 7. Un error de plantilla no debe filtrar el valor por stdout ni stderr.
#[test]
fn render_out_con_error_de_plantilla_no_filtra_el_valor() {
    let env = TestEnv::new("DB_PASSWORD: s3cr3t-value\n");
    let template = env.write_template("t.j2", "{{ DB_PASSWORD | int }}");
    let out_path = env.home.join("o.txt");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "un error de plantilla debe salir con código distinto de 0"
    );
    assert!(
        !stdout.contains(SECRET) && !stderr.contains(SECRET),
        "el valor no debe filtrarse. stdout: {stdout} stderr: {stderr}"
    );
    assert!(
        !stderr.contains("Referenced variables"),
        "el error no debe volcar las variables referenciadas. stderr: {stderr}"
    );
    assert!(
        !out_path.exists(),
        "no debe crearse el fichero de salida si falla el renderizado"
    );
}

/// 8. `RUST_LOG=info` no debe contaminar stdout con `render --out`.
#[test]
fn render_out_con_rust_log_info_deja_stdout_vacio() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let out_path = env.home.join("rendered.conf");

    let output = env
        .command()
        .env("RUST_LOG", "info")
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "render --out debe salir con 0. stderr: {stderr}"
    );
    assert!(
        stdout.is_empty(),
        "los logs no deben escribirse en stdout. stdout: {stdout}"
    );
    assert_eq!(
        std::fs::read_to_string(&out_path).expect("falta el fichero de salida"),
        "Host: localhost"
    );
}

/// 9. El fichero de salida debe crearse con permisos 0600.
#[cfg(unix)]
#[test]
fn render_out_crea_fichero_con_permisos_0600() {
    use std::os::unix::fs::PermissionsExt;

    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let out_path = env.home.join("rendered.conf");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    assert!(
        output.status.success(),
        "render --out debe salir con 0. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mode = std::fs::metadata(&out_path)
        .expect("falta el fichero de salida")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o600, "el fichero de salida debe tener permisos 0600");
}

/// 10. `render --out` no debe sobrescribir el archivo de secretos.
#[test]
fn render_out_rechaza_sobrescribir_el_archivo_de_secretos() {
    let original = "DB_HOST: localhost\nDB_PASSWORD: s3cr3t-value\n";
    let env = TestEnv::new(original);
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&env.secrets_file)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "render --out no debe aceptar el archivo de secretos como destino"
    );
    assert!(
        stderr.contains("archivo de secretos"),
        "el error debe explicar que no se puede sobrescribir el archivo de secretos. stderr: {stderr}"
    );
    assert!(
        !stderr.contains(SECRET),
        "el error no debe filtrar el valor. stderr: {stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&env.secrets_file).expect("falta el archivo de secretos"),
        original,
        "el archivo de secretos debe quedar intacto"
    );
}

/// 11. `render --out` no debe seguir enlaces simbólicos.
#[cfg(unix)]
#[test]
fn render_out_rechaza_enlaces_simbolicos() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let target = env.home.join("target.txt");
    std::fs::write(&target, "original").expect("no se pudo crear el objetivo");
    let link = env.home.join("link.txt");
    std::os::unix::fs::symlink(&target, &link).expect("no se pudo crear el enlace simbólico");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&link)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "render --out no debe seguir un enlace simbólico"
    );
    assert!(
        stderr.contains("enlace simbólico"),
        "el error debe mencionar el enlace simbólico. stderr: {stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&target).expect("falta el objetivo"),
        "original",
        "el objetivo del enlace no debe modificarse"
    );
}

/// 12. `render --out` con un directorio inexistente falla con error claro.
#[test]
fn render_out_con_directorio_inexistente_falla() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let out_path = env.home.join("no-existe").join("rendered.conf");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "render --out debe fallar si el directorio no existe"
    );
    assert!(
        stderr.contains("escribir"),
        "el error debe ser un mensaje de escritura en español. stderr: {stderr}"
    );
    assert!(
        !stderr.contains(SECRET),
        "el error no debe filtrar el valor. stderr: {stderr}"
    );
}

/// 13. La longitud se cuenta en caracteres, no en bytes.
#[test]
fn lookup_mask_cuenta_caracteres_no_bytes() {
    // "contraseña" tiene 10 caracteres y 11 bytes (la `ñ` ocupa 2 bytes).
    let env = TestEnv::new("PASSWORD: contraseña\n");

    let output = env
        .command()
        .args(["lookup", "PASSWORD", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta lookup --mask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "lookup --mask debe salir con 0. stderr: {stderr}"
    );
    assert_eq!(
        stdout,
        mask_message("PASSWORD", 10),
        "la longitud debe contarse en caracteres"
    );
    assert!(
        !stdout.contains("contraseña") && !stderr.contains("contraseña"),
        "el valor no debe filtrarse. stdout: {stdout} stderr: {stderr}"
    );
}

/// 14. Un valor YAML no string se distingue de una clave ausente.
#[test]
fn lookup_mask_con_valor_no_string_falla() {
    let env = TestEnv::new("API_TOKEN: 12345\n");

    let output = env
        .command()
        .args(["lookup", "API_TOKEN", "--mask"])
        .output()
        .expect("no se pudo ejecutar crypta lookup --mask");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "un valor no string debe provocar un error"
    );
    assert!(
        stderr.contains("API_TOKEN"),
        "el error debe mencionar la clave. stderr: {stderr}"
    );
    assert!(
        stderr.contains("no es una cadena"),
        "el error debe indicar que el valor no es una cadena. stderr: {stderr}"
    );
    assert!(
        !stderr.contains("no existe"),
        "no debe confundirse con una clave ausente. stderr: {stderr}"
    );
    assert!(
        !stdout.contains("12345") && !stderr.contains("12345"),
        "el valor no debe filtrarse. stdout: {stdout} stderr: {stderr}"
    );
}

/// 15. `--mask --log-access` registra la clave pero nunca el valor.
#[test]
fn mask_con_log_access_no_escribe_el_valor_en_el_log() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["lookup", "API_TOKEN", "--mask", "--log-access"])
        .output()
        .expect("no se pudo ejecutar crypta lookup --mask --log-access");

    assert!(
        output.status.success(),
        "lookup --mask --log-access debe salir con 0. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let log_path = env.secrets_dir.join("access.log");
    let log = std::fs::read_to_string(&log_path).expect("no se creó access.log");
    assert!(
        log.contains("lookup") && log.contains("API_TOKEN"),
        "el log debe registrar la acción y la clave. log: {log}"
    );
    assert!(
        !log.contains(SECRET),
        "el log no debe contener el valor. log: {log}"
    );
}

/// 16. Un fichero preexistente con permisos 0644 queda en 0600 tras `render --out`.
#[cfg(unix)]
#[test]
fn render_out_sobrescribe_fichero_0644_con_0600() {
    use std::os::unix::fs::PermissionsExt;

    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let out_path = env.home.join("existing.conf");
    std::fs::write(&out_path, "contenido previo").expect("no se pudo crear el fichero previo");
    std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(0o644))
        .expect("no se pudieron fijar los permisos 0644");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    assert!(
        output.status.success(),
        "render --out debe salir con 0. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mode = std::fs::metadata(&out_path)
        .expect("falta el fichero de salida")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o600, "el fichero debe quedar con permisos 0600");
    assert_eq!(
        std::fs::read_to_string(&out_path).expect("no se pudo leer el fichero de salida"),
        "Host: localhost"
    );
}

/// 17. Un hardlink a `secrets.yml` se rechaza y el store queda intacto.
#[cfg(unix)]
#[test]
fn render_out_rechaza_hardlink_al_archivo_de_secretos() {
    let original = "DB_HOST: localhost\nDB_PASSWORD: s3cr3t-value\n";
    let env = TestEnv::new(original);
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let link = env.home.join("hardlink.yml");
    std::fs::hard_link(&env.secrets_file, &link).expect("no se pudo crear el hardlink");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&link)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "un hardlink al archivo de secretos debe rechazarse"
    );
    assert!(
        stderr.contains("archivo de secretos"),
        "el error debe indicar que no se puede sobrescribir el archivo de secretos. stderr: {stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&env.secrets_file).expect("no se pudo leer el store"),
        original,
        "el store de secretos debe quedar intacto"
    );
}

/// 18. `RUST_LOG=debug` no debe contaminar stdout con `render --out`.
#[test]
fn render_out_con_rust_log_debug_deja_stdout_vacio() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let out_path = env.home.join("rendered.conf");

    let output = env
        .command()
        .env("RUST_LOG", "debug")
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&out_path)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    assert!(
        output.status.success(),
        "render --out debe salir con 0. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "los logs de debug no deben escribir en stdout. stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(
        std::fs::read_to_string(&out_path).expect("falta el fichero de salida"),
        "Host: localhost"
    );
}

/// 19. Un destino que no es un fichero regular (FIFO) se rechaza sin bloquear.
#[cfg(unix)]
#[test]
fn render_out_rechaza_ficheros_no_regulares() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");
    let fifo = env.home.join("fifo");

    let status = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .expect("no se pudo ejecutar mkfifo");
    assert!(status.success(), "mkfifo debe crear el FIFO");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .arg("--out")
        .arg(&fifo)
        .output()
        .expect("no se pudo ejecutar crypta render --out");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "un FIFO como destino debe rechazarse"
    );
    assert!(
        stderr.contains("fichero regular"),
        "el error debe indicar que no es un fichero regular. stderr: {stderr}"
    );
    assert!(
        !stderr.contains(SECRET),
        "el error no debe filtrar el valor. stderr: {stderr}"
    );
}
