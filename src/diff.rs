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

    dbg!(&inds);
    let diff = complete_diff(inds);
    dbg!(&diff);
    diff
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
