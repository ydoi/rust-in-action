fn main() {
  let search_term = "picture";
  let quote = "\
Every face, every shop, badroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.What do we seek through millions of page?";
  for (i, line) in quote.lines().enumerate() {
    if line.contains(search_term) {
      let line_num = i + 1;
      println!("{}: {}", line_num, line);
    }
  }
}
