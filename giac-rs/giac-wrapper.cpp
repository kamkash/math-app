#include "giac-wrapper.h"

#include "gen.h"
#include "giac.h"
#include "static_extern.h"
#include "sym2poly.h"

#include <string>
#include <new>
#include <mutex>


using namespace giac;

// Global mutex for thread safety
static std::mutex giac_mutex;

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
    std::lock_guard<std::mutex> lock(giac_mutex);
    return new context_t{context()};
}

void context_free(context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    delete ctx;
}

// Gen management
gen_t *gen_new(const char *expr, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try
    {
        gen_t *g = new gen_t();
        g->value = gen(expr, &ctx->value);
        return g;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_new_from_double(double value, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try
    {
        gen_t *g = new gen_t();
        g->value = gen(value);
        return g;
    }
    catch (...)
    {
        return nullptr;
    }
}

const char *gen_to_string(gen_t *g, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    static std::string result = "";
    try
    {
        result = g->value.print(&ctx->value);
        return result.c_str();
    }
    catch (...)
    {
        return "[ERROR]";
    }
}

void gen_free(gen_t *g)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    delete g;
}

// Example API
gen_t *gen_simplify(gen_t *g, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
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
    std::lock_guard<std::mutex> lock(giac_mutex);
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
    std::lock_guard<std::mutex> lock(giac_mutex);
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

// Arithmetic operators
gen_t *gen_add(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = a->value + b->value;
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_sub(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = a->value - b->value;
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_mul(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = a->value * b->value;
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_div(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = a->value / b->value;
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_pow(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = pow(a->value, b->value, &ctx->value);
        return result;
    } catch (...) {
        return nullptr;
    }
}

// Symbolic constants
gen_t *gen_pi(context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = cst_pi;
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_e(context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = exp(1);
        return result;
    } catch (...) {
        return nullptr;
    }
}

// Symbolic operators
gen_t *gen_symb_plus(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_plus, v);
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_symb_mult(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_multiply, v);
        return result;
    } catch (...) {
        return nullptr;
    }
}

gen_t *gen_symb_pow(gen_t *a, gen_t *b, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_pow, v);
        return result;
    } catch (...) {
        return nullptr;
    }
}

// Substitute variables in an expression
// vars: array of variable names, values: array of gen_t*, n: number of substitutions
gen_t *gen_subs(gen_t *expr, const char **vars, gen_t **values, size_t n, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        vecteur subs_vec;
        subs_vec.push_back(expr->value); // First element is the expression itself
        for (size_t i = 0; i < n; ++i) {
            subs_vec.push_back(symb_equal(identificateur(vars[i]), values[i]->value));
        }
        gen_t *result = new gen_t;
        result->value = _subs(gen(subs_vec), &ctx->value);
        return result;
    } catch (...) {
        return nullptr;
    }
}

// Evaluate an expression in a context
gen_t *gen_eval(gen_t *expr, context_t *ctx)
{
    std::lock_guard<std::mutex> lock(giac_mutex);
    try {
        gen_t *result = new gen_t;
        result->value = eval(expr->value, &ctx->value);
        return result;
    } catch (...) {
        return nullptr;
    }
}