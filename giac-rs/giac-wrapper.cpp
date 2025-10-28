// Convert a gen_t to f64 if possible. Returns 1 on success, 0 on failure. Writes result to out_ptr.

#include "giac-wrapper.h"

#include "gen.h"
#include "giac.h"
#include "static_extern.h"
#include "sym2poly.h"

#include <string>
#include <new>
#define USE_LOCK
#ifdef USE_LOCK
#include <mutex>
#endif

using namespace giac;

// Toggleable global mutex for thread safety. Define USE_LOCK to enable locking.
#ifdef USE_LOCK
// Ugly global mutex for thread safety
static std::mutex giac_mutex;
#endif

// Lock macro: expands to a std::lock_guard when USE_LOCK is defined, otherwise a no-op.
#ifdef USE_LOCK
#define GIAC_LOCK() std::lock_guard<std::mutex> lock(giac_mutex)
#else
#define GIAC_LOCK() ((void)0)
#endif

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
    GIAC_LOCK();
    return new context_t{context()};
}

void context_free(context_t *ctx)
{
    GIAC_LOCK();
    delete ctx;
}

context_t *context_clone(const context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        context_t *new_ctx = new context_t();
        new_ctx->value = ctx->value;
        return new_ctx;
    }
    catch (const std::bad_alloc &)
    {
        return nullptr; // Memory allocation failed
    }
}

// Gen management
gen_t *gen_new(const char *expr, context_t *ctx)
{
    GIAC_LOCK();
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

gen_t *gen_symbol(const char *expr, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *g = new gen_t();
        std::string str(expr);
        identificateur sym(str);
        g->value = gen(sym);
        return g;
    }
    catch (...)
    {
        return nullptr;
    }
}


// Parse API: explicit alias for creating a gen from a string.
// Kept as a separate function to make intent clear to C callers.
gen_t *gen_parse(const char *expr, context_t *ctx)
{
    // Delegate to gen_new which already wraps giac::gen
    return gen_new(expr, ctx);
}

gen_t *gen_new_from_double(double value, context_t *ctx)
{
    GIAC_LOCK();
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
    GIAC_LOCK();
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
    GIAC_LOCK();
    delete g;
}

// Example API
gen_t *gen_simplify(gen_t *g, context_t *ctx)
{
    GIAC_LOCK();
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
    GIAC_LOCK();
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
    GIAC_LOCK();
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
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = a->value + b->value;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_sub(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = a->value - b->value;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_mul(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = a->value * b->value;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_div(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = a->value / b->value;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_pow(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = pow(a->value, b->value, &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

// Symbolic constants
gen_t *gen_pi(context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = cst_pi;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_e(context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = exp(1);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

// Symbolic operators
gen_t *gen_symb_plus(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_plus, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_mult(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_multiply, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_pow(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_pow, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_sub(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_minus, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_div(gen_t *a, gen_t *b, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        v.push_back(b->value);
        result->value = symbolic(at_divide, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_sin(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_sin, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_cos(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_cos, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_tan(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_tan, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_sinh(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_sinh, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_cosh(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_cosh, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_tanh(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_tanh, v);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_sqrt(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        vecteur v;
        v.push_back(a->value);
        result->value = symbolic(at_sqrt, v);
        return result;
    }
    catch (...)
    {
        // If evaluation fails for any reason, keep the symbolic form.
        return nullptr;
    }
}

gen_t *gen_symb_log(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = symbolic(at_log10, a->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_ln(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = symbolic(at_ln, a->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_symb_exp(gen_t *a, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = symbolic(at_exp, a->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

// Substitute variables in an expression
// vars: array of variable names, values: array of gen_t*, n: number of substitutions
gen_t *gen_subs(gen_t *expr, const char **vars, gen_t **values, size_t n, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        vecteur subs_vec;
        subs_vec.push_back(expr->value); // First element is the expression itself
        for (size_t i = 0; i < n; ++i)
        {
            subs_vec.push_back(symb_equal(identificateur(vars[i]), values[i]->value));
        }
        gen_t *result = new gen_t;
        result->value = _subs(gen(subs_vec), &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

// Evaluate an expression in a context
gen_t *gen_eval(gen_t *expr, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = eval(expr->value, &ctx->value);
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

gen_t *gen_clone(gen_t *g, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        gen_t *result = new gen_t;
        result->value = g->value;
        return result;
    }
    catch (...)
    {
        return nullptr;
    }
}

int gen_is_vecteur(const gen_t *g, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        if (!g)
            return 0;
        if (g->value.type == giac::_VECT)
            return 1;
        else
            return 0;
    }
    catch (...)
    {
        return 0;
    }
}

size_t gen_vecteur_len(const gen_t *g, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        if (!g)
            return 0;
        if (g->value.type == giac::_VECT)
            return g->value._VECTptr->size();
        else
            return 0;
    }
    catch (...)
    {
        return 0;
    }
}

gen_t *gen_vecteur_get(const gen_t *g, size_t i, context_t *ctx)
{
    GIAC_LOCK();
    try
    {
        if (!g)
            return nullptr;

        if (g->value.type == giac::_VECT)
        {
            if (i < g->value._VECTptr->size())
            {
                gen_t *out = new gen_t();
                out->value = g->value._VECTptr->at(i);
                return out;
            }
        }
        return nullptr;
    }
    catch (...)
    {
        return nullptr;
    }
}

int is_symbol(const gen_t *g)
{
    GIAC_LOCK();
    return g->value.type == _IDNT;
}

int is_number(const gen_t *g)
{
    GIAC_LOCK();
    return g->value.type == _INT_ || g->value.type == _DOUBLE_ || g->value.type == _FLOAT_ ||
           g->value.type == _ZINT || g->value.type == _REAL || g->value.type == _CPLX ||
           g->value.type == _FRAC || g->value.type == _EXT;
}

int is_constant(const gen_t *g)
{
    GIAC_LOCK();
    return g->value.is_constant();
}

int equals(const gen_t *a, const gen_t *b)
{
    GIAC_LOCK();
    return a->value == b->value;
}

/////////////////////////////////////////////////////////////////
/// Operators.
/////////////////////////////////////////////////////////////////

// Singleton operator identifiers
static const identificateur DIV_ID("__DIV__");
static const identificateur AND_ID("__AND__");
static const identificateur OR_ID("__OR__");
static const identificateur NOT_ID("__NOT__");
static gen_t GEN_ADD = {at_plus};
static gen_t GEN_SUB = {at_minus};
static gen_t GEN_MUL = {at_prod};
static gen_t GEN_POW = {at_pow};
static gen_t GEN_DIV = {gen(DIV_ID)};
static gen_t GEN_AND = {gen(AND_ID)};
static gen_t GEN_OR = {gen(OR_ID)};
static gen_t GEN_NOT = {gen(NOT_ID)};
static gen_t GEN_EQ = {at_equal};
static gen_t GEN_NE = {at_different};
static gen_t GEN_LT = {at_inferieur_strict};
static gen_t GEN_LE = {at_inferieur_egal};
static gen_t GEN_GT = {at_superieur_strict};
static gen_t GEN_GE = {at_superieur_egal};

int is_add(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_plus;
}

int is_sub(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_minus;
}

int is_mul(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_prod;
}

int is_div(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_IDNT && g->value == GEN_DIV.value;
}

int is_pow(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_pow;
}

int is_and(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_IDNT && g->value == GEN_AND.value;
}

int is_or(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_IDNT && g->value == GEN_OR.value;
}
int is_not(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_IDNT && g->value == GEN_NOT.value;
}
int is_eq(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_equal;
}
int is_ne(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_different;
}
int is_lt(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_inferieur_strict;
}
int is_gt(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_superieur_strict;
}
int is_le(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_inferieur_egal;
}
int is_ge(const gen_t *g)
{
    GIAC_LOCK();
    return g && g->value.type == giac::_FUNC && g->value._SYMBptr->sommet == giac::at_superieur_egal;
}

gen_t *get_add_op()
{
    return &GEN_ADD;
}

gen_t *get_sub_op()
{
    return &GEN_SUB;
}

gen_t *get_mul_op()
{
    return &GEN_MUL;
}

gen_t *get_div_op()
{
    return &GEN_DIV;
}

gen_t *get_pow_op()
{
    return &GEN_POW;
}

gen_t *get_and_op()
{
    return &GEN_AND;
}

gen_t *get_or_op()
{
    return &GEN_OR;
}

gen_t *get_not_op()
{
    return &GEN_NOT;
}

gen_t *get_eq_op()
{
    return &GEN_EQ;
}

gen_t *get_lt_op()
{
    return &GEN_LT;
}

gen_t *get_le_op()
{
    return &GEN_LE;
}

gen_t *get_gt_op()
{
    return &GEN_GT;
}

gen_t *get_ge_op()
{
    return &GEN_GE;
}

gen_t *get_ne_op()
{
    return &GEN_NE;
}

int gen_to_f64(gen_t *g, double *out_ptr)
{
    GIAC_LOCK();
    try
    {
        if (!g || !out_ptr)
            return 0;
        // Try to convert to double using gen::to_double()
        double val = 0.0;
        if (g->value.type == _DOUBLE_ || g->value.type == _FLOAT_ || g->value.type == _REAL ||
            g->value.type == _INT_ || g->value.type == _ZINT || g->value.type == _FRAC)
        {
            val = g->value.to_double(nullptr);
            *out_ptr = val;
            return 1;
        }
        // Try to evaluate to a number if possible
        gen evaluated = g->value.evalf(1, 0);
        if (evaluated.type == _DOUBLE_ || evaluated.type == _FLOAT_ || evaluated.type == _REAL ||
            evaluated.type == _INT_ || evaluated.type == _ZINT || evaluated.type == _FRAC)
        {
            val = evaluated.to_double(nullptr);
            *out_ptr = val;
            return 1;
        }
        return 0;
    }
    catch (...)
    {
        return 0;
    }
}