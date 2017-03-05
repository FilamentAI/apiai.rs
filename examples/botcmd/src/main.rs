extern crate apiai;

use apiai::client::{ApiAIClient,ApiRequest};


fn main() {
    use std::io::{self, Write};
    println!("API.AI Demo (Press CTRL+C to quit)...");

    let my_token = String::from("ce2f54f8eb444d74af85f89e30ef2fd3");

    let client = ApiAIClient{
        access_token: my_token,
        ..Default::default()
    };
    let mut input = String::new();
    loop {
        print!("<<< ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {


                let req = ApiRequest{
                    query: Option::Some(input.clone()),
                    ..Default::default()
                };

                let response = client.query(req).unwrap();

                println!(">>> {}", response.result.fulfillment.speech);

            }
            Err(error) => println!("error: {}", error),
        }
    }

}
