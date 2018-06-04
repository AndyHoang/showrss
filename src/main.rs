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
use std::convert::From;
use std::process::Command;

impl From<Item> for Episode{

    fn from(item: Item) -> Episode{
        //println!("{:?}", item)c
        let show_name :Option<&str>= item.extensions().get("tv")
            .and_then(|m| m.get("show_name"))
            .and_then(|vec| vec.into_iter().last().and_then(|e| e.value()));
        let link = item.link().map(String::from).map_or_else(|| Vec::new(), |link| vec![link]);
        let e: Episode = Episode{title:item.title().map(String::from ), show_name: show_name.map(String::from), magnet_links: link, ..Default::default()};
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
use std::io::{self};


fn main() {
    let channel = Channel::from_url("http://showrss.info/user/105107.rss?magnets=true&namespaces=true&name=null&re=null").unwrap();
    let items = channel.into_items();
    let episodes :Vec<Episode> = items.into_iter().map(|x| Episode::from(x)).collect();
    for (i, ep) in episodes.iter().enumerate(){
        println!("{:?} {:?}", i, ep.title);
    }
    println!("select ep to stream!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let selector = input.trim().parse::<u32>().unwrap();
            let selected = episodes.get(selector as usize).unwrap();
            let link :&str= selected.magnet_links.last().unwrap();
            println!("piping to iina {:?}", link);
            let the_output = Command::new("webtorrent")
                    .args(&[link, "--iina"])
                    .output()
                    .ok()
                    .expect("failed to execute process");
            let encoded = String::from_utf8_lossy(the_output.stdout.as_slice());
            print!("{}", encoded);
        }
        Err(error) => println!("error: {}", error),
    }


}
