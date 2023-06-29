use std::collections::HashSet;
use crate::solver::graph::Graph;

/// N-dimensional lattice graph, with some coordinates cyclic or acyclic.
#[derive(Debug)]
pub struct GridND {
    /// Number of points in each direction (vector length is # dimensions). 0 and 1 not allowed.
    dimensions: Vec<usize>,

    /// Steps in each of the directions (not exposed).
    /// # Example:
    /// To move in the positive 3rd direction, add step_sizes.get(2) to the coordinate
    step_sizes: Vec<usize>,

    /// Do the dimension loop around?
    glue: Vec<bool>,

    /// How many points are there in this graph?
    /// For looping over all points
    nr_points: usize,
}


impl From<(Vec<usize>, Vec<bool>)> for GridND {
    /// Construct an nD grid from two vectors.
    /// # Parameters:
    /// `value = (dimensions, glue)`
    /// * `dimensions`: the vector of dimensions of the grid.
    /// * `glue`: in what way the grid is glued together. True in the ith coordinate means that the ith coordinate is cyclic, false means that the coordinate is acyclic.
    /// # Glue examples
    ///  * false: line segment
    ///  * true: circle
    ///  * false, false: rectangle
    ///  * false, true or true, false: thin cylinder wall
    ///  * true, true: torus
    ///  * true, false, false: thick cylinder wall
    /// # Dimension examples
    ///  * 10: line/circle of 10 vertices
    ///  * 4, 10: rectangle/thin cylinder wall/torus of 40 vertices
    /// # Example
    /// 40x40 (1600 vertices) thin cylinder wall grid, where the first dimension is cyclic and the second is not
    /// ```
    /// let g = GridND::from((vec![40, 40], vec![true, false]))
    /// ```
    fn from(value: (Vec<usize>, Vec<bool>)) -> Self {
        let (dimensions, glue) = value;

        // Make sure that we have enough glue-data to specify the entire GridND
        assert_eq!(dimensions.len(), glue.len());
        assert!(!dimensions.contains(&0usize));
        assert!(!dimensions.contains(&1usize));

        // compute step sizes
        let mut step_sizes = vec![];
        let mut running_product = 1;
        for i in dimensions.iter() {
            step_sizes.push(running_product.clone());
            running_product *= i;
        }

        let nr_points = step_sizes.last().unwrap() * dimensions.last().unwrap();

        GridND {
            dimensions,
            step_sizes,
            glue,
            nr_points,
        }
    }
}

impl From<Vec<usize>> for GridND {
    /// Construct a cyclic nD grid from a vector.
    /// # Parameter:
    /// * `dimensions` the vector of dimensions of the grid
    /// # Example
    /// 40x20 (800 vertices) toroidal grid, where the both dimensions are cyclic
    /// ```
    /// let g = GridND::from(vec![40, 40])
    /// ```
    fn from(dimensions: Vec<usize>) -> Self {
        let glue: Vec<bool> = vec![true; dimensions.len()];

        GridND::from((dimensions, glue))
    }
}

impl Graph for GridND {
    fn nr_points(&self) -> usize {
        self.nr_points
    }

    // Finding the neighbors of a particular inspection point on the regular grid (hard logic, think deeply)
    fn get_neighbors(&self, particle: usize) -> HashSet<usize> {
        let mut neighbors: HashSet<usize> = HashSet::new();

        for (dimension_index, step_size) in self.step_sizes.iter().enumerate() {
            let current_dimension = self.dimensions.get(dimension_index).unwrap();
            // the coordinate of the point in the current dimension
            let current_coordinate = particle / step_size % current_dimension;

            if current_coordinate == 0 {
                // Check if the inspection point is on the close boundary for the dimension

                // now only the + is valid
                neighbors.insert(particle + step_size);

                if *self.glue.get(dimension_index).unwrap() { // If this dimension is cyclic, loop around
                    neighbors.insert(particle + step_size * current_dimension - step_size);
                }
            } else if current_coordinate == current_dimension - 1 {
                // Check if the inspection point is on the far boundary for the dimension

                // now only the - is valid
                neighbors.insert(particle - step_size);

                if *self.glue.get(dimension_index).unwrap() { // if this dimension is cyclic, loop around
                    neighbors.insert(particle + step_size - step_size * current_dimension);
                }
            } else {
                // hence the point must be a generic point (in the middle)
                neighbors.insert(particle + step_size);
                neighbors.insert(particle - step_size);
            }
        }

        neighbors
    }

    fn describe(&self) {
        match self.dimensions.len() {
            0 => {
                panic!("Wrongly constructed grid graph!")
            }
            1 => {
                if self.glue[0] {
                    println!("Circle graph with {} points.", self.dimensions[0])
                }
            }
            2 => {
                if self.glue[0] && self.glue[1] {
                    println!("2D toroidal graph, of size {}x{}.",
                             self.dimensions[0], self.dimensions[1])
                } else if self.glue[0] {
                    println!("2D cylinder graph, where the first dimension is cyclic, of size {}x{}",
                             self.dimensions[0], self.dimensions[1])
                } else if self.glue[1] {
                    println!("2D cylinder graph, where the second dimension is cyclic, of size {}x{}",
                             self.dimensions[0], self.dimensions[1])
                } else {
                    println!("Rectangular graph, of size {}x{}",
                             self.dimensions[0], self.dimensions[1])
                }
            }
            _ => {
                println!("General {}D grid graph, glued at {:?}, of size {:?}", self.dimensions.len(),
                         self.glue, self.dimensions)
            }
        }
    }
}
