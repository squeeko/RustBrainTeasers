fn main() {
    if 'X' == 'Χ' {
        println!("It matches");
    } else {
        println!("It doesn't match");
    }
}
/*
 This is how hackers pass fake URLs etc. THe first letter is US capital 'X' and the other is the Greek capital 'Chi' that looks like 'X'.
 LOL this is too funny!!!!!!

 Unicode allows for homoglyphs, which are very similar or identical
characters that can be encoded in different ways. The first X is the Latin
Unicode character, encoded as 0x58. The second Χ is the Greek capital letter
Chi, encoded in UTF-8 as 0xCE 0xA7. If you look closely, they aren’t identical,
but in some fonts—notably Consolas on Windows—they are indistinguishable.

Wikipedia Homoglyphs
https://en.wikipedia.org/wiki/Homoglyph
Homoglyph Attack Generator
https://www.irongeek.com/homoglyph-attack-generator.php
Nettfiske—Homoglyph Detector Crate
https://crates.io/crates/nettfiske
Unicode Reverse Crate
https://crates.io/crates/unicode-reverse
Weird Text Generator
https://lingojam.com/WeirdTextGenerator

 */
