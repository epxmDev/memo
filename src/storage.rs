use crate::model::Note;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Storage {
    directory: PathBuf,
    notes_path: PathBuf,
}

impl Storage {
    pub fn new() -> io::Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "impossible de déterminer le dossier personnel de l'utilisateur",
            )
        })?;
        let directory = home.join(".local/share/memo");
        fs::create_dir_all(&directory).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "impossible de créer le dossier de données {}: {error}",
                    directory.display()
                ),
            )
        })?;

        Ok(Self {
            notes_path: directory.join("notes.json"),
            directory,
        })
    }

    pub fn history_path(&self) -> PathBuf {
        self.directory.join("history.txt")
    }

    pub fn load(&self) -> io::Result<Vec<Note>> {
        let contents = match fs::read_to_string(&self.notes_path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!("impossible de lire {}: {error}", self.notes_path.display()),
                ));
            }
        };

        serde_json::from_str(&contents).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "impossible de décoder le fichier {}: {error}",
                    self.notes_path.display()
                ),
            )
        })
    }

    pub fn save(&self, notes: &[Note]) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(notes).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("impossible d'encoder les notes en JSON: {error}"),
            )
        })?;
        let temporary_path = self.notes_path.with_extension("json.tmp");

        fs::write(&temporary_path, contents).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "impossible d'écrire le fichier temporaire {}: {error}",
                    temporary_path.display()
                ),
            )
        })?;
        fs::rename(&temporary_path, &self.notes_path).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "impossible de remplacer {}: {error}",
                    self.notes_path.display()
                ),
            )
        })
    }
}
