use ordered_float::*;

fn main() {
    _simplex()
}
fn _simplex() {
    struct Constraint {
        coefficients: Vec<f32>,
        max_value: f32

    }
    impl Constraint {
        fn _equal_to_zero(&mut self) {
        }
    }

    struct ObjectiveFunction {
        //function: [f32;2],
        function: Vec<f32>,
        max: bool
    }
    impl ObjectiveFunction {
        fn equal_to_zero(&mut self) {
            if self.max {
                for i in 0..self.function.len() {
                    self.function[i] = -self.function[i]
                    //i = -i;
                }
            }
        }
    } 

    ///*
    // problem from https://math.libretexts.org/Bookshelves/Applied_Mathematics/Book%3A_Applied_Finite_Mathematics_(Sekhon_and_Bloom)/04%3A_Linear_Programming_The_Simplex_Method/4.02%3A_Maximization_By_The_Simplex_Method
    let function:Vec<f32> = vec![24.0, 22.0, 45.0];
    let constraint1 = Constraint{coefficients: vec![2.0, 1.0, 3.0], max_value: 42.0};
    let constraint2 = Constraint{coefficients: vec![2.0, 1.0, 2.0], max_value: 40.0};
    let constraint3 = Constraint{coefficients: vec![1.0, 0.5, 1.0], max_value: 45.0};
    let constraints = vec![constraint1, constraint2, constraint3];
    //*/

    /*
    //problem from https://www.pmcalculators.com/example/simplex-method-examples/
    let function:Vec<f32> = vec![40.0, 30.0];
    let constraint1 = Constraint{coefficients: vec![1.0, 1.0], max_value: 12.0};
    let constraint2 = Constraint{coefficients: vec![2.0, 1.0], max_value: 16.0};
    let constraints = vec![constraint1, constraint2];
    */
    
    let mut objective_function = ObjectiveFunction{function: function, max:true};
    objective_function.equal_to_zero();
    println!("{:?}", objective_function.function);

    let mut _tableau = initialize_table(constraints, objective_function);
    _tableau._construct_table();
    println!("{:?}", _tableau.tableau);
    while _tableau.continue_simplex() {
        println!("lets go");
        _tableau.find_pivot_column();
        println!("{}", &_tableau.pivot_column);
        _tableau.find_pivot_row();
        println!("{}", &_tableau.pivot_row);
        _tableau.pivot_table();
        println!("{:?}", _tableau.tableau);
        _tableau.pivot_other_rows();
        println!("{:?}", _tableau.tableau);
    }

    struct _Tableau {
        constraints: Vec<Constraint>,
        objective_function: ObjectiveFunction,
        tableau: Vec<Vec<f32>>,
        pivot_column: usize,
        pivot_row: usize
    }
    

    fn initialize_table(constraints: Vec<Constraint>, objective_function: ObjectiveFunction) -> _Tableau {
        let empty_table:Vec<Vec<f32>> = vec![vec![]];
        let pivot_column:usize = 0;
        let pivot_row:usize = 0;
        let mut _tableau = _Tableau{constraints:constraints, objective_function:objective_function, 
            tableau:empty_table, pivot_column, pivot_row};
        _tableau
    }


    impl _Tableau {
        fn pivot_other_rows(&mut self) {
            //let pivot_row = &tableau.tableau[pivot_row_number];
            let pivot_row = self.pivot_row;
            let pivot_column = self.pivot_column;
            for (i, row) in self.tableau.clone().into_iter().enumerate() {
                if i != pivot_row && row[pivot_column] != 0.0 {
                    let multiplier = row[pivot_column];
                    let new_row1: Vec<f32> = self.tableau[pivot_row].iter().map(|e| e*multiplier).collect(); 
                    let new_row2: Vec<f32> = row.iter().zip(&new_row1).map(|(a, b)| a-b).collect();
                    self.tableau[i] = new_row2;
                }
            }
        }
        fn pivot_table(&mut self) {
            let pivot_row = self.pivot_row;
            let pivot_column = self.pivot_column;
            let pivot_element = self.tableau[pivot_row][pivot_column];
            //let pivot_element = 2;
            println!("Pivot element is : {}", pivot_element);
            self.tableau[pivot_row] = self.tableau[pivot_row].iter().map(|e| e/pivot_element).collect();
        }

        fn find_pivot_row(&mut self) {
            let mut pivot_row:usize = 0;
            let mut min_value:f32 = self.constraints[0].max_value;
            //TODO: max values should be divided by the pivot column elements to find pivot row
            for (i, constraint) in self.constraints.iter().enumerate() {
                if constraint.max_value/constraint.coefficients[self.pivot_column] < min_value {
                    min_value = constraint.max_value/constraint.coefficients[self.pivot_column];
                    pivot_row = i;
                }
            }
            self.pivot_row = pivot_row;
        }

        fn continue_simplex(&self) -> bool {
            let zero:Option<&f32> = Some(&0.0);
            for (_,j) in self.tableau.last().unwrap().iter().enumerate() {
                if Some(j) < zero {
                    return true
                }
            }
            return false
        }

        fn find_pivot_column(&mut self) {
            let mut dummy_pivot_column:usize = 0;
            //find the minimal float value by converting it into an OrderedFloat first
            let min_value = self.tableau.last().unwrap().iter().map(|v| OrderedFloat(*v)).min();
            //convert the OrderedFloat back to f32
            let min_float_option = min_value.map(|OrderedFloat(v)| v as f32);
            // get the value inside the Option
            let min_float = match min_float_option {
                Some(x) => x,
                None => 0.0
            };
            //let min_value2 = min_value as f32;
            for (i,j) in self.tableau.last().unwrap().iter().enumerate() {
                if j ==  &min_float {
                    dummy_pivot_column = i;
                }
            } 
            self.pivot_column = dummy_pivot_column;
        }
        fn _construct_table(&mut self) {
            let mut table:Vec<Vec<f32>> = Vec::new();
            let nb_constraints = self.constraints.len();
            for i in 0..nb_constraints {
                let coefficients = self.constraints[i].coefficients.clone();
                table.push(coefficients);
                for j in 0..nb_constraints+1 {
                    if i==j {
                        table[i].push(1.0);
                    }
                    else {
                        table[i].push(0.0);
                    }
                }
                table[i].push(self.constraints[i].max_value);
            }
            let mut objective_row = self.objective_function.function.clone();
            for _ in 0..nb_constraints {
                objective_row.push(0.0)
            }
            let mut last_nb:Vec<f32> = vec![1.0,0.0];
            objective_row.append(&mut last_nb);
            table.push(objective_row);

            self.tableau = table;
        }
    }

}

