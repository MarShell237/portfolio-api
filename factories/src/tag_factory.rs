use entities::tags as tag;
use fake::faker::color::raw::*;
use fake::faker::lorem::raw::*;
use fake::locales::FR_FR;
use fake::{Dummy, Fake, Faker};
use sea_orm::ActiveValue::{NotSet, Set};

#[derive(Debug, Dummy)]
pub struct TagFactoryDTO {
    #[dummy(faker = "Words(FR_FR, 1..5)")]
    pub words: Vec<String>,

    #[dummy(faker = "HexColor(FR_FR)")]
    pub color: String,

    #[dummy(faker = "Sentences(FR_FR, 1..3)")]
    pub description_sentences: Vec<String>,
}

// 2. Une structure simple sans traits complexes
pub struct TagFactory;

impl TagFactory {
    /// Génère un ActiveModel prêt pour la base de données
    pub fn create() -> tag::ActiveModel {
        // Génère des données aléatoires via le DTO
        let dto: TagFactoryDTO = Faker.fake();

        let name = dto.words.join(" ");
        let slug = name
            .to_lowercase()
            .replace(['\'', '"'], "")
            .replace(' ', "-");
        let description = dto.description_sentences.join(" ");

        tag::ActiveModel {
            id: NotSet,
            name: Set(name),
            slug: Set(slug),
            color: Set(dto.color),
            description: Set(description),
            icon: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}
