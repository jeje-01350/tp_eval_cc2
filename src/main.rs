use std::io;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Livre {
    titre: String,
    auteur: String,
    isbn: String,
    annee_publication: u32,
}

struct Bibliotheque {
    livres: Vec<Livre>,
    fichier: String,
}

impl Bibliotheque {
    fn new(fichier: &str) -> Self {
        let mut bibliotheque = Bibliotheque {
            livres: Vec::new(),
            fichier: fichier.to_string(),
        };
        bibliotheque.charger_donnees().unwrap_or_else(|_| {
            println!("Création d'une nouvelle bibliothèque.");
        });
        bibliotheque
    }

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

    fn sauvegarder_donnees(&self) -> Result<(), Box<dyn Error>> {
        let contenu = serde_json::to_string_pretty(&self.livres)?;
        std::fs::write(&self.fichier, contenu)?;
        Ok(())
    }

    fn ajouter_livre(&mut self, livre: Livre) -> Result<(), Box<dyn Error>> {
        self.livres.push(livre);
        self.sauvegarder_donnees()?;
        Ok(())
    }

    fn rechercher_par_titre(&self, titre: &str) -> Vec<&Livre> {
        self.livres
            .iter()
            .filter(|livre| livre.titre.to_lowercase().contains(&titre.to_lowercase()))
            .collect()
    }

    fn rechercher_par_isbn(&self, isbn: &str) -> Option<&Livre> {
        self.livres
            .iter()
            .find(|livre| livre.isbn.to_lowercase() == isbn.to_lowercase())
    }

    fn afficher_tous_les_livres(&self) {
        if self.livres.is_empty() {
            println!("La bibliothèque est vide.");
            return;
        }

        println!("\nListe des livres dans la bibliothèque :");
        for (index, livre) in self.livres.iter().enumerate() {
            println!("{}. {} - {} (ISBN: {}, Année: {})",
                index + 1,
                livre.titre,
                livre.auteur,
                livre.isbn,
                livre.annee_publication
            );
        }
    }

    fn retirer_livre(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.livres.len() {
            return Err("Index invalide".into());
        }
        self.livres.remove(index);
        self.sauvegarder_donnees()?;
        Ok(())
    }
}

fn main() {
    let mut bibliotheque = Bibliotheque::new("bibliotheque.json");
    
    loop {
        println!("\n=== Menu de Gestion de la Bibliothèque ===");
        println!("1. Ajouter un livre");
        println!("2. Rechercher un livre");
        println!("3. Afficher tous les livres");
        println!("4. Retirer un livre");
        println!("5. Quitter");
        print!("\nVotre choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        match choix {
            1 => {
                println!("\nAjout d'un nouveau livre :");
                let livre = saisir_livre();
                match bibliotheque.ajouter_livre(livre) {
                    Ok(_) => println!("Livre ajouté avec succès !"),
                    Err(e) => println!("Erreur lors de l'ajout du livre : {}", e),
                }
            }
            2 => {
                println!("\nRecherche de livre");
                println!("1. Rechercher par titre");
                println!("2. Rechercher par ISBN");
                print!("Votre choix : ");
                io::stdout().flush().unwrap();
                
                let mut type_recherche = String::new();
                io::stdin().read_line(&mut type_recherche).unwrap();
                
                match type_recherche.trim().parse::<u32>() {
                    Ok(1) => {
                        print!("Entrez le titre à rechercher : ");
                        io::stdout().flush().unwrap();
                        let mut titre = String::new();
                        io::stdin().read_line(&mut titre).unwrap();
                        let resultats = bibliotheque.rechercher_par_titre(titre.trim());
                        
                        if resultats.is_empty() {
                            println!("Aucun livre trouvé.");
                        } else {
                            println!("\nRésultats de la recherche :");
                            for livre in resultats {
                                println!("- {} - {} (ISBN: {}, Année: {})",
                                    livre.titre,
                                    livre.auteur,
                                    livre.isbn,
                                    livre.annee_publication
                                );
                            }
                        }
                    },
                    Ok(2) => {
                        print!("Entrez l'ISBN à rechercher : ");
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
                            None => println!("Aucun livre trouvé avec cet ISBN."),
                        }
                    },
                    _ => println!("Option de recherche invalide."),
                }
            }
            3 => {
                bibliotheque.afficher_tous_les_livres();
            }
            4 => {
                bibliotheque.afficher_tous_les_livres();
                if !bibliotheque.livres.is_empty() {
                    print!("\nEntrez le numéro du livre à retirer : ");
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
                        Ok(_) => println!("Livre retiré avec succès !"),
                        Err(e) => println!("Erreur : {}", e),
                    }
                }
            }
            5 => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide. Veuillez réessayer."),
        }
    }
}

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

    print!("Année de publication : ");
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
