//! Fase 0 — Caracterización del comportamiento ACTUAL de `render`, `lookup` y
//! `get` sin flags nuevos.
//!
//! Estos tests deben pasar antes de implementar `--out`/`--mask` y deben
//! seguir pasando después: fijan el contrato "sin flags nada cambia".

mod common;

use common::TestEnv;

#[test]
fn render_sin_flags_escribe_el_resultado_en_stdout() {
    let env = TestEnv::new("DB_HOST: localhost\nDB_PASSWORD: s3cret\n");
    let template = env.write_template(
        "template.j2",
        "Host: {{ DB_HOST }}; Password: {{ DB_PASSWORD }}",
    );

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .output()
        .expect("no se pudo ejecutar crypta render");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "render sin flags debe salir con 0. stderr: {stderr}"
    );
    assert_eq!(
        stdout, "Host: localhost; Password: s3cret",
        "render sin flags debe escribir el resultado en stdout"
    );
}

#[test]
fn render_sin_flags_no_crea_fichero_de_salida() {
    let env = TestEnv::new("DB_HOST: localhost\n");
    let template = env.write_template("template.j2", "Host: {{ DB_HOST }}");

    let output = env
        .command()
        .arg("render")
        .arg(&template)
        .output()
        .expect("no se pudo ejecutar crypta render");

    assert!(output.status.success());

    // Comportamiento actual: sin `--out` no se escribe ningún fichero nuevo.
    let rendered = env.home.join("rendered.conf");
    assert!(
        !rendered.exists(),
        "render sin flags no debe crear ficheros de salida"
    );
}

#[test]
fn lookup_sin_flags_imprime_el_valor_exacto_sin_newline() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["lookup", "API_TOKEN"])
        .output()
        .expect("no se pudo ejecutar crypta lookup");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "lookup sin flags debe salir con 0. stderr: {stderr}"
    );
    assert_eq!(
        stdout, "s3cr3t-value",
        "lookup sin flags debe imprimir el valor exacto, sin newline final"
    );
}

#[test]
fn lookup_sin_flags_con_clave_inexistente_falla() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    let output = env
        .command()
        .args(["lookup", "NO_EXISTE"])
        .output()
        .expect("no se pudo ejecutar crypta lookup");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "lookup de una clave inexistente debe fallar"
    );
    assert!(
        stderr.contains("NO_EXISTE"),
        "el error debe mencionar la clave. stderr: {stderr}"
    );
    assert!(
        stderr.contains("no existe"),
        "el error debe indicar que la clave no existe. stderr: {stderr}"
    );
}

#[test]
fn get_sin_flags_mantiene_la_copia_al_portapapeles() {
    let env = TestEnv::new("API_TOKEN: s3cr3t-value\n");

    // Sin display el portapapeles puede no estar disponible; con o sin éxito,
    // el comando debe seguir usando la ruta de copia y no imprimir el valor.
    let output = env
        .command()
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .args(["get", "API_TOKEN"])
        .output()
        .expect("no se pudo ejecutar crypta get");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("portapapeles") || stderr.contains("portapapeles"),
        "get sin flags debe seguir usando el portapapeles. stdout: {stdout} stderr: {stderr}"
    );
    assert!(
        !stdout.contains("s3cr3t-value") && !stderr.contains("s3cr3t-value"),
        "get sin flags no debe imprimir el valor. stdout: {stdout} stderr: {stderr}"
    );
}
