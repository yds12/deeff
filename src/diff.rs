use crate::CFG;

#[derive(Debug)]
struct DiffCell(usize);

impl From<usize> for DiffCell {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl From<bool> for DiffCell {
    fn from(val: bool) -> Self {
        Self(if val { 1 } else { 0 })
    }
}

struct DiffTable(Vec<DiffCell>, usize);

impl std::fmt::Debug for DiffTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let left = self.1;
        let right = self.0.len() / left;

        write!(f, "\n")?;

        for i in 0..left {
            for j in 0..right {
                write!(f, "{}", self.get(i, j).0)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl DiffTable {
    pub fn new(left: usize, right: usize) -> Self {
        let vec = (0..left * right).map(|_| false.into()).collect();
        Self(vec, left)
    }

    pub fn get(&self, left: usize, right: usize) -> &DiffCell {
        &self.0[left * self.1 + right]
    }

    pub fn set(&mut self, left: usize, right: usize, val: DiffCell) {
        self.0[left * self.1 + right] = val;
    }

    fn find_max(&self, left: usize, right: usize) -> (usize, usize, usize) {
        let mut max = 0;
        let mut maxi = 0;
        let mut maxj = 0;

        for i in 0..left {
            for j in 0..right {
                let val = self.get(i, j).0;

                if val > max {
                    max = val;
                    maxi = i;
                    maxj = j;
                }
            }
        }

        return (max, maxi, maxj);
    }

    pub fn calculate_lengths(&mut self) {
        let left = self.1;
        let right = self.0.len() / left;

        for i in 0..left {
            for j in 0..right {
                let cur = self.get(i, j).0;

                if cur == 0 {
                    continue;
                }
                let val = self.find_max(i, j).0 + 1;
                self.set(i, j, val.into());
            }
        }
    }

    pub fn find_indices(&mut self) -> Vec<(usize, usize)> {
        let mut indices = Vec::new();
        let left = self.1;
        let right = self.0.len() / left;

        let mut i = left;
        let mut j = right;

        loop {
            let (_, maxi, maxj) = self.find_max(i, j);
            indices.push((maxi, maxj));

            i = maxi;
            j = maxj;

            if i * j == 0 {
                break;
            }
        }

        indices.iter().rev().map(|&(i, j)| (i, j)).collect()
    }
}

pub fn diff<T, F>(left: &Vec<T>, right: &Vec<T>, cmp: F) -> Vec<(Option<usize>, Option<usize>)>
where
    F: Fn(&T, &T) -> bool,
{
    let mut matrix = DiffTable::new(left.len(), right.len());

    for i in 0..left.len() {
        for j in 0..right.len() {
            matrix.set(i, j, cmp(&left[i], &right[j]).into());
        }
    }

    matrix.calculate_lengths();
    let inds = matrix.find_indices();
    complete_diff(inds)
}

fn complete_diff(indices: Vec<(usize, usize)>) -> Vec<(Option<usize>, Option<usize>)> {
    let mut diff = Vec::new();
    let mut last_i = 0;
    let mut last_j = 0;

    for (i, j) in indices {
        while i - last_i > 1 {
            diff.push((Some(last_i + 1), None));
            last_i += 1;
            continue;
        }
        while j - last_j > 1 {
            diff.push((None, Some(last_j + 1)));
            last_j += 1;
            continue;
        }
        diff.push((Some(i), Some(j)));
        last_i = i;
        last_j = j;
    }

    diff
}

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
    format: F,
    compare: G,
) where
    F: Fn(&T) -> &str,
    G: Fn(&T, &T) -> bool,
{
    let show_adds = !CFG.only_dels;
    let show_dels = !CFG.only_adds;
    let show_small_change = !CFG.only_adds && !CFG.only_dels && !CFG.only_dels_and_adds;
    let show_match = !CFG.only_diff && show_small_change;

    let only_diff = CFG.only_diff;
    let color = !CFG.no_color;

    let open_red = "\x1b[0;31m";
    let open_green = "\x1b[0;32m";
    let open_blue = "\x1b[0;36m";
    let close = "\x1b[0m";

    let width = 90;
    for (l, r) in alignment {
        match (l, r) {
            (Some(l), Some(r)) => {
                let total_match = compare(&left[l], &right[r]);

                if show_match || (show_small_change && !total_match) {
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
                if show_dels {
                    if color {
                        print!("{}", open_red);
                    }
                    println!("{}\t{:<width$}  \t{:<width$}", l, format(&left[l]), "");
                    if color {
                        print!("{}", close);
                    }
                }
            }
            (None, Some(r)) => {
                if show_adds {
                    if color {
                        print!("{}", open_green);
                    }
                    println!(" \t{:<width$} {}\t{:<width$}", "", r, format(&right[r]));
                    if color {
                        print!("{}", close);
                    }
                }
            }
            (None, None) => println!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec!["a", "b", "c", "d"];
        let b = vec!["a", "e", "b", "c", "d"];
        diff(&a, &b, |c, d| c == d);
    }
}
