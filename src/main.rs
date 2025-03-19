// Fonction pour la recherche dichotomique
fn recherche_dichotomique(arr: &[i32], cible: i32) -> Option<usize> {
    // L'indice le plus à gauche c'est-à-dire l'indice 0
    let mut gauche: isize = 0;

    // L'indice de la fin du tableau
    let mut droite: isize = arr.len() as isize - 1;

    /*
        - On va éfféctuer une boucle: tant que l'indice gauche est inférieur ou égale à l'indice de droite
        -> On calcule l'indice milieu (l'indice moyenne du table => moyenne = (indiceGauche + indiceDroite) % 2)
    */
    while gauche <= droite {
        // Calcul du milieu_tableau
        let milieu_tableau: isize = (gauche + droite) / 2;

        // Conversion "isize" -> "usize"
        let milieu_tableau = milieu_tableau as usize;

        // On éffectue une condition pour trouver l'indice de notre élement
        if arr[milieu_tableau] == cible {
            // Si l'élément est trouvé, on retourne son indice
            return Some(milieu_tableau);
        } else if arr[milieu_tableau] < cible {
            // Rechercher dans la moitié droite du tableau
            gauche = milieu_tableau as isize + 1;
        } else {
            // Rechercher dans la moitié gauche du tableau
            droite = milieu_tableau as isize - 1;
        }
    }
    // Si l'élément n'est pas trouvé, retourner None
    None
}

fn main() {
    // On déclare notre tableau
    let tableau: [i32; 11] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21];

    // On désign l'element à rechercher
    let cible: i32 = 13;

    /*
    On éffectue la recherche et on retourn le resultat en utilisant la condition suivante:
    -> On appelle la fonction "recherche_dichitomique"
    */
    match recherche_dichotomique(&tableau, cible) {
        // Si l'élement est existe et trouvé dans le tableau
        Some(index) => println!("L'élément {} a été trouvé à l'indice {}", cible, index),

        // Si on a pas trouvé l'élement du tableau
        None => println!("L'élément {} n'a pas été trouvé dans le tableau", cible),
    }
}
