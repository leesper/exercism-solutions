pub fn reverse(input: &str) -> String {
    use unicode_reverse::reverse_grapheme_clusters_in_place;
    let mut reversed = input.to_string();
    reverse_grapheme_clusters_in_place(&mut reversed);
    reversed
}
