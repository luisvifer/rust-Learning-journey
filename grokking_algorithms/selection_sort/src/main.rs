
use std::cmp::Ordering;
#[derive(Debug,Clone,Eq)]
struct MostPlayedArtist {
    artist: String,
    play_count: i32
}

impl Ord for MostPlayedArtist{
    fn cmp(&self, other: &Self) -> Ordering {
        self.play_count.cmp(&other.play_count)
    }
}
impl PartialOrd for MostPlayedArtist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for MostPlayedArtist {
    fn eq(&self, other: &Self) -> bool {
        self.play_count == other.play_count
    }
}





impl MostPlayedArtist {
    fn new() -> Self {
        MostPlayedArtist { artist: String::from(""), play_count: 0 }
    }
}


fn main() {
    let mut artist_list = Vec::new();

   
    artist_list.push(MostPlayedArtist{
        artist: String::from("Radiohead"),
        play_count:156
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("Kishore kumar"),
        play_count:141
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("The black keys"),
        play_count:35
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("Neutral milk hotel"),
        play_count:94
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("Beck"),
        play_count:88
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("The strokes"),
        play_count:61
    });
    artist_list.push(MostPlayedArtist{
        artist: String::from("Wilco"),
        play_count:111
    });
    println!("Non Ordered list");
    for i in 0..artist_list.len() { 
        println!("{:?},",artist_list[i]); 
    }
    let order_list = sort(&mut artist_list);
    println!("Ordered list");
    for i in 0..order_list.len() { 
        println!("{:?},",order_list[i]); 
    }


}

    fn sort (list:&mut Vec<MostPlayedArtist>) -> Vec<MostPlayedArtist> {
        let mut artist_list = Vec::new();

        while ! list.is_empty() {
           let mut max_popular_index: usize = 0;
            for  i in 0..list.len() {
                if list[max_popular_index].play_count < list[i].play_count{
                    max_popular_index = i;
                }
            }
            artist_list.push(list.remove(max_popular_index));
            
        }
        artist_list
    }