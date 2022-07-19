use mlib::translate::translate;
use mlib::music::mp3::mp3;
use mlib::music::flac::flac;

fn main() {
    mlib::version();
    translate();
    mp3();
    flac();
    mlib::flac();
}
