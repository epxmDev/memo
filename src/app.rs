use crate::model::Note;
use crate::storage::Storage;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::error::Error;
use std::io;

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> AppResult<()> {
    let storage = Storage::new()?;
    let mut notes = storage.load()?;
    let mut editor = Editor::<(), DefaultHistory>::new()?;
    let history_path = storage.history_path();

    print_welcome(notes.len());

    if history_path.exists() {
        editor.load_history(&history_path)?;
    }

    loop {
        match editor.readline("memo> ") {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                editor.add_history_entry(input)?;
                if execute(input, &mut notes, &storage)? {
                    break;
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(error) => return Err(Box::new(error)),
        }
    }

    editor.save_history(&history_path)?;
    Ok(())
}

fn execute(input: &str, notes: &mut Vec<Note>, storage: &Storage) -> AppResult<bool> {
    let mut parts = input.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or_default();
    let argument = parts.next().unwrap_or_default().trim();

    match command {
        "add" => add_note(argument, notes, storage)?,
        "list" => list_notes(notes),
        "search" => search_notes(argument, notes),
        "rm" => remove_note(argument, notes, storage)?,
        "help" => print_help(),
        "exit" | "quit" => return Ok(true),
        _ => println!(
            "{} Commande inconnue. Utilisez {} pour afficher l'aide.",
            "Erreur:".red().bold(),
            "help".cyan()
        ),
    }

    Ok(false)
}

fn add_note(text: &str, notes: &mut Vec<Note>, storage: &Storage) -> AppResult<()> {
    if text.is_empty() {
        println!("{} Usage: add <texte>", "Erreur:".red().bold());
        return Ok(());
    }

    let id = notes
        .len()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("impossible de générer un nouvel identifiant"))?;
    notes.push(Note::new(id, text.to_owned()));

    if let Err(error) = storage.save(notes) {
        notes.pop();
        return Err(Box::new(error));
    }

    println!(
        "{} Note {} ajoutée.",
        "OK:".green().bold(),
        id.to_string().cyan()
    );
    Ok(())
}

fn list_notes(notes: &[Note]) {
    if notes.is_empty() {
        println!(
            "{} Ajoutez votre première note avec {}.",
            "Carnet vide.".yellow().bold(),
            "add <texte>".cyan()
        );
        return;
    }

    render_notes(notes.iter());
    println!("{} note(s)", notes.len().to_string().dimmed());
}

fn search_notes(keyword: &str, notes: &[Note]) {
    if keyword.is_empty() {
        println!("{} Usage: search <mot-clé>", "Erreur:".red().bold());
        return;
    }

    let keyword = keyword.to_lowercase();
    let matches: Vec<&Note> = notes
        .iter()
        .filter(|note| note.text.to_lowercase().contains(&keyword))
        .collect();

    if matches.is_empty() {
        println!(
            "{} Aucune note ne contient « {} ».",
            "Recherche vide.".yellow().bold(),
            keyword
        );
        return;
    }

    render_notes(matches.iter().copied());
    println!(
        "{} résultat(s) pour « {} »",
        matches.len().to_string().dimmed(),
        keyword
    );
}

fn render_notes<'a>(notes: impl IntoIterator<Item = &'a Note>) {
    let notes: Vec<&Note> = notes.into_iter().collect();
    let id_width = notes
        .iter()
        .map(|note| note.id.to_string().len())
        .max()
        .unwrap_or(2)
        .max(2);
    let date_width = 19;
    let text_width = notes
        .iter()
        .map(|note| note.text.chars().count())
        .max()
        .unwrap_or(4)
        .max(4);
    let border = format!(
        "+{}+{}+{}+",
        "-".repeat(id_width + 2),
        "-".repeat(date_width + 2),
        "-".repeat(text_width + 2)
    );

    println!("{}", border.dimmed());
    println!(
        "| {:>id_width$} | {:<date_width$} | {:<text_width$} |",
        format!("{:>id_width$}", "ID").bold(),
        format!("{:<date_width$}", "DATE").bold(),
        format!("{:<text_width$}", "NOTE").bold()
    );
    println!("{}", border.dimmed());
    for note in notes {
        println!(
            "| {:>id_width$} | {:<date_width$} | {:<text_width$} |",
            format!("{:>id_width$}", note.id).cyan().bold(),
            format!("{}", note.created_at.format("%Y-%m-%d %H:%M:%S")),
            note.text
        );
    }
    println!("{}", border.dimmed());
}

fn print_welcome(note_count: usize) {
    println!(
        "\n{} {}",
        "memo".cyan().bold(),
        format!("v{}", env!("CARGO_PKG_VERSION")).dimmed()
    );
    println!(
        "{} note(s) enregistrée(s). Tapez {} pour les commandes, {} pour quitter.\n",
        note_count.to_string().green(),
        "help".cyan(),
        "quit".cyan()
    );
}

fn remove_note(id_text: &str, notes: &mut Vec<Note>, storage: &Storage) -> AppResult<()> {
    let id = match id_text.parse::<usize>() {
        Ok(id) if id > 0 => id,
        _ => {
            println!(
                "{} Usage: rm <id numérique positif>",
                "Erreur:".red().bold()
            );
            return Ok(());
        }
    };

    let Some(index) = notes.iter().position(|note| note.id == id) else {
        println!("Aucune note avec l'identifiant {id}.");
        return Ok(());
    };

    let removed = notes.remove(index);
    for (index, note) in notes.iter_mut().enumerate() {
        note.id = index + 1;
    }

    if let Err(error) = storage.save(notes) {
        notes.insert(index, removed);
        for (index, note) in notes.iter_mut().enumerate() {
            note.id = index + 1;
        }
        return Err(Box::new(error));
    }

    println!("{} Note {id} supprimée.", "OK:".green().bold());
    Ok(())
}

fn print_help() {
    let commands = [
        ("add <texte>", "Ajoute une note horodatée"),
        ("list", "Affiche toutes les notes"),
        ("search <mot-clé>", "Recherche dans le texte des notes"),
        ("rm <id>", "Supprime une note et ré-indexe les identifiants"),
        ("help", "Affiche cette aide"),
        ("exit, quit", "Quitte l'application"),
    ];

    println!("{}", "Commandes".bold().underline());
    for (command, description) in commands {
        println!("  {:<34} {}", command.cyan(), description);
    }
}
