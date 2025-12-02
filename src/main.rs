use clap::{Parser, ValueEnum};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashSet;
use std::io::BufReader;
use std::time::Duration;
use std::fs::File;
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
#[derive(Debug, Deserialize, Serialize, Clone)]
struct RedditPost {
    id: String,
    created_utc: f64,
    permalink: String,
    title: String,
}




async fn fetch_subreddit(subreddit:&String, sort:&SortOrder) -> Result<Vec<RedditPost>, Box<dyn Error>>{
    let sort_type = match sort{
        SortOrder::Hot => "hot",
        SortOrder::New => "new",
        SortOrder::Top => "top",
    };
    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_type);
   //? Apparently i need to use a User_agent bc reddit blacklists it otherwise, from what i've understood, the user_agent works like a ID badge
   //? if i use simply reqwest, it sees it as a possible attack/spam and it blacklists it
    let client = match Client::builder().user_agent("rust-redditor").build(){
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };
    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(format!("Error:Received status code: {}", resp.status()).into());
        }
    //? I get status 403 for some reason, it gets blacklisted if i do it like

    let json_data: Response = resp.json().await?;

    let posts:Vec<RedditPost> = json_data.data.children.into_iter().map(|c| c.data).collect();
   // ? println!("id: {} \n title: {} \n  permalink: {} \n created_utc: {}", posts[0].id, posts[0].title, posts[0].permalink, posts[0].created_utc);
    Ok(posts)
}


// & so i wont consume it (forgot to add it initially :) ) 

// TODO cant i make this more efficient, instead of adding only 1 post, to add every new pos that appeared in the last N seconds?

fn save_post(post:&RedditPost, filename:&str) {
        
    let mut posts: Vec<RedditPost> =  match File::open(&filename){
        Ok(f) => { // the file exists so we read its content
            let rdr = BufReader::new(f);
            match serde_json::from_reader(rdr) {
                Ok(old_posts) => old_posts,
                Err(_) => Vec::new(),  // if empty or cant read the content then create a fresh list
            }
        }

        Err(_) => { //? if i cant open it then i just create a fresh list
            println!("Error when trying to open {} ", filename);
            println!("Creating a new file...");
            Vec::new()
        }
    };

    //? add the new post
    posts.push(post.clone());

   // DELETE old file and replace it with it's updated version

    match File::create(&filename) {
        Ok(f) => { 
            serde_json::to_writer_pretty(f, &posts);
        },
        Err(_) => {
            println!("Could not update the file!");
    }
    }

}



#[tokio::main]
async fn main() {
    let args: Args =  Args::parse();
    println!("Subbreddit: {}", args.subreddit);
    println!("Sort Order: {:?}", args.sort);
    println!("Interval: {}", args.interval);
    let mut seen_posts: HashSet<String> = HashSet::new(); // here i will store the id of each post, i keep track of wether or not i have seen it
    
    loop{
        let mut new_posts = 0; 
        println!("-------------------------------------");
        match fetch_subreddit(&args.subreddit, &args.sort).await {
        Ok(posts) => { 
            for post in posts {
                if !seen_posts.contains(&post.id) {
                    println!("Title: {} \n Creation_Date: {} \n PermaLink: https://reddit.com{} \n ", post.title, post.created_utc, post.permalink);
                    println!("-------------------------------------");
                    seen_posts.insert(post.id.clone());
                    new_posts += 1;
                    save_post(&post, "feed.json");
                }
            }    
            if new_posts == 0 {
                println!("No new posts from {} seconds ago", args.interval);
            }
        },
        Err(e) => eprintln!("Failed to fetch: {}", e),
    };
    tokio::time::sleep(Duration::from_secs(args.interval)).await;
    }
}
