use std::io;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::error::Error;

// Structure de base pour un livre
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Livre {
    titre: String,
    auteur: String,
    isbn: String,
    annee_publication: u32,
}

// Gestion de la bibliothèque et de la persistance
struct Bibliotheque {
    livres: Vec<Livre>,
    fichier: String,
}

impl Bibliotheque {
    // Crée une nouvelle bibliothèque ou charge une existante
    fn new(fichier: &str) -> Self {
        let mut bibliotheque = Bibliotheque {
            livres: Vec::new(),
            fichier: fichier.to_string(),
        };
        bibliotheque.charger_donnees().unwrap_or_else(|_| {
            println!("Nouvelle bibliothèque créée.");
        });
        bibliotheque
    }

    // Charge les livres depuis le fichier JSON
    fn charger_donnees(&mut self) -> Result<(), Box<dyn Error>> {
        if !Path::new(&self.fichier).exists() {
            return Ok(());
        }

        let contenu = std::fs::read_to_string(&self.fichier)?;
        if contenu.is_empty() {
            return Ok(());
        }

        self.livres = serde_json::from_str(&contenu)?;
        Ok(())
    }

    // Sauvegarde les livres dans le fichier JSON
    fn sauvegarder_donnees(&self) -> Result<(), Box<dyn Error>> {
        let contenu = serde_json::to_string_pretty(&self.livres)?;
        std::fs::write(&self.fichier, contenu)?;
        Ok(())
    }

    // Ajoute un livre et sauvegarde
    fn ajouter_livre(&mut self, livre: Livre) -> Result<(), Box<dyn Error>> {
        self.livres.push(livre);
        self.sauvegarder_donnees()?;
        Ok(())
    }

    // Recherche par titre (insensible à la casse)
    fn rechercher_par_titre(&self, titre: &str) -> Vec<&Livre> {
        self.livres
            .iter()
            .filter(|livre| livre.titre.to_lowercase().contains(&titre.to_lowercase()))
            .collect()
    }

    // Recherche par ISBN (recherche exacte)
    fn rechercher_par_isbn(&self, isbn: &str) -> Option<&Livre> {
        self.livres
            .iter()
            .find(|livre| livre.isbn.to_lowercase() == isbn.to_lowercase())
    }

    // Affiche la liste des livres
    fn afficher_tous_les_livres(&self) {
        if self.livres.is_empty() {
            println!("La bibliothèque est vide.");
            return;
        }

        println!("\nListe des livres :");
        for (index, livre) in self.livres.iter().enumerate() {
            println!("{}. {} de {} (ISBN: {}, Année: {})",
                index + 1,
                livre.titre,
                livre.auteur,
                livre.isbn,
                livre.annee_publication
            );
        }
    }

    // Supprime un livre par son index
    fn retirer_livre(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.livres.len() {
            return Err("Index invalide".into());
        }
        self.livres.remove(index);
        self.sauvegarder_donnees()?;
        Ok(())
    }
}

// Point d'entrée du programme
fn main() {
    let mut bibliotheque = Bibliotheque::new("bibliotheque.json");
    
    loop {
        println!("\n=== Menu Principal ===");
        println!("1. Ajouter un livre");
        println!("2. Rechercher un livre");
        println!("3. Afficher tous les livres");
        println!("4. Retirer un livre");
        println!("5. Quitter");
        print!("\nChoix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide.");
                continue;
            }
        };

        match choix {
            1 => {
                println!("\nNouveau livre :");
                let livre = saisir_livre();
                match bibliotheque.ajouter_livre(livre) {
                    Ok(_) => println!("Livre ajouté !"),
                    Err(e) => println!("Erreur : {}", e),
                }
            }
            2 => {
                println!("\nRecherche :");
                println!("1. Par titre");
                println!("2. Par ISBN");
                print!("Choix : ");
                io::stdout().flush().unwrap();
                
                let mut type_recherche = String::new();
                io::stdin().read_line(&mut type_recherche).unwrap();
                
                match type_recherche.trim().parse::<u32>() {
                    Ok(1) => {
                        print!("Titre : ");
                        io::stdout().flush().unwrap();
                        let mut titre = String::new();
                        io::stdin().read_line(&mut titre).unwrap();
                        let resultats = bibliotheque.rechercher_par_titre(titre.trim());
                        
                        if resultats.is_empty() {
                            println!("Aucun résultat.");
                        } else {
                            println!("\nRésultats :");
                            for livre in resultats {
                                println!("- {} de {} (ISBN: {}, Année: {})",
                                    livre.titre,
                                    livre.auteur,
                                    livre.isbn,
                                    livre.annee_publication
                                );
                            }
                        }
                    },
                    Ok(2) => {
                        print!("ISBN : ");
                        io::stdout().flush().unwrap();
                        let mut isbn = String::new();
                        io::stdin().read_line(&mut isbn).unwrap();
                        
                        match bibliotheque.rechercher_par_isbn(isbn.trim()) {
                            Some(livre) => {
                                println!("\nLivre trouvé :");
                                println!("- {} - {} (ISBN: {}, Année: {})",
                                    livre.titre,
                                    livre.auteur,
                                    livre.isbn,
                                    livre.annee_publication
                                );
                            },
                            None => println!("ISBN non trouvé."),
                        }
                    },
                    _ => println!("Option invalide."),
                }
            }
            3 => {
                bibliotheque.afficher_tous_les_livres();
            }
            4 => {
                bibliotheque.afficher_tous_les_livres();
                if !bibliotheque.livres.is_empty() {
                    print!("\nNuméro du livre à retirer : ");
                    io::stdout().flush().unwrap();
                    let mut index = String::new();
                    io::stdin().read_line(&mut index).unwrap();
                    let index: usize = match index.trim().parse::<usize>() {
                        Ok(num) => num - 1,
                        Err(_) => {
                            println!("Numéro invalide.");
                            continue;
                        }
                    };

                    match bibliotheque.retirer_livre(index) {
                        Ok(_) => println!("Livre retiré !"),
                        Err(e) => println!("Erreur : {}", e),
                    }
                }
            }
            5 => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Option invalide."),
        }
    }
}

// Saisie des informations d'un livre
fn saisir_livre() -> Livre {
    print!("Titre : ");
    io::stdout().flush().unwrap();
    let mut titre = String::new();
    io::stdin().read_line(&mut titre).unwrap();

    print!("Auteur : ");
    io::stdout().flush().unwrap();
    let mut auteur = String::new();
    io::stdin().read_line(&mut auteur).unwrap();

    print!("ISBN : ");
    io::stdout().flush().unwrap();
    let mut isbn = String::new();
    io::stdin().read_line(&mut isbn).unwrap();

    print!("Année : ");
    io::stdout().flush().unwrap();
    let mut annee = String::new();
    io::stdin().read_line(&mut annee).unwrap();
    let annee: u32 = annee.trim().parse().unwrap_or(0);

    Livre {
        titre: titre.trim().to_string(),
        auteur: auteur.trim().to_string(),
        isbn: isbn.trim().to_string(),
        annee_publication: annee,
    }
}
