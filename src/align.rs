pub fn align<T, F>(left: &Vec<T>, right: &Vec<T>, compare: F) -> Vec<(Option<usize>, Option<usize>)>
where
    F: Fn(&T, &T) -> bool,
{
    let mut result = Vec::new();
    let mut left_used: Vec<bool> = (0..left.len()).map(|_| false).collect();
    let mut right_used: Vec<bool> = (0..right.len()).map(|_| false).collect();
    let mut indices = (0_usize, 0_usize);

    loop {
        if (indices.0 <= indices.1 || indices.1 >= right.len()) && indices.0 < left.len() {
            let i = indices.0;

            if !left_used[i] {
                match right
                    .iter()
                    .enumerate()
                    .find(|(j, elem)| !right_used[*j] && compare(elem, &left[i]))
                {
                    Some((index, _)) => {
                        right_used[index] = true;
                        result.push((Some(i), Some(index)));
                    }
                    None => result.push((Some(i), None)),
                }
                left_used[i] = true;
            }

            indices.0 += 1;
        } else if indices.1 < right.len() {
            let i = indices.1;

            if !right_used[i] {
                match left
                    .iter()
                    .enumerate()
                    .find(|(j, elem)| !left_used[*j] && compare(elem, &right[i]))
                {
                    Some((index, _)) => {
                        left_used[index] = true;
                        result.push((Some(index), Some(i)));
                    }
                    None => result.push((None, Some(i))),
                }
                right_used[i] = true;
            }

            indices.1 += 1;
        } else {
            break;
        }
    }

    result
}

pub fn print_alignment<T, F, G>(
    left: &Vec<T>,
    right: &Vec<T>,
    alignment: Vec<(Option<usize>, Option<usize>)>,
    only_diff: bool,
    color: bool,
    format: F,
    compare: G,
) where
    F: Fn(&T) -> &str,
    G: Fn(&T, &T) -> bool,
{
    let open_red = "\x1b[0;31m";
    let open_green = "\x1b[0;32m";
    let open_blue = "\x1b[0;36m";
    let close = "\x1b[0m";

    let width = 90;
    for (l, r) in alignment {
        match (l, r) {
            (Some(l), Some(r)) => {
                if !only_diff {
                    let total_match = compare(&left[l], &right[r]);

                    if color && !total_match {
                        print!("{}", open_blue);
                    }
                    println!(
                        "{}\t{:<width$} {}\t{:<width$}",
                        l,
                        format(&left[l]),
                        r,
                        format(&right[r])
                    );
                    if color && !total_match {
                        print!("{}", close);
                    }
                }
            }
            (Some(l), None) => {
                if color {
                    print!("{}", open_red);
                }
                println!("{}\t{:<width$}  \t{:<width$}", l, format(&left[l]), "");
                if color {
                    print!("{}", close);
                }
            }
            (None, Some(r)) => {
                if color {
                    print!("{}", open_green);
                }
                println!(" \t{:<width$} {}\t{:<width$}", "", r, format(&right[r]));
                if color {
                    print!("{}", close);
                }
            }
            (None, None) => println!(),
        }
    }
}
