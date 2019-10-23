mod simple;
mod pause;
mod mapred;
mod channels;
mod jim_try;

use simple::simple;
use pause::pause;
use mapred::mapred;
use channels::channels;
use jim_try::jim_try;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    simple();
    pause();
    mapred();
    pause();
    channels();
    pause();
    println!("Jim Try");
    jim_try();  
}