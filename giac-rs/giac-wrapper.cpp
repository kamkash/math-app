#include "giac-wrapper.h"

#include "gen.h"
#include "giac.h"

#include <string>
#include <new>

using namespace giac;

// Opaque structs
struct context_opaque
{
    context value;
};

struct gen_opaque
{
    gen value;
};

// Context management
context_t *context_new(void)
{
    return new context_t{context()};
}

void context_free(context_t *ctx)
{
    delete ctx;
}

// Gen management
gen_t *gen_new(const char *expr, context_t *ctx)
{
    try
    {
        gen_t *g = new gen_t;
        g->value = gen(expr, &ctx->value);
        return g;
    }
    catch (...)
    {
        return nullptr;
    }
}

const char *gen_to_string(gen_t *g, context_t *ctx)
{
    static std::string result;
    try
    {
        result = g->value.print(&ctx->value);
        return result.c_str();
    }
    catch (...)
    {
        result = "[ERROR]";
        return result.c_str();
    }
}

void gen_free(gen_t *g)
{
    delete g;
}

// Example API
gen_t *gen_simplify(gen_t *g, context_t *ctx)
{
    try
    {
        gen_t *result = new gen_t;
        result->value = simplify(g->value, &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_diff(gen_t *g, const char *var, context_t *ctx)
{
    try
    {
        gen_t *result = new gen_t;
        result->value = derive(g->value, identificateur(var), &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_integrate(gen_t *g, context_t *ctx)
{
    try
    {
        gen_t *result = new gen_t;
        result->value = _integrate(g->value, &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}