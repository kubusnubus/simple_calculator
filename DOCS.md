## USAGE OF THE SUPPORTED FUNCTIONS:
* addition: marked by the '+' character, gathers input from both sides of the character
* subtraction: marked by the '-' character, gathers input from both sides of the character
* multiplication: marked by the '*' character, gathers input from both sides of the character
* division: marked by the '/' character, gathers input from both sides of the character, division by zero will result in an error
* exponentiation: marked by the '^' character, gathers input from both sides of the character, exponentiating a negative number to a fractional power will result in NaN
* factorial: marked by the '!' character, gathers input from the left side of the character, the only argument values are the integers from 0 to 200 included, if input is not one of these numbers, it will result in an error
* sin: marked by the "sin" string, gathers input from the right side of the string
* cos: the same usage as the sin function
* tan: implemented as sin/cos, if the argument is one of pi/2 + k*pi (k being an integer), it will result in an error
* arcsin: marked by the "asin" string, gathers input from the right side of the string (because of the current implementation, the further away the input is from 0, the less accurate the result will be, please try not to enter values close to -1 or close to 1), the input being less that -1 or greater than 1 will result in an error
* arccos: marked by the "acos" string, other than that, it is the same as with the arcsin function
* arctan: marked by the "arctan" string, gathers input from the right side of the string (because of the current implementation, the further away the input is from 0, the less accurate the result will be, please try not to enter large values)
* exp: marked by the "exp", string, gathers input from the right side of the string, exp(x) is equivalent to e^x, imaginary numbers are not yet supported
* ln: marked by the "ln" string, gathers input from the right side of the string, the input value being less than or equal to zero will result in an error
* log: marked by the "log" string, gathers two arguments (first-base, second-the number of what the logarithm is taken) from the right separated by a comma, the two inputs need to be closed in parentheses (like log(a, b)), 'b' being less than or equal to zero, 'a' being equal to one or less than or equal to zero will result in an error
* productlog: marked by the 'W' character, gathers two arguments (first-the branch, -1 for the negative part of the domain or 0 for the entire domain, second-the number of what the productlog is taken) from the right side separated by a comma, the two inputs need to be closed in parentheses (like W(a, b)), 'b' being less than -e^(-1) will cause an error, the branch being equal to -1 for x greater than or equal to 0 will also result in an error
* sqrt: marked by the "sqrt" string, gathers input from the right side of the string, the input value being less than zero will result in an error
* floor: marked by the "floor" string, gathers input from the right side of the string
* ceil: marked by the "ceil" string, gathers input from the right side of the string
* mod: marked by the "mod" string, gathers input from both sides of the string, the second input value being equal to zero will result in an error
* sum (capital sigma): marked by the "sum" string, gathers three arguments from the right side (first-the number the summation starts on, second-the number the summation ends on, third-the summed up expression, 'k' being the index variable), invalid number of commas (precisely two are needed) will result in an error, the three arguments need to be closed in parentheses (like sum(1, 10, k))
* product (capital pi): marked by the "prod" string, the usage is the same as with the sum function
* definite integral: marked by the "int" string, the usage is the same as with the sum function, just the integration variable is 'x'
* derivative: marked by the "der" string, gathers two arguments from the right side (first-the point the derivative is taken at, second-the derived expression, the variable is 'x')
