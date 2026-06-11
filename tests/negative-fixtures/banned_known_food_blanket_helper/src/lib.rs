use tracewake_content::schema::FixtureSchema;

pub fn blanket_seed_food_sources(fixture: &mut FixtureSchema) {
    fixture.populate_known_food_sources_for_all_actors();
}
