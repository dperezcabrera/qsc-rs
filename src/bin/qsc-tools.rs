use clap::{Parser, Subcommand, Args};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Parser)]
#[command(version, about="QSC tools (ML-DSA-3 keygen, addr, sign, verify)")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen(KeygenArgs),
    Addr(AddrArgs),
    Sign(SignArgs),
    Verify(VerifyArgs),
}

#[derive(Args)]
struct KeygenArgs {
    /// Ruta base (sin extensión) para escribir .sk y .pk, ej: --out /keys/alice
    #[arg(long)]
    out: Option<String>,
    /// Alternativa: directorio de salida
    #[arg(long)]
    out_dir: Option<String>,
    /// Alternativa: nombre base sin extensión
    #[arg(long)]
    name: Option<String>,
}
#[derive(Args)]
struct AddrArgs {
    #[arg(long)]
    pk_file: String,
}
#[derive(Args)]
struct SignArgs {
    /// Clave secreta en hex O ruta a fichero .sk con hex
    #[arg(long)]
    sk: String,
    #[arg(long)]
    payload: String,
}
#[derive(Args)]
struct VerifyArgs {
    #[arg(long)]
    pk: String,
    #[arg(long)]
    payload: String,
    #[arg(long)]
    sig: String,
}

fn read_hex_or_file(s: &str) -> Result<String> {
    let p = Path::new(s);
    if p.exists() {
        Ok(fs::read_to_string(p)?.trim().to_string())
    } else {
        Ok(s.trim().to_string())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Keygen(a) => {
            let (sk, pk) = qsc_rs_simple_contracts::pq::keygen_mldsa3();

            let (out_dir, name) = if let Some(out) = a.out {
                let p = Path::new(&out);
                let dir = p.parent().unwrap_or(Path::new(".")).to_string_lossy().to_string();
                let name = p.file_name().unwrap().to_string_lossy().to_string();
                (dir, name)
            } else {
                (a.out_dir.unwrap_or_else(|| "keys".into()), a.name.unwrap_or_else(|| "alice".into()))
            };

            fs::create_dir_all(&out_dir)?;
            let skp = format!("{}/{}.sk", out_dir, name);
            let pkp = format!("{}/{}.pk", out_dir, name);
            fs::write(&skp, hex::encode(sk))?;
            fs::write(&pkp, hex::encode(pk))?;
            println!("Wrote {}", pkp);
            println!("Wrote {}", skp);
        }
        Commands::Addr(a) => {
            let pk_hex = fs::read_to_string(&a.pk_file)?.trim().to_string();
            let pk = hex::decode(&pk_hex)?;
            let addr = qsc_rs_simple_contracts::pq::address_from_pk(&pk);
            println!("{}", addr);
        }
        Commands::Sign(a) => {
            let sk_hex = read_hex_or_file(&a.sk)?;
            let payload = a.payload;
            let sk = hex::decode(sk_hex.trim())?;
            let sig = qsc_rs_simple_contracts::pq::sign_mldsa3(payload.as_bytes(), &sk);
            println!("{}", hex::encode(sig));
        }
        Commands::Verify(a) => {
            let pk = hex::decode(a.pk.trim())?;
            let sig = hex::decode(a.sig.trim())?;
            let ok = qsc_rs_simple_contracts::pq::verify_mldsa3(a.payload.as_bytes(), &sig, &pk);
            println!("{}", if ok { "OK" } else { "FAIL" });
        }
    }
    Ok(())
}
