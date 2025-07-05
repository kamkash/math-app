#pragma once
#include <stddef.h>

#ifdef __cplusplus
extern "C"
{
#endif

    // Opaque handles
    struct context_opaque;
    struct gen_opaque;
    typedef struct context_opaque context_t;
    typedef struct gen_opaque gen_t;

    // Context management
    context_t *context_new(void);
    void context_free(context_t *ctx);

    // Gen management
    gen_t *gen_new(const char *expr, context_t *ctx);
    gen_t *gen_new_from_double(double value, context_t *ctx);
    const char *gen_to_string(gen_t *g, context_t *ctx);
    void gen_free(gen_t *g);

    // Example API
    gen_t *gen_simplify(gen_t *g, context_t *ctx);
    gen_t *gen_diff(gen_t *g, const char *var, context_t *ctx);
    gen_t *gen_integrate(gen_t *g, context_t *ctx);

    // Arithmetic operators
    gen_t *gen_add(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_sub(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_mul(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_div(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_pow(gen_t *a, gen_t *b, context_t *ctx);

    // Symbolic constants
    gen_t *gen_pi(context_t *ctx);
    gen_t *gen_e(context_t *ctx);

    // Symbolic operators
    gen_t *gen_symb_plus(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_symb_mult(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_symb_pow(gen_t *a, gen_t *b, context_t *ctx);

    gen_t *gen_subs(gen_t *expr, const char **vars, gen_t **values, size_t n, context_t *ctx);
    gen_t *gen_eval(gen_t *expr, context_t *ctx);

#ifdef __cplusplus
}
#endif