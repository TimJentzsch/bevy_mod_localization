use bevy::ecs::system::Resource;

use super::language_id::LanguageId;

#[derive(Debug, Default, PartialEq, Eq, Clone, Resource)]
pub struct LocaleChain(pub Vec<LanguageId>);
