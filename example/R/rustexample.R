#' Sort a vector using Rust
#'
#' @rdname rust.sort
#' @param v Vector to sort
#' @return Sorted vector
#' @examples rust.sort(c(5, 3, 1, 2, 4))
#' @useDynLib example rust_sort
rust.sort <- function(v) {
    .Call(rust_sort, v)
}

#' Calculate the mean of a vector using Rust
#'
#' @rdname rust.mean
#' @param v Vector to calculate the mean on
#' @return Mean of the vector
#' @examples rust.mean(rnorm(64, mean=5, sd=1))
#' @useDynLib example rust_mean
rust.mean <- function(v) {
    .Call(rust_mean, v)
}
