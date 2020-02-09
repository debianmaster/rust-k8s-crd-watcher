


extern crate serde;
extern crate serde_json;
extern crate ureq;
use serde::{Deserialize, Serialize};
use std::env;

use ureq::json;

use kube::{
    api::{Informer, Object, RawApi, WatchEvent},
    client::APIClient,
    config,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub title: String,
    pub authors: Option<Vec<String>>,
}

// This is a convenience alias that describes the object we get from Kubernetes
type KubeBook = Object<serde_json::Value, serde_json::Value>;

fn main()  {
    // Load the kubeconfig file.
    let kubeconfig = config::load_kube_config()
        .or_else(|_| config::incluster_config())
        .expect("kubeconfig failed to load");
    
    // Create a new client
    let client = APIClient::new(kubeconfig);

    // Set a namespace. We're just hard-coding for now.
    let namespace = "default";

    // Describe the CRD we're working with.
    // This is basically the fields from our CRD definition.
    let resource = RawApi::customResource("books")
        .group("example.technosophos.com")
        .within(&namespace);
    
    // Create our informer and start listening.
    let informer = Informer::raw(client, resource)
        .init()
        .expect("informer init failed");
    loop {
        informer.poll().expect("informer poll failed");

        // Now we just do something each time a new book event is triggered.
        while let Some(event) = informer.pop() {
            handle(event);
        }
    }
}


fn handle(event: WatchEvent<KubeBook>) {
    println!("event a book ");
    // This will receive events each time something
    match event {
        WatchEvent::Added(book) => {     
            println!("Added a book {}", book.metadata.name);
            update_db(book,"/register".to_string());
        },
        WatchEvent::Deleted(book) => {
            println!("Deleted a book {}", book.metadata.name);
            update_db(book,"/deregister".to_string());
        },
        WatchEvent::Modified(book) =>{
            println!("modified a book {}", book.metadata.name);
            update_db(book,"/register".to_string());
        },
        _ => {
            println!("another event");
        },
    }
}
fn update_db(book: KubeBook,update_type: String){
    //"https://en600nj5ohlgq.x.pipedream.net"
    let mut uri:String  = env::var("webhook").unwrap().to_string();
    uri.push_str(&update_type);
    let resp = ureq::post(&uri)
    .set("Content-Type", "application/json")
    .set("update-type","added")
    .send_json(json!(book));
    if resp.ok() {
        println!("ok")
    }
    else{
        println!("not ok")
    }
    println!("done adding a book {}", book.metadata.name);
}