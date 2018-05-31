#![recursion_limit = "1024"]
extern crate rss;
#[macro_use]
extern crate error_chain;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}

}


error_chain!{

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        //Reqwest(reqwest::Error);
        Rss(rss::Error);
    }
}
use rss::{Channel, Item};
use rss::extension::{ExtensionMap};
use std::convert::From;


impl From<Item> for Episode{

    fn from(item: Item) -> Episode{
        //println!("{:?}", item)c
        let show_name :Option<&str>= item.extensions().get("tv")
            .and_then(|m| m.get("show_name"))
            .and_then(|vec| vec.into_iter().last().and_then(|e| e.value()));
        let link = item.link().map(String::from).map_or_else(|| Vec::new(), |link| vec![link]);
        let e: Episode = Episode{title:item.title().map(String::from ), show_name: show_name.map(String::from), magnet_links: link, ..Default::default()};
        println!("{:?}", e);
        e
    }
}
#[derive(Default, Debug)]
struct Episode{
    show_name: Option<String>,
    title: Option<String>,
    magnet_links: Vec<String>,
    http_links: Vec<String>
}



fn main() {
    let channel = Channel::from_url("http://showrss.info/user/105107.rss?magnets=true&namespaces=true&name=null&re=null").unwrap();
    let items = channel.into_items();
    let episodes :Vec<Episode> = items.into_iter().map(|x| Episode::from(x)).collect();

}
