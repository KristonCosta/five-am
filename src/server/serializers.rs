use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    builder: Vec<EntityBuilder>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EntityBuilder {
    id: String,
    renderable: Renderable,
    name: String,
    priority: Option<Priority>,
    display_cabinet: Option<bool>,
    tradeable: Option<bool>,
    inventory: Option<Inventory>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Inventory {
    contents: Vec<String>,
    capacity: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Renderable {
    pub glyph: GlyphData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlyphData {
    pub ch: char,
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub render_order: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Priority {
    pub value: u8,
}

pub mod entity_factory {
    use super::{Data, EntityBuilder};
    use crate::color::Color;
    use crate::{component, geom::Point, glyph::Glyph};
    use legion::prelude::{CommandBuffer, Entity};
    use serde_json::from_str;
    use std::collections::HashMap;
    use std::fs;

    pub struct EntityFactory {
        registry: HashMap<String, EntityBuilder>,
    }

    impl EntityFactory {
        pub fn load() -> Self {
            //  let file_contents = load_file("data/entities.json").await.expect("Couldn't find entity factory file");
            let raw_string = fs::read_to_string("entities.json").expect("Couldn't load json"); // std::str::from_utf8(&file_contents).expect("Couldn't get raw string from factory file");
            let mut data: Data = from_str(&raw_string).expect("Invalid entity factory file");
            let mut registry = HashMap::new();
            for factory in data.builder.drain(..) {
                registry.insert(factory.id.clone(), factory);
            }
            Self { registry }
        }

        fn deserialize_color(color: Option<String>) -> Option<Color> {
            match color {
                None => None,
                Some(color) => None,
            }
        }

        pub fn build(
            &self,
            id: &str,
            position: Option<Point>,
            buffer: &mut CommandBuffer,
        ) -> Entity {
            let options = self
                .registry
                .get(id)
                .expect(&format!("Could not find {:?}", id));
            let builder = buffer.start_entity();
            let builder = builder
                .with_component(component::Renderable {
                    glyph: Glyph {
                        ch: options.renderable.glyph.ch,
                        foreground: Self::deserialize_color(
                            options.renderable.glyph.foreground.clone(),
                        ),
                        background: Self::deserialize_color(
                            options.renderable.glyph.background.clone(),
                        ),
                        render_order: options.renderable.glyph.render_order,
                    },
                })
                .with_component(component::Name {
                    name: options.name.clone(),
                })
                .with_component(component::TileBlocker);

            let entity = builder.build();

            if let Some(position) = &position {
                buffer.add_component(
                    entity,
                    component::Position {
                        x: position.x,
                        y: position.y,
                    },
                );
            }
            if let Some(priority) = &options.priority {
                buffer.add_component(
                    entity,
                    component::Priority {
                        value: priority.value,
                    },
                )
            }

            let mut has_inventory = false;
            if let Some(inventory) = &options.inventory {
                has_inventory = true;
                let mut contents = vec![];
                for item in &inventory.contents {
                    let item_config = self
                        .registry
                        .get(item)
                        .expect(&format!("Could not find {:?}", id));
                    if let Some(_) = &item_config.inventory {
                        panic!("Tried to store an inventory entitiy in another inventory. Base: {:?} Item: {:?}");
                    }
                    contents.push(self.build(item, None, buffer));
                }

                buffer.add_component(
                    entity,
                    component::Inventory {
                        contents: contents,
                        capacity: inventory.capacity,
                    },
                )
            }

            if let Some(tradeable) = options.tradeable {
                if tradeable {
                    buffer.add_component(
                        entity,
                        component::Tradeable {
                            request: None
                        }
                    )
                }
            }

            if let Some(display) = options.display_cabinet {
                if display {
                    if !has_inventory {
                        buffer.add_component(
                            entity,
                            component::Inventory {
                                contents: vec![],
                                capacity: 1,
                            },
                        );
                    }
                    buffer.add_tag(entity, component::DisplayCabinet);
                }
            }

            entity
        }
    }
}
