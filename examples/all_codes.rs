use inline_colorization::*;

fn main() {
  println!("{rgb} whatever the fuck this is{fg_reset}");

  println!("{apost}apostrophe{apost}");
  println!("{quote}Quotation{quote}");

  println!("no_style");

  // styles
  println!("{style_bold}style_bold{style_reset}");
  println!("{style_underline}style_underline{style_reset}");
  println!("{style_italics}style_italics{style_reset}");
  println!("{style_blink}style_blink{style_reset}");

  // colors
  println!("{bg_white}{fg_black}color_black{fg_reset}{bg_reset}");
  println!("{fg_red}fg_red{fg_reset}");
  println!("{fg_green}fg_green{fg_reset}");
  println!("{fg_yellow}fg_yellow{fg_reset}");
  println!("{fg_blue}fg_blue{fg_reset}");
  println!("{fg_magenta}fg_magenta{fg_reset}");
  println!("{fg_cyan}fg_cyan{fg_reset}");
  println!("{fg_white}fg_white{fg_reset}");
  println!("{fg_white}{fg_b_black}fg_b_black{fg_reset}{bg_reset}");
  println!("{fg_b_red}fg_b_red{fg_reset}");
  println!("{fg_b_green}fg_b_green{fg_reset}");
  println!("{fg_b_yellow}fg_b_yellow{fg_reset}");
  println!("{fg_b_blue}fg_b_blue{fg_reset}");
  println!("{fg_b_magenta}fg_b_magenta{fg_reset}");
  println!("{fg_b_cyan}fg_b_cyan{fg_reset}");
  println!("{fg_b_white}fg_b_white{fg_reset}");

  // backgrounds
  println!("{bg_black}bg_black{bg_reset}");
  println!("{bg_red}bg_red{bg_reset}");
  println!("{bg_green}bg_green{bg_reset}");
  println!("{bg_yellow}bg_yellow{bg_reset}");
  println!("{bg_blue}bg_blue{bg_reset}");
  println!("{bg_magenta}bg_magenta{bg_reset}");
  println!("{bg_cyan}bg_cyan{bg_reset}");
  println!("{fg_black}{bg_white}bg_white{bg_reset}{fg_reset}");
  println!("{bg_b_black}bg_b_black{bg_reset}");
  println!("{bg_b_red}bg_b_red{bg_reset}");
  println!("{bg_b_green}bg_b_green{bg_reset}");
  println!("{bg_b_yellow}bg_b_yellow{bg_reset}");
  println!("{bg_b_blue}bg_b_blue{bg_reset}");
  println!("{bg_b_magenta}bg_b_magenta{bg_reset}");
  println!("{bg_b_cyan}bg_b_cyan{bg_reset}");
  println!("{fg_black}{bg_b_white}bg_b_white{bg_reset}{fg_reset}");

  // other

  let warning = fg_b_yellow.to_owned() + style_bold + style_blink;
  let error = fg_red.to_owned() + style_bold + style_blink;
  let success = fg_cyan.to_owned() + style_bold;
  let full_reset = fg_reset.to_owned() + style_reset;
  println!("{warning}warning{full_reset}");
  println!("{success}success{full_reset}");
  println!("{error}error{full_reset}");
  println!("test_line");
}
