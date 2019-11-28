dyn.load("../target/debug/libexample.so")

print(is.loaded("hello_wrapper"))

hello_world <- function() {
    .Call("hello_wrapper", "Tim")
}

meaning <- function() {
    .Call("meaning", 42)
}

circ <- function() {
    .Call("circ", 1)
}

is_awesome <- function() {
    .Call("is_awesome", FALSE)
}

print(hello_world())
print(meaning())
print(circ())
print(is_awesome())
v <- .Call("create_vec")
print(head(v))
print(tail(v))
print(head(length(v)))
v2 <- .Call("create_vec_nested")
print(v2)
.Call("print_vec_nested", v2)
print(.Call("nihilism"))

.Call("hello_bindgen")
print(.Call("its_a_vec", c(0, 1, 2, 3, 4, 5)))

randv <- sample(1:64, 64)
print(randv)
print(.Call("rust_sort", randv))
