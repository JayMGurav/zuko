
// use crate::db::zuko_cli::get_question;
// use crate::config::zuko_context::ZukoContext;
// use crate::utils::bootstrap_solution;


// pub async fn execute(
//     context: &ZukoContext,
//     topic_slug: String,
// ) {
//     let question = match get_question(context, topic_slug).await {
//         Ok(q) => q,
//         Err(e) => {
//             eprintln!("Failed to get question: {}", e);
//             return;
//         }
//     };

// // based on the project root in context bootstrap the problem directory 
// // and populate the question dettails in the readme.md of the title_slug/language/readme.md file
//    let project_root = context.project_root.clone();
//    let preferred_language_list = context.preferred_language_list.clone();
//     if let Err(e) = bootstrap_solution::bootstrap_solution(
//         project_root,
//         preferred_language_list,
//         &question,
//     ) {
//         eprintln!("Failed to bootstrap solution: {}", e);
//     } else {
//         println!("Solution bootstrapped successfully for question: {}", question.title);
//     }
// }
 