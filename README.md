## Simplex

A simplex is an optimization algorithm used to solve linear programming problems.
The simplex method is applicable to any problem that can be formulated in terms of linear objective function subject to a set of linear constraints.

The simplex algorithm is an iterative process.

### Linear pogramming problems
These are problems where the solution consists of finding values for requireements that have a linear relationship between them.

### linear vs non linear models:
Linear models are basically models that produce an output depending on parameters multiplied by independent variable
We say that this model is Linear in the parameters.
In linear models, by doubling the target, the parameters will double as well.
#### Example
The side of a square and its area are not linear. In fact, this is a quadratic relationship. If
you double the side of a square, its area will increase 4 times.
The relationship between the radius of a circle and it's area is not linear as well.
There are other types of non-linear relationships:
- Quadratic
- Exponential
- Logarithmic

## simplex
The solution of a linear programming problem always occurs at one of the corner points of the solution space.
The solution space looks like a polytope. 
The goal is to jump from one solution to another in an iterative way such that the value of the objective function is improved.

The simplex algorithm works in two phases:
- A starting point is found on one of the extreme points.
- We move from one feasible solution to another using the pivot operation

The inequalities of the constraint problem are transformed into	equalities by ccreating new terms called slack variables.

1. Transform the inegalities into egalities
2. Tranform minimization objective funtions into maximization. Set objective function equal to 0
3. Write down the parameters in a table called a canonical table
4. Find the pivot row and pivot value and add the (pivot value) * (pivot row) to the other rows
5. The pivot column is the column containing the value of the objective row with the most negative coefficient. We take the most negative column because that is the one that will have the biggest influence on the objective function.
6. Then we do the pivot. Pivoting is about transforming the pivot element into a 1 and have all transform all the other values of that column into 0's by adding the the pivot row.

## Problem examples:
Two examples are already implemented in the code. 
These examples were retrieved from :
- https://www.pmcalculators.com/example/simplex-method-examples/ 
- https://math.libretexts.org/Bookshelves/Applied_Mathematics/Book%3A_Applied_Finite_Mathematics_(Sekhon_and_Bloom)/04%3A_Linear_Programming_The_Simplex_Method/4.02%3A_Maximization_By_The_Simplex_Method

One can switch from one example to the other by commenting/uncommenting parts of the code. You can also solve your own problem by specifying the objective function + direction (min/max) and the constraints.

Improvements to the code could include passing arguments of a problem through the command line.

