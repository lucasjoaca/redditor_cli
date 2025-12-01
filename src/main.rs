use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::error::Error;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{


    //? Positional argument!!!
    #[arg(index = 1)] //Tells the program that this is a mandatory argument
    subreddit: String,


    //? short -> generates the -s flag
    //? long -> generates the --sort flag
    #[arg(short = 's', long = "sort", default_value = "hot")]
    sort: SortOrder,

    #[arg(short = 't', long = "time", default_value_t = 60)]
    interval: u64,

}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SortOrder{
    Hot, //? This is the default one
    New,
    Top,
}

#[derive(Debug, Deserialize)]
struct Response{
    data: Children,
}
#[derive(Debug, Deserialize)]
struct Children{
    children: Vec<PostContainer>,
}

#[derive(Debug, Deserialize)]
struct PostContainer{
    data: RedditPost,
}
#[derive(Debug, Deserialize)]
struct RedditPost {
    id: String,
    created_utc: f64,
    permalink: String,
    title: String,
}




fn fetch_subreddit(subreddit:String, sort:SortOrder) -> Vec<RedditPost>{
    let sort_type = match sort{
        SortOrder::Hot => "hot",
        SortOrder::New => "new",
        SortOrder::Top => "top",
    };
    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_type);

}


fn main() {
    let args: Args =  Args::parse();
    println!("Subbreddit: {}", args.subreddit);
    println!("Sort Order: {:?}", args.sort);
    println!("Interval: {}", args.interval);
}
