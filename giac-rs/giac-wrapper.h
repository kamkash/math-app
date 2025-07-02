#pragma once

#ifdef __cplusplus
extern "C"
{
#endif

    // Opaque handles
    typedef struct context_opaque context_t;
    typedef struct gen_opaque gen_t;

    // Context management
    context_t *context_new(void);
    void context_free(context_t *ctx);

    // Gen management
    gen_t *gen_new(const char *expr, context_t *ctx);
    const char *gen_to_string(gen_t *g, context_t *ctx);
    void gen_free(gen_t *g);

    // Example API
    gen_t *gen_simplify(gen_t *g, context_t *ctx);
    gen_t *gen_diff(gen_t *g, const char *var, context_t *ctx);
    gen_t *gen_integrate(gen_t *g, const char *var, context_t *ctx);

#ifdef __cplusplus
}
#endif