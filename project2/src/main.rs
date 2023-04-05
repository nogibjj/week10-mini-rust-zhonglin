use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let perfumes = [
        "Chanel No. 5",
        "Joy by Jean Patou",
        "Shalimar by Guerlain",
        "Opium by Yves Saint Laurent",
        "Miss Dior by Christian Dior",
    ];

    let descriptions = [
        "a symphony of flowers and aldehydes",
        "a floral explosion",
        "a sensual and sophisticated fragrance",
        "an oriental and exotic scent",
        "a timeless classic with a modern twist",
    ];

    let mut rng = thread_rng();
    let perfume = perfumes.choose(&mut rng).unwrap();
    let description = descriptions.choose(&mut rng).unwrap();

    println!(
        "Indulge in the luxurious scent of {} - {}",
        perfume, description
    );
}
