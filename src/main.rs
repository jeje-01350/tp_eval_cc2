use eframe::egui;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;
use std::path::Path;

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

        let contenu = fs::read_to_string(&self.fichier)?;
        if contenu.is_empty() {
            return Ok(());
        }

        self.livres = serde_json::from_str(&contenu)?;
        Ok(())
    }

    // Sauvegarde les livres dans le fichier JSON
    fn sauvegarder_donnees(&self) -> Result<(), Box<dyn Error>> {
        let contenu = serde_json::to_string_pretty(&self.livres)?;
        fs::write(&self.fichier, contenu)?;
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

// Application principale
struct BibliothequeApp {
    bibliotheque: Bibliotheque,
    nouveau_livre: Livre,
    recherche_titre: String,
    recherche_isbn: String,
    onglet_actif: Onglet,
    message: String,
    message_type: MessageType,
}

#[derive(PartialEq)]
enum Onglet {
    Liste,
    Ajout,
    Recherche,
}

#[derive(PartialEq)]
enum MessageType {
    Info,
    Erreur,
    Succes,
}

impl Default for BibliothequeApp {
    fn default() -> Self {
        Self {
            bibliotheque: Bibliotheque::new("bibliotheque.json"),
            nouveau_livre: Livre {
                titre: String::new(),
                auteur: String::new(),
                isbn: String::new(),
                annee_publication: 0,
            },
            recherche_titre: String::new(),
            recherche_isbn: String::new(),
            onglet_actif: Onglet::Liste,
            message: String::new(),
            message_type: MessageType::Info,
        }
    }
}

impl eframe::App for BibliothequeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Gestion de Bibliothèque");
            ui.horizontal(|ui| {
                if ui.button("Liste des Livres").clicked() {
                    self.onglet_actif = Onglet::Liste;
                }
                if ui.button("Ajouter un Livre").clicked() {
                    self.onglet_actif = Onglet::Ajout;
                }
                if ui.button("Rechercher").clicked() {
                    self.onglet_actif = Onglet::Recherche;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.onglet_actif {
                Onglet::Liste => {
                    ui.heading("Liste des Livres");
                    if self.bibliotheque.livres.is_empty() {
                        ui.label("Aucun livre dans la bibliothèque.");
                    } else {
                        let mut index_a_supprimer = None;
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (index, livre) in self.bibliotheque.livres.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.heading(&livre.titre);
                                        ui.label(format!("Auteur: {}", livre.auteur));
                                        ui.label(format!("ISBN: {}", livre.isbn));
                                        ui.label(format!("Année: {}", livre.annee_publication));
                                    });
                                    if ui.button("Supprimer").clicked() {
                                        index_a_supprimer = Some(index);
                                    }
                                });
                                ui.separator();
                            }
                        });
                        
                        if let Some(index) = index_a_supprimer {
                            if let Err(e) = self.bibliotheque.retirer_livre(index) {
                                self.message = format!("Erreur: {}", e);
                                self.message_type = MessageType::Erreur;
                            } else {
                                self.message = "Livre supprimé avec succès.".to_string();
                                self.message_type = MessageType::Succes;
                            }
                        }
                    }
                }
                Onglet::Ajout => {
                    ui.heading("Ajouter un Nouveau Livre");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Titre: ");
                        ui.text_edit_singleline(&mut self.nouveau_livre.titre);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Auteur: ");
                        ui.text_edit_singleline(&mut self.nouveau_livre.auteur);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("ISBN: ");
                        ui.text_edit_singleline(&mut self.nouveau_livre.isbn);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Année: ");
                        ui.add(egui::DragValue::new(&mut self.nouveau_livre.annee_publication));
                    });
                    
                    ui.add_space(10.0);
                    if ui.button("Ajouter").clicked() {
                        match self.bibliotheque.ajouter_livre(self.nouveau_livre.clone()) {
                            Ok(_) => {
                                self.message = "Livre ajouté avec succès.".to_string();
                                self.message_type = MessageType::Succes;
                                self.nouveau_livre = Livre {
                                    titre: String::new(),
                                    auteur: String::new(),
                                    isbn: String::new(),
                                    annee_publication: 0,
                                };
                            }
                            Err(e) => {
                                self.message = format!("Erreur: {}", e);
                                self.message_type = MessageType::Erreur;
                            }
                        }
                    }
                }
                Onglet::Recherche => {
                    ui.heading("Rechercher un Livre");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Rechercher par titre: ");
                        ui.text_edit_singleline(&mut self.recherche_titre);
                    });
                    
                    if !self.recherche_titre.is_empty() {
                        let resultats = self.bibliotheque.rechercher_par_titre(&self.recherche_titre);
                        if resultats.is_empty() {
                            ui.label("Aucun résultat trouvé.");
                        } else {
                            for livre in resultats {
                                ui.vertical(|ui| {
                                    ui.heading(&livre.titre);
                                    ui.label(format!("Auteur: {}", livre.auteur));
                                    ui.label(format!("ISBN: {}", livre.isbn));
                                    ui.label(format!("Année: {}", livre.annee_publication));
                                });
                                ui.separator();
                            }
                        }
                    }
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Rechercher par ISBN: ");
                        ui.text_edit_singleline(&mut self.recherche_isbn);
                    });
                    
                    if !self.recherche_isbn.is_empty() {
                        if let Some(livre) = self.bibliotheque.rechercher_par_isbn(&self.recherche_isbn) {
                            ui.vertical(|ui| {
                                ui.heading(&livre.titre);
                                ui.label(format!("Auteur: {}", livre.auteur));
                                ui.label(format!("ISBN: {}", livre.isbn));
                                ui.label(format!("Année: {}", livre.annee_publication));
                            });
                        } else {
                            ui.label("Aucun livre trouvé avec cet ISBN.");
                        }
                    }
                }
            }
            
            if !self.message.is_empty() {
                ui.add_space(10.0);
                match self.message_type {
                    MessageType::Info => ui.label(&self.message),
                    MessageType::Erreur => ui.colored_label(egui::Color32::RED, &self.message),
                    MessageType::Succes => ui.colored_label(egui::Color32::GREEN, &self.message),
                };
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Gestion de Bibliothèque",
        options,
        Box::new(|_cc| Box::new(BibliothequeApp::default())),
    )
}
