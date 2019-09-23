pub mod space_ship;
pub mod bullet;
pub mod asteroid;
pub mod all_entities;

// Every entity of the game
pub enum Entity {
    Spaceship(space_ship::Spaceship),
    Asteroid(asteroid::Asteroid),
    Bullet(bullet::Bullet),
}

// Entity index
pub type EntityIndex = usize;
