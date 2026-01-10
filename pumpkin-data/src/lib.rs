#![allow(unused_imports)]

#[rustfmt::skip]
#[path = "generated/item.rs"]
pub mod item;

#[rustfmt::skip]
#[path = "generated/packet.rs"]
pub mod packet;

#[rustfmt::skip]
#[path = "generated/screen.rs"]
pub mod screen;

#[rustfmt::skip]
#[path = "generated/particle.rs"]
pub mod particle;

#[rustfmt::skip]
#[path = "generated/sound_category.rs"]
mod sound_category;

#[rustfmt::skip]
#[path = "generated/sound.rs"]
mod sound_enum;

#[rustfmt::skip]
#[path = "generated/recipes.rs"]
pub mod recipes;

#[rustfmt::skip]
#[path = "generated/data_component.rs"]
pub mod data_component;
pub mod data_component_impl;

#[rustfmt::skip]
#[path = "generated/attributes.rs"]
pub mod attributes;

pub mod sound {
    pub use crate::sound_category::*;
    pub use crate::sound_enum::*;
}

#[rustfmt::skip]
#[path = "generated/noise_parameter.rs"]
pub mod noise_parameter;

#[rustfmt::skip]
#[path = "generated/biome.rs"]
pub mod biome;

#[rustfmt::skip]
#[path = "generated/chunk_status.rs"]
pub mod chunk_status;

pub mod chunk {
    pub use super::biome::*;
    pub use super::chunk_status::ChunkStatus;
    pub use super::noise_parameter::*;
}

#[rustfmt::skip]
#[path = "generated/game_event.rs"]
pub mod game_event;

#[rustfmt::skip]
#[path ="generated/game_rules.rs"]
pub mod game_rules;

#[rustfmt::skip]
#[path = "generated/entity_pose.rs"]
mod entity_pose;

#[rustfmt::skip]
#[path = "generated/entity_status.rs"]
mod entity_status;

#[rustfmt::skip]
#[path = "generated/entity_type.rs"]
mod entity_type;

#[rustfmt::skip]
#[path = "generated/spawn_egg.rs"]
mod spawn_egg;

#[rustfmt::skip]
#[path = "generated/dimension.rs"]
pub mod dimension;

#[rustfmt::skip]
#[path = "generated/enchantment.rs"]
mod enchantment;
pub use enchantment::*;

pub mod entity {
    pub use super::entity_pose::*;
    pub use super::entity_status::*;
    pub use super::entity_type::*;
    pub use super::spawn_egg::*;
}

#[rustfmt::skip]
#[path = "generated/world_event.rs"]
mod world_event;

#[rustfmt::skip]
#[path = "generated/message_type.rs"]
mod message_type;

pub mod world {
    pub use super::message_type::*;
    pub use super::world_event::*;
}

#[rustfmt::skip]
#[path = "generated/scoreboard_slot.rs"]
pub mod scoreboard;

#[rustfmt::skip]
#[path = "generated/damage_type.rs"]
pub mod damage;

#[rustfmt::skip]
#[path = "generated/fluid.rs"]
pub mod fluid;

#[rustfmt::skip]
#[path = "generated/block.rs"]
pub mod block_properties;

#[rustfmt::skip]
#[path = "generated/tag.rs"]
pub mod tag;

#[rustfmt::skip]
#[path = "generated/noise_router.rs"]
pub mod noise_router;

#[rustfmt::skip]
#[path = "generated/composter_increase_chance.rs"]
pub mod composter_increase_chance;

#[rustfmt::skip]
#[path = "generated/flower_pot_transformations.rs"]
pub mod flower_pot_transformations;

#[rustfmt::skip]
#[path = "generated/fuels.rs"]
pub mod fuels;

#[rustfmt::skip]
#[path = "generated/effect.rs"]
pub mod effect;

#[rustfmt::skip]
#[path = "generated/potion.rs"]
pub mod potion;

#[rustfmt::skip]
#[path = "generated/potion_brewing.rs"]
pub mod potion_brewing;

#[rustfmt::skip]
#[path = "generated/recipe_remainder.rs"]
pub mod recipe_remainder;

mod block_direction;
pub mod block_state;
mod blocks;
mod collision_shape;

pub use block_direction::BlockDirection;
pub use block_direction::FacingExt;
pub use block_direction::HorizontalFacingExt;
pub use block_state::BlockState;
pub use block_state::BlockStateRef;
pub use blocks::Block;
pub use collision_shape::CollisionShape;

#[cfg(test)]
mod tests {
    use super::data_component::DataComponent;

    /// Test that PHF map lookup works for known data components
    #[test]
    fn test_data_component_lookup() {
        // Test a few known data components
        assert!(DataComponent::try_from_name("minecraft:custom_data").is_some());
        assert!(DataComponent::try_from_name("minecraft:max_stack_size").is_some());
        assert!(DataComponent::try_from_name("minecraft:damage").is_some());

        // Test that invalid names return None
        assert!(DataComponent::try_from_name("invalid:component").is_none());
        assert!(DataComponent::try_from_name("").is_none());
    }

    /// Test that round-trip conversion works (name -> enum -> name)
    #[test]
    fn test_data_component_roundtrip() {
        let name = "minecraft:custom_data";
        let component = DataComponent::try_from_name(name).expect("Component should exist");
        assert_eq!(component.to_name(), name);
    }

    /// Test that ID conversion works
    #[test]
    fn test_data_component_id_conversion() {
        // Get a component by name
        let component =
            DataComponent::try_from_name("minecraft:custom_data").expect("Component should exist");

        // Convert to ID and back
        let id = component.to_id();
        let component_from_id =
            DataComponent::try_from_id(id).expect("Component should exist by ID");

        assert_eq!(component, component_from_id);
    }
}
