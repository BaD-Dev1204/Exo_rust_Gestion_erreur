fn main() {
    /* ------------------------------- TVA ------------------------------ */

    //Le but est de créer une fonction qui permet de calculer le prix tvac a partir du prix htva et du taux tva applicable
    let prix_htva = 250.25;
    let taux_tva = 21.00;
    let est_en_promotion = true;

    match calculer_prix_tvac_moins_promotion(prix_htva, taux_tva, est_en_promotion) {
        Ok(prix_tvac) => println!("Le prix tvac est : {}", prix_tvac),
        Err(e) => println!("Erreur : {}", e),
    }


}

// Fonction pour calculer le prix TVAC
fn calculer_prix_tvac(prix_htva: f64, taux_tva: f64) -> Result<f64, String> {
    if prix_htva < 0.0 {
        return Err("Erreur : Le prix htva ne peut pas être inférieur à 0".to_string());
    } else if taux_tva < 0.0 {
        return Err("Erreur : Le taux tva ne peut pas être inférieur à 0".to_string());
    }
    // Calcul du prix TVAC
    let prix_tvac = prix_htva * (1.0 + taux_tva / 100.0);
    Ok(prix_tvac)
}

fn calculer_prix_tvac_moins_promotion(
    prix_htva: f64,
    taux_tva: f64,
    est_en_promotion: bool,
) -> Result<f64, String> {
    let mut prix_tvac = calculer_prix_tvac(prix_htva, taux_tva)?;

    if est_en_promotion {
        prix_tvac -= 20.0;
        if prix_tvac < 0.0 {
            return Err(
                "Erreur : Le prix tvac après application de la promotion ne peut pas être négatif"
                    .to_string(),
            );
        }
    }

    Ok(prix_tvac)
}
