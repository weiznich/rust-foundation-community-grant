use bevy::prelude::*;

fn main() {
    // This is the standard Bevy boilerplate
    // App's are initialized, systems are added to the App
    // and finally run
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(valid_system)
        // the trait bound `A: WorldQuery` is not satisfied
        // the trait `WorldQuery` is not implemented for `A`
        .add_system(missing_reference_in_query_system)
        // type mismatch resolving `for<'w, 's> <ReadFetch<B> as Fetch<'w, 's>>::Item == bool`
        // required because of the requirements on the impl of `FilterFetch` for `ReadFetch<B>`
        .add_system(missing_brackets_in_query_system)
        // NOTE: The errors for these two systems do not appear until the other problems are resolved
        // the trait bound `fn(MyResource) {malformed_system_parameter_system}: IntoSystem<(), (), _>` is not satisfied
        // required because of the requirements on the impl of `IntoSystemDescriptor<_>` for `fn(MyResource) {malformed_system_parameter_system}`
        .add_system(malformed_system_parameter_system)
        // the trait bound `for<'r, 's, 't0> fn(&'r mut bevy::prelude::Commands<'s, 't0>) {requesting_commands_as_reference_system}: IntoSystem<(), (), _>` is not satisfied
        // required because of the requirements on the impl of `IntoSystemDescriptor<_>` for `for<'r, 's, 't0> fn(&'r mut bevy::prelude::Commands<'s, 't0>) {requesting_commands_as_reference_system}`
        .add_system(requesting_commands_as_reference_system)
        .run()
}

// Entities have components
#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

#[derive(Component)]
struct C;

// Resources are global singletons
// that are accessible through the ECS
#[derive(Debug)]
struct MyResource;

// Functions that impl IntoSystem can be turned into systems
// To do so, each of their parameters must impl SystemParam
//
// We use the all_tuples! macro to work around the lack of variadics :(
fn valid_system(
    // Used for doing complex defered work
    mut commands: Commands,
    // This will fetch the unique Entity identifier,
    // and the components A and B (mutably)
    // for all entities with the components A & B & C
    mut query: Query<(Entity, &A, &mut B), With<C>>,
    my_res: Res<MyResource>,
) {
    //
    commands.spawn().insert_bundle((A, B, C));

    // Res is a smart pointer wrapper type used to fetch the resource from the ECS
    dbg!(&*my_res);

    // Queries are typically unpacked and iterated over
    for (entity, _a, mut _b) in query.iter_mut() {
        dbg!(entity);
    }
}

// The user requested A in the system, rather than &A
fn missing_reference_in_query_system(query: Query<A, With<B>>) {}

// The user needs to include both &A and &B in the first type parameter of Query
fn missing_brackets_in_query_system(query: Query<&A, &B>) {}

// The user has forgotten to add the Res wrapper around MyResource
fn malformed_system_parameter_system(my_res: MyResource) {}

// The user has invalidly requested a mutable reference to Commands,
// rather using `mut commands: Commands`
fn requesting_commands_as_reference_system(commands: &mut Commands) {}
