# inline_colorization
Copy and paste the library through writing in Cargo.toml under dependencies:
```
inline_colorization = { git = "https://github.com/d4140n-4h3-1/inline_colorization.git" }
```
And in you main.rs file:
```
use inline_colorization::*;
```
Then you can run:
```
println!("Lets the user {fg_red}colorize{fg_reset} and {style_underline}style the output{style_reset} text using inline variables");
```
| Text Style Variables |
|----------------------|
| style_bold           |
| style_underline      |
| style_italics        |
| style_blink          |
| style_reset          |


| Text Color Variables |
|----------------------|
| fg_black             |
| fg_red               |
| fg_green             |
| fg_yellow            |
| fg_blue              |
| fg_magenta           |
| fg_cyan              |
| fg_white             |
| fg_b_black           |
| fg_b_red             |
| fg_b_green           |
| fg_b_yellow          |
| fg_b_blue            |
| fg_b_magenta         |
| fg_b_cyan            |
| fg_b_white           |
| fg_reset             |


| Text Background Variables |
|---------------------------|
| bg_black                  |
| bg_red                    |
| bg_green                  |
| bg_yellow                 |
| bg_blue                   |
| bg_magenta                |
| bg_cyan                   |
| bg_white                  |
| bg_b_black                |
| bg_b_red                  |
| bg_b_green                |
| bg_b_yellow               |
| bg_b_blue                 |
| bg_b_magenta              |
| bg_b_cyan                 |
| bg_b_white                |
| bg_reset                  |

Just remember to reset the style, color or background when you want the default text setting

For an example of the expected result you can run:

```
cargo run --example all_codes
```
