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
    gen_t *gen_symb_sub(gen_t *a, gen_t *b, context_t *ctx);
    gen_t *gen_symb_div(gen_t *a, gen_t *b, context_t *ctx);

    // Trigonometric functions
    gen_t *gen_symb_sin(gen_t *a, context_t *ctx);
    gen_t *gen_symb_cos(gen_t *a, context_t *ctx);
    gen_t *gen_symb_tan(gen_t *a, context_t *ctx);
    // Square root (symbolic)
    gen_t *gen_symb_sqrt(gen_t *a, context_t *ctx);

    // logs
    gen_t *gen_symb_log(gen_t *a, context_t *ctx);
    gen_t *gen_symb_ln(gen_t *a, context_t *ctx);
    gen_t *gen_symb_exp(gen_t *a, context_t *ctx);

    // Evaluation and substitution
    gen_t *gen_subs(gen_t *expr, const char **vars, gen_t **values, size_t n, context_t *ctx);
    gen_t *gen_eval(gen_t *expr, context_t *ctx);

    // Vecteur (list) helpers: detect and access elements
    // Note: these helpers accept a context pointer to produce reliable string output
    int gen_is_vecteur(const gen_t *g, context_t *ctx);
    size_t gen_vecteur_len(const gen_t *g, context_t *ctx);
    gen_t *gen_vecteur_get(const gen_t *g, size_t i, context_t *ctx);

    gen_t *gen_clone(gen_t *g, context_t *ctx);

    int is_symbol(const gen_t *g);
    int is_number(const gen_t *g);
    int is_constant(const gen_t *g);

    int equals(const gen_t *a, const gen_t *b);

    int is_add(const gen_t *g);
    int is_sub(const gen_t *g);
    int is_mul(const gen_t *g);
    int is_div(const gen_t *g);
    int is_pow(const gen_t *g);

    int is_not(const gen_t *g);
    int is_and(const gen_t *g);
    int is_or(const gen_t *g);
    int is_eq(const gen_t *g);
    int is_ne(const gen_t *g);
    int is_lt(const gen_t *g);
    int is_le(const gen_t *g);
    int is_gt(const gen_t *g);
    int is_ge(const gen_t *g);

    gen_t *get_add_op();
    gen_t *get_sub_op();
    gen_t *get_mul_op();
    gen_t *get_div_op();
    gen_t *get_pow_op();
    gen_t *get_neg_op();
    gen_t *get_inv_op();
    gen_t *get_not_op();
    gen_t *get_and_op();
    gen_t *get_or_op();
    gen_t *get_eq_op();
    gen_t *get_ne_op();
    gen_t *get_lt_op();
    gen_t *get_le_op();
    gen_t *get_gt_op();
    gen_t *get_ge_op();

    int gen_to_f64(gen_t *g, double *out_ptr);
    
#ifdef __cplusplus
}
#endif