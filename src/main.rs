// #[macro_use]
// extern crate serde_derive;
//
// extern crate reqwest;
//
// mod lib;
//
// fn main() {
//     let mut response = reqwest::get("https://dog.ceo/api/breed/hound/images/random")
//         .expect("Failed to send request");
//     if let Ok(dog) = response.json::<lib::dog::Dog>() {
//         print!("{}", dog);
//     }
// }

// // extern crate futures;
// // extern crate telegram_bot;
// // extern crate tokio_core;

// // use futures::Stream;
// // use tokio_core::reactor::Core;
// // use telegram_bot::*;

// // fn main() {
// //     let mut core = Core::new().unwrap();

// //     let token = "613685423:AAH8LlDkhbG0arh74kgIhJUc9uJkBt52XS8";
// //     let api = Api::configure(token).build(core.handle()).unwrap();

// //     // Fetch new updates via long poll method
// //     let future = api.stream().for_each(|update| {

// //         // If the received update contains a new message...
// //         if let UpdateKind::Message(message) = update.kind {
// //             println!("{:?}", message);
// //             match message.kind {
// //                 _ => {
// //                     api.spawn(message.text_reply(
// //                         format!("Hi, {}! I don't understand you", message.from.first_name))
// //                     );
// //                 }
// //             }

// //             //if let MessageKind::Text {ref data, ..} = message.kind {
// //             //    // Print received text message to stdout.
// //             //    println!("<{}>: {}", message.from.first_name, data);
// //             //    // Answer message with "Hi".
// //             //    api.spawn(message.text_reply(
// //             //        format!("Hi, {}! You just wrote '{}'", message.from.first_name, data.chars().rev().collect::<String>())
// //             //    ));
// //             //}
// //         }

// //         Ok(())
// //     });

// //     core.run(future).unwrap();
// // }
fn main() {
    let mut v = vec![1, 2, 3];

    {
        let v2 = &v;
        for i in v2 {
            println!("{}", i);
        }
    }
}
