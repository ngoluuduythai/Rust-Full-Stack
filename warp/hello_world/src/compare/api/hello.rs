// https://stackoverflow.com/questions/29068716/how-do-you-use-a-macro-from-inside-its-own-crate
// Macros defined this way become gloally available with #[macro_export]

// How to remove this crate prefixes?
// Is it worth spending time to make this files just to modulize a little part?
// Stop doing this?
#[macro_export]
macro_rules! hello {
    () => {
        crate::routes::hello_route::hello()
        .and_then(crate::handlers::hello_handler::hello)
        // hello_route::hello()
        // .and_then(hello_handler::hello)
    }
}

// It is equal to save this inside main.rs file
// See compare/main_with_macro.rs

// use self::{
//     routes::{
//         hello_route,
//     },
//     handlers::{
//         hello_handler
//     },

// };

// #[macro_export]
// macro_rules! hello {
//     () => {
//         hello_route::hello()
//         .and_then(hello_handler::hello)
//     }
// }