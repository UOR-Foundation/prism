//! `wiki-link-check` — validate every UOR-Framework wiki backlink in the
//! repository against the wiki source repo.
//!
//! See [`AGENTS.md` § 6](../../../AGENTS.md) for the complete specification
//! of what this tool does and why.
//!
//! Usage:
//!
//! ```text
//! wiki-link-check [OPTIONS] [PATH]
//!
//! Scan PATH (default: current directory) for URLs matching the
//! UOR-Framework wiki, and verify each refers to a page (and optional
//! anchor) that exists in the wiki source repository.
//!
//! Options:
//!   --wiki-path <DIR>    Use an already-cloned wiki tree at DIR.
//!                        Implies --no-clone.
//!   --wiki-rev  <REV>    Pin to a specific wiki revision (commit, tag, or
//!                        branch). Overrides PRISM_WIKI_REV.
//!   --cache-dir <DIR>    Override the default cache directory.
//!   --no-clone           Refuse to clone the wiki; require --wiki-path
//!                        or an existing cached clone.
//!   --quiet              Suppress the summary line on success.
//!   --verbose            Print every URL, not just broken ones.
//!   -h, --help           Print this message.
//!
//! Environment:
//!   PRISM_WIKI_REV       Default value for --wiki-rev when unset on the
//!                        command line.
//!
//! Exit codes:
//!   0  All wiki backlinks resolve.
//!   1  At least one backlink is broken.
//!   2  Usage or I/O error.
//! ```

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

mod scan;
mod slug;
mod wiki;

/// URL of the upstream wiki source repository.
const WIKI_REPO_URL: &str = "https://github.com/UOR-Foundation/UOR-Framework.wiki.git";

/// Parsed CLI arguments.
struct Args {
    scan_root: PathBuf,
    wiki_path: Option<PathBuf>,
    wiki_rev: Option<String>,
    cache_dir: Option<PathBuf>,
    no_clone: bool,
    quiet: bool,
    verbose: bool,
}

fn main() -> ExitCode {
    let args = match parse_args(env::args().skip(1)) {
        Ok(a) => a,
        Err(ArgError::Help) => {
            print!("{}", help_text());
            return ExitCode::SUCCESS;
        }
        Err(ArgError::Bad(msg)) => {
            eprintln!("wiki-link-check: {msg}");
            eprintln!("\n{}", help_text());
            return ExitCode::from(2);
        }
    };
    match run(&args) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("wiki-link-check: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &Args) -> Result<bool, Box<dyn std::error::Error>> {
    let wiki = open_wiki(args)?;
    let founds = scan::collect(&args.scan_root)?;
    let mut total = 0_usize;
    let mut broken_count = 0_usize;

    for found in &founds {
        // Skip URLs that originate inside the cached wiki clone itself —
        // the wiki freely cross-references its own pages and is not the
        // subject of validation.
        if found.file.starts_with(wiki.path()) {
            continue;
        }
        total += 1;
        match wiki.validate(&found.page, found.anchor.as_deref()) {
            Ok(()) => {
                if args.verbose {
                    println!(
                        "OK     {}:{}: {}",
                        found.file.display(),
                        found.line,
                        found.url
                    );
                }
            }
            Err(e) => {
                broken_count += 1;
                println!(
                    "BROKEN {}:{}: {}",
                    found.file.display(),
                    found.line,
                    found.url
                );
                println!("       {e}");
            }
        }
    }

    if !args.quiet {
        eprintln!(
            "wiki-link-check: scanned {total} backlink(s); {broken_count} broken; wiki at {}",
            wiki.path().display(),
        );
    }
    Ok(broken_count == 0)
}

fn open_wiki(args: &Args) -> Result<wiki::Wiki, Box<dyn std::error::Error>> {
    if let Some(p) = &args.wiki_path {
        return Ok(wiki::Wiki::open(p)?);
    }
    let cache = args.cache_dir.clone().unwrap_or_else(default_cache_dir);
    let rev = args
        .wiki_rev
        .clone()
        .or_else(|| env::var("PRISM_WIKI_REV").ok());
    if args.no_clone {
        if !cache.join(".git").exists() {
            return Err(format!("--no-clone set and no cached wiki at {}", cache.display()).into());
        }
    } else {
        wiki::clone_or_update(&cache, WIKI_REPO_URL, rev.as_deref())?;
    }
    Ok(wiki::Wiki::open(&cache)?)
}

fn default_cache_dir() -> PathBuf {
    let base = git_repo_root().unwrap_or_else(|| PathBuf::from("."));
    base.join(".cache").join("wiki-link-check").join("wiki")
}

fn git_repo_root() -> Option<PathBuf> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?;
    Some(PathBuf::from(s.trim()))
}

enum ArgError {
    Help,
    Bad(String),
}

fn parse_args<I: IntoIterator<Item = String>>(iter: I) -> Result<Args, ArgError> {
    let mut scan_root: Option<PathBuf> = None;
    let mut wiki_path: Option<PathBuf> = None;
    let mut wiki_rev: Option<String> = None;
    let mut cache_dir: Option<PathBuf> = None;
    let mut no_clone = false;
    let mut quiet = false;
    let mut verbose = false;

    let mut argv = iter.into_iter();
    while let Some(a) = argv.next() {
        match a.as_str() {
            "-h" | "--help" => return Err(ArgError::Help),
            "--wiki-path" => {
                let v = argv
                    .next()
                    .ok_or_else(|| ArgError::Bad("--wiki-path needs a value".into()))?;
                wiki_path = Some(PathBuf::from(v));
                no_clone = true;
            }
            "--wiki-rev" => {
                wiki_rev = Some(
                    argv.next()
                        .ok_or_else(|| ArgError::Bad("--wiki-rev needs a value".into()))?,
                );
            }
            "--cache-dir" => {
                let v = argv
                    .next()
                    .ok_or_else(|| ArgError::Bad("--cache-dir needs a value".into()))?;
                cache_dir = Some(PathBuf::from(v));
            }
            "--no-clone" => no_clone = true,
            "--quiet" => quiet = true,
            "--verbose" => verbose = true,
            other if !other.starts_with('-') => {
                if scan_root.is_some() {
                    return Err(ArgError::Bad(format!(
                        "unexpected positional argument: {other}"
                    )));
                }
                scan_root = Some(PathBuf::from(other));
            }
            other => return Err(ArgError::Bad(format!("unknown argument: {other}"))),
        }
    }

    Ok(Args {
        scan_root: scan_root.unwrap_or_else(|| PathBuf::from(".")),
        wiki_path,
        wiki_rev,
        cache_dir,
        no_clone,
        quiet,
        verbose,
    })
}

fn help_text() -> String {
    "wiki-link-check [OPTIONS] [PATH]\n\
     \n\
     Validate UOR-Framework wiki backlinks under PATH (default: \".\")\n\
     against the wiki source repository.\n\
     \n\
     Options:\n  \
       --wiki-path <DIR>   Use an already-cloned wiki at DIR (implies --no-clone)\n  \
       --wiki-rev  <REV>   Pin wiki revision (commit/tag/branch)\n  \
       --cache-dir <DIR>   Override default cache directory\n  \
       --no-clone          Require pre-existing wiki cache; do not clone\n  \
       --quiet             Suppress success summary\n  \
       --verbose           Print every URL, not just broken ones\n  \
       -h, --help          Show this help\n\
     \n\
     Environment:\n  \
       PRISM_WIKI_REV      Fallback for --wiki-rev\n\
     \n\
     Exit codes:\n  \
       0  all backlinks resolve\n  \
       1  one or more backlinks are broken\n  \
       2  usage or I/O error\n"
        .to_string()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Args {
        parse_args(args.iter().map(|s| (*s).to_string()))
            .map_err(|e| match e {
                ArgError::Help => "help".to_string(),
                ArgError::Bad(s) => s,
            })
            .unwrap()
    }

    #[test]
    fn defaults() {
        let a = parse(&[]);
        assert_eq!(a.scan_root, PathBuf::from("."));
        assert!(a.wiki_path.is_none());
        assert!(!a.no_clone);
        assert!(!a.quiet);
        assert!(!a.verbose);
    }

    #[test]
    fn positional_path() {
        let a = parse(&["crates"]);
        assert_eq!(a.scan_root, PathBuf::from("crates"));
    }

    #[test]
    fn wiki_path_implies_no_clone() {
        let a = parse(&["--wiki-path", "/tmp/wiki"]);
        assert_eq!(a.wiki_path, Some(PathBuf::from("/tmp/wiki")));
        assert!(a.no_clone);
    }

    #[test]
    fn flags() {
        let a = parse(&["--quiet", "--verbose", "--no-clone"]);
        assert!(a.quiet);
        assert!(a.verbose);
        assert!(a.no_clone);
    }

    #[test]
    fn unknown_arg_rejected() {
        let res = parse_args(["--bogus".to_string()]);
        assert!(matches!(res, Err(ArgError::Bad(_))));
    }

    #[test]
    fn wiki_rev_value_required() {
        let res = parse_args(["--wiki-rev".to_string()]);
        assert!(matches!(res, Err(ArgError::Bad(_))));
    }

    #[test]
    fn help_short_circuits() {
        let res = parse_args(["-h".to_string()]);
        assert!(matches!(res, Err(ArgError::Help)));
    }

    #[test]
    fn help_text_lists_options() {
        let h = help_text();
        for needle in &["--wiki-path", "--wiki-rev", "--no-clone", "PRISM_WIKI_REV"] {
            assert!(h.contains(needle), "help missing {needle}");
        }
    }

    #[test]
    fn ignored_paths_default() {
        // smoke: default cache dir lives below repo root and under .cache/
        let p = default_cache_dir();
        assert!(
            p.iter().any(|c| c == std::ffi::OsStr::new(".cache")),
            "{p:?}"
        );
    }
}
