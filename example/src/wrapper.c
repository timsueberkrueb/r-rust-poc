#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <Rinternals.h>

SEXP rust_sort(SEXP v);
SEXP rust_mean(SEXP v);

static const R_CallMethodDef CallEntries[] = {
  {"rust_sort", (DL_FUNC) &rust_sort, 1},
  {"rust_mean", (DL_FUNC) &rust_mean, 1},
  {NULL, NULL, 0}
};

void R_init_rustexample(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
