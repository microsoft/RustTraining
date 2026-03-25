use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

/// (slug, title, description, category)
const BOOKS: &[(&str, &str, &str, &str)] = &[
    (
        "c-cpp-book",
        "Rust for C/C++ Programmers",
        "Move semantics, RAII, FFI, embedded, no_std",
        "bridge",
    ),
    (
        "csharp-book",
        "Rust for C# Programmers",
        "Best for Swift / C# / Java developers",
        "bridge",
    ),
    (
        "python-book",
        "Rust for Python Programmers",
        "Dynamic → static typing, GIL-free concurrency",
        "bridge",
    ),
    (
        "async-book",
        "Async Rust: From Futures to Production",
        "Tokio, streams, cancellation safety",
        "deep-dive",
    ),
    (
        "rust-patterns-book",
        "Rust Patterns",
        "Pin, allocators, lock-free structures, unsafe",
        "advanced",
    ),
    (
        "type-driven-correctness-book",
        "Type-Driven Correctness",
        "Type-state, phantom types, capability tokens",
        "expert",
    ),
    (
        "engineering-book",
        "Rust Engineering Practices",
        "Build scripts, cross-compilation, coverage, CI/CD",
        "practices",
    ),
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must live in a workspace subdirectory")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => {
            cmd_build();
            cmd_serve();
        }
        Some("deploy") => cmd_deploy(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("Unknown command: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let stream: &mut dyn Write = if code == 0 {
        &mut std::io::stdout()
    } else {
        &mut std::io::stderr()
    };
    let _ = writeln!(
        stream,
        "\
Usage: cargo xtask <COMMAND>

Commands:
  build    Build all books into site/ (for local preview)
  serve    Build and serve at http://localhost:3000
  deploy   Build all books into docs/ (for GitHub Pages)
  clean    Remove site/ and docs/ directories"
    );
    std::process::exit(code);
}

// ── build ────────────────────────────────────────────────────────────

fn cmd_build() {
    build_to("site");
}

fn cmd_deploy() {
    build_to("docs");
    println!("\nTo publish, commit docs/ and enable GitHub Pages → \"Deploy from a branch\" → /docs.");
}

fn build_to(dir_name: &str) {
    let root = project_root();
    let out = root.join(dir_name);

    if out.exists() {
        fs::remove_dir_all(&out).expect("failed to clean output dir");
    }
    fs::create_dir_all(&out).expect("failed to create output dir");

    println!("Building unified site into {dir_name}/\n");

    let mut ok = 0u32;
    for &(slug, _, _, _) in BOOKS {
        let book_dir = root.join(slug);
        if !book_dir.is_dir() {
            eprintln!("  ✗ {slug}/ not found, skipping");
            continue;
        }
        let dest = out.join(slug);
        let status = Command::new("mdbook")
            .args(["build", "--dest-dir"])
            .arg(&dest)
            .current_dir(&book_dir)
            .status()
            .expect("failed to run mdbook — is it installed?");

        if status.success() {
            println!("  ✓ {slug}");
            ok += 1;
        } else {
            eprintln!("  ✗ {slug} FAILED");
        }
    }
    println!("\n  {ok}/{} books built", BOOKS.len());

    write_landing_page(&out);
    println!("\nDone! Output in {dir_name}/");
}

fn category_label(cat: &str) -> &str {
    match cat {
        "bridge" => "Bridge",
        "deep-dive" => "Deep Dive",
        "advanced" => "Advanced",
        "expert" => "Expert",
        "practices" => "Practices",
        _ => cat,
    }
}

fn write_landing_page(site: &Path) {
    let cards: String = BOOKS
        .iter()
        .map(|&(slug, title, desc, cat)| {
            let label = category_label(cat);
            format!(
                r#"    <a class="card cat-{cat}" href="{slug}/">
      <h2>{title} <span class="label">{label}</span></h2>
      <p>{desc}</p>
    </a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Rust Training Books</title>
  <style>
    :root {{
      --bg: #1a1a2e;
      --card-bg: #16213e;
      --accent: #e94560;
      --text: #eee;
      --muted: #a8a8b3;
      --clr-bridge: #4ade80;
      --clr-deep-dive: #22d3ee;
      --clr-advanced: #fbbf24;
      --clr-expert: #c084fc;
      --clr-practices: #2dd4bf;
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 3rem 1rem;
    }}
    h1 {{ font-size: 2.5rem; margin-bottom: 0.5rem; }}
    h1 span {{ color: var(--accent); }}
    .subtitle {{ color: var(--muted); font-size: 1.1rem; margin-bottom: 1.2rem; }}

    /* Legend */
    .legend {{
      display: flex; flex-wrap: wrap; gap: 0.6rem 1.4rem;
      justify-content: center; margin-bottom: 2.2rem;
      font-size: 0.8rem; color: var(--muted);
    }}
    .legend-item {{ display: flex; align-items: center; gap: 0.35rem; }}
    .legend-dot {{
      width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0;
    }}

    /* Grid & Cards */
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1.5rem;
      max-width: 1000px;
      width: 100%;
    }}
    .card {{
      background: var(--card-bg);
      border-radius: 12px;
      padding: 1.5rem 1.5rem 1.5rem 1.25rem;
      text-decoration: none;
      color: var(--text);
      transition: transform 0.15s, box-shadow 0.15s;
      border: 1px solid rgba(255,255,255,0.05);
      border-left: 4px solid var(--stripe);
    }}
    .card:hover {{
      transform: translateY(-4px);
      box-shadow: 0 8px 25px color-mix(in srgb, var(--stripe) 30%, transparent);
      border-color: rgba(255,255,255,0.08);
      border-left-color: var(--stripe);
    }}
    .card h2 {{ font-size: 1.2rem; margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; }}
    .card p  {{ color: var(--muted); font-size: 0.9rem; line-height: 1.4; }}

    /* Category colours */
    .cat-bridge     {{ --stripe: var(--clr-bridge); }}
    .cat-deep-dive  {{ --stripe: var(--clr-deep-dive); }}
    .cat-advanced   {{ --stripe: var(--clr-advanced); }}
    .cat-expert     {{ --stripe: var(--clr-expert); }}
    .cat-practices  {{ --stripe: var(--clr-practices); }}

    /* Label pill */
    .label {{
      font-size: 0.55rem; font-weight: 700; letter-spacing: 0.08em;
      text-transform: uppercase; padding: 0.15em 0.55em;
      border-radius: 4px; white-space: nowrap; flex-shrink: 0;
      color: var(--bg); background: var(--stripe);
    }}

    footer {{ margin-top: 3rem; color: var(--muted); font-size: 0.85rem; }}
  </style>
</head>
<body>
  <h1>🦀 <span>Rust</span> Training Books</h1>
  <p class="subtitle">Pick the guide that matches your background</p>

  <div class="legend">
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-bridge)"></span> Bridge &mdash; learn Rust from another language</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-deep-dive)"></span> Deep Dive</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-advanced)"></span> Advanced</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-expert)"></span> Expert</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-practices)"></span> Practices</span>
  </div>

  <div class="grid">
{cards}
  </div>
  <footer>Built with <a href="https://rust-lang.github.io/mdBook/" style="color:var(--accent)">mdBook</a></footer>
</body>
</html>
"##
    );

    let path = site.join("index.html");
    fs::write(&path, html).expect("failed to write index.html");
    println!("  ✓ index.html");
}

/// Resolve `request_target` (HTTP request path, e.g. `/foo/bar?x=1`) to a file under `site_canon`.
/// Returns `None` for traversal attempts, missing files, or paths that escape `site_canon` (symlinks).
fn resolve_site_file(site_canon: &Path, request_target: &str) -> Option<PathBuf> {
    let path_only = request_target.split('?').next()?.split('#').next()?;
    let decoded = percent_decode_path(path_only);
    if decoded.as_bytes().contains(&0) {
        return None;
    }
    let rel = decoded.trim_start_matches('/');
    let mut file_path = site_canon.to_path_buf();
    if !rel.is_empty() {
        for seg in rel.split('/').filter(|s| !s.is_empty()) {
            if seg == ".." {
                return None;
            }
            file_path.push(seg);
        }
    }
    if file_path.is_dir() {
        file_path.push("index.html");
    }
    let real = fs::canonicalize(&file_path).ok()?;
    if !real.starts_with(site_canon) {
        return None;
    }
    real.is_file().then_some(real)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn percent_decode_path(input: &str) -> String {
    let mut decoded = Vec::with_capacity(input.len());
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                decoded.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

// ── serve ────────────────────────────────────────────────────────────

fn cmd_serve() {
    let site = project_root().join("site");
    let site_canon = fs::canonicalize(&site).expect(
        "site/ not found — run `cargo xtask build` first (e.g. `cargo xtask serve` runs build automatically)",
    );
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).expect("failed to bind port 3000");

    // Handle Ctrl+C gracefully so cargo doesn't report an error
    ctrlc_exit();

    println!("\nServing at http://{addr}  (Ctrl+C to stop)");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        if let Some(file_path) = resolve_site_file(&site_canon, path) {
            let body = fs::read(&file_path).unwrap_or_default();
            let mime = guess_mime(&file_path);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&body);
        } else {
            let body = b"404 Not Found";
            let header = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(body);
        }
    }
}

/// Install a Ctrl+C handler that exits cleanly (code 0) instead of
/// letting the OS terminate with STATUS_CONTROL_C_EXIT.
fn ctrlc_exit() {
    unsafe {
        libc_set_handler();
    }
}

#[cfg(windows)]
unsafe fn libc_set_handler() {
    // SetConsoleCtrlHandler via the Windows API
    extern "system" {
        fn SetConsoleCtrlHandler(
            handler: Option<unsafe extern "system" fn(u32) -> i32>,
            add: i32,
        ) -> i32;
    }
    unsafe extern "system" fn handler(_ctrl_type: u32) -> i32 {
        std::process::exit(0);
    }
    unsafe {
        SetConsoleCtrlHandler(Some(handler), 1);
    }
}

#[cfg(not(windows))]
unsafe fn libc_set_handler() {
    // On Unix, register SIGINT via libc
    extern "C" {
        fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    }
    extern "C" fn handler(_sig: i32) {
        std::process::exit(0);
    }
    unsafe {
        signal(2 /* SIGINT */, handler);
    }
}

fn guess_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

// ── clean ────────────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir_name in ["site", "docs"] {
        let dir = root.join(dir_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("failed to remove dir");
            println!("Removed {dir_name}/");
        }
    }
}
