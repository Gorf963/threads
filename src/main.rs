mod simple;
mod pause;
mod mapred;
mod channels;

use simple::simple;
use pause::pause;
use mapred::mapred;
use channels::channels;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    simple();
    pause();
    mapred();
    pause();
    channels();
    pause();
    
}