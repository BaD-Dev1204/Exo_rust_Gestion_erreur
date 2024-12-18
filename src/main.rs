fn main() {
    /* ------------------------------- TVA ------------------------------ */

    //Le but est de créer une fonction qui permet de calculer le prix tvac a partir du prix htva et du taux tva applicable
    //Ajout du montant de la promotion
    let prix_htva = 120.00;
    let taux_tva = 21.00;
    let promotion_applicable = 15.00;

    match calculer_prix_tvac_moins_promotion(prix_htva, taux_tva, promotion_applicable) {
        Ok(prix_tvac) => println!("Le prix tvac est : {}", prix_tvac),
        Err(e) => println!("{}", e),
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

// Fonction pour calculer le prix TVAC moins la promotion
fn calculer_prix_tvac_moins_promotion(
    prix_htva: f64,
    taux_tva: f64,
    promotion_applicable: f64,
) -> Result<f64, String> {
    if promotion_applicable < 0.0 {
        return Err("Le montant de la promotion ne peut pas être négatif.".to_string());
    }

    let prix_tvac = calculer_prix_tvac(prix_htva, taux_tva)?;

    let prix_apres_promotion = prix_tvac - promotion_applicable;
    if prix_apres_promotion < 0.0 {
        return Err("Le prix TVAC après promotion ne peut pas être négatif.".to_string());
    }

    Ok(prix_apres_promotion)
}
