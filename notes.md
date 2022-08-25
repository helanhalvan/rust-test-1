# Syntax
A program consists of functions, which are defined by 1+ variable binding, followed by either a single evaluation, or 1+ preconditon/evaluation pairs. 

The function call add(5,0) executes by:
Binding A=5, B=0
Evaluating preconditions, 
    A =/= 0 && B =/= 0 -> false
    A == 0 -> false
    B == 0 -> true
Executing the function 
    A -> A

add
    (A, B) // variable bind
    { A =/= 0 && B =/= 0 } // precondition expr, must be side effect free!
    | A + B | // evaluation
    { A == 0 }
    | B |
    { B == 0 }
    | A |
end

map
    ([], F, A)
        | A |
    ([H|T], F, A)
        | map(T, F, [Acc|F(H)]) |
end
