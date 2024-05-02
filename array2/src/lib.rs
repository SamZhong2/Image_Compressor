// Struct storing all of the data the an array
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// * 'data': The data desired to be stored in the array
    ///
    /// Returns the array created
    pub fn new(width: usize, height: usize, data: T) -> Self {
        let data = vec![data; width * height];
        Array2 {
            width,
            height,
            data,
        }
    }

    /// The height
    pub fn height(&self) -> usize {
        self.height
    }

    /// The width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns an immutable reference to the element at the given `column` and `row`
    /// as long as that index is in bounds
    /// (wrapped in [`Some`]). Returns [`None`] if out of bounds.
    ///
    /// # Arguments
    ///
    /// * `c`: the column index.
    /// * `r`: the row index.
    ///
    /// # Returns
    ///
    /// * An `Option<&T>` which is a reference to the value at
    /// coordinates `(c,r)` if those indices are in bounds,
    /// and `None` otherwise.
    pub fn get(&self, c: usize, r: usize) -> Option<&T> {
        self.get_index(c, r).map(|index| &self.data[index])
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y * self.width + x] = value;
        
    }

    /// Returns an immutable reference to the element at the given `column` and `row`
    /// as long as that index is in bounds
    /// (wrapped in [`Some`]). Returns [`None`] if out of bounds.
    ///
    /// # Arguments
    ///
    /// * `c`: the column index.
    /// * `r`: the row index.
    ///
    /// # Returns
    ///
    /// * An `Option<&mut T>` which is a mutable reference to the value at
    /// coordinates `(c,r)` if those indices are in bounds,
    /// and `None` otherwise.
    pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
        self.get_index(c, r).map(move |index| &mut self.data[index])
    }

    /// helper function which implements the representation invariant
    /// returns an `Option<usize>` which is the index in the
    /// internal `data` `Vec` of the requested element if it's in bounds,
    /// and `None` otherwise
    fn get_index(&self, c: usize, r: usize) -> Option<usize> {
        if c < self.width && r < self.height {
            Some(r * self.width + c)
        } else {
            None
        }
    }

    /// An Iterator over the values of the underlying
    /// representation.
    ///
    /// # Returns
    ///
    /// * An `Iterator<Item = &elem>` which is a reference to a contained value.
    fn _iter_values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    fn _iter_row_major_naive(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(move |(i, element)| {
            let c = i % self.width;
            let r = i / self.width;
            (c, r, element)
        })
    }

    /// Returns an iterator over the rows of the underlying
    /// representation.
    ///
    /// # Returns
    ///
    /// * An `Iterator<Item = (row, &elem)>` which is a 2-tuple holding the row index and
    /// a reference to a contained value.
    fn iter_rows(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &T>)> {
        self.data
            .chunks(self.width)
            .enumerate()
            .map(|(i, row)| (i, row.iter()))
    }

    /// An Iterator over the values of the `Array2` in row-major order.
    ///
    /// # Returns
    ///
    /// * An `Iterator<Item = (col, row, &elem)>` which is a 3-tuple holding the column index,
    /// row index, and a reference to a contained value.
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.iter_rows()
            .flat_map(|(r, row)| row.enumerate().map(move |(c, element)| (c, r, element)))
    }

    /// Returns an iterator over the columns of the underlying
    /// representation.
    ///
    /// # Returns
    ///
    /// * An `Iterator<Item = (col, &elem)>` which is a 2-tuple holding the column index and
    /// a reference to a contained value.
    fn iter_cols(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = &T>)> {
        (0..self.width)
            // get the start of every column as a fresh iter and keep the index of the column
            .map(move |c| (c, self.data.iter().skip(c)))
            // for each iterator on a column, step forward by width for the correct next element in that column
            .map(move |(c, col_start)| (c, col_start.step_by(self.width)))
    }

    /// An Iterator over the values of the `Array2` in column-major order.
    ///
    /// # Returns
    ///
    /// * An `Iterator<Item = (col, row, &elem)>` which is a 3-tuple holding the column index,
    /// row index, and a reference to a contained value.
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.iter_cols()
            .flat_map(|(c, col)| col.enumerate().map(move |(r, element)| (c, r, element)))
    }
    
    /// from_row_major
    /// 
    /// # Arguments
    /// 
    /// * 'width': the width of the 'Array2'
    /// * 'height': the height of the 'Array2'
    /// * 'elements': the elements being read in in row major
    /// 
    /// Returns the array2 read in as row major
    pub fn from_row_major(width: usize, height: usize, elements: &Vec<T>) -> Self {
        

        Self {
            width,
            height,
            data: elements.clone(),
        }
    }

    /// from_col_major
    /// 
    /// # Arguments
    /// 
    /// * 'width': the width of the 'Array2'
    /// * 'height': the height of the 'Array2'
    /// * 'elements': the elements being read in in column major
    /// 
    /// Returns the array2 read in as row major
    pub fn from_col_major(width: usize, height: usize, elements: &Vec<T>) -> Self {
       
        let mut data = Vec::with_capacity(width * height);

        for col in 0..width {
            for row in 0..height {
                let index = col * height + row;
                data.push(elements[index].clone());
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
}
