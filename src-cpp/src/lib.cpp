#include "llama.h"
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>
#include <mutex>
#include <nlohmann/json.hpp>

#ifdef _WIN32
#define EXPORT __declspec(dllexport)
#else
#define EXPORT __attribute__((visibility("default")))
#endif

extern "C"
{
    void _free_model();
    void _free_context();
    void _free_samplers();
    bool _init_samplers();
    bool _init_model(const char *model_path, int ngl);

    std::mutex model_mutex;
    static llama_model *model = nullptr;
    static llama_context *ctx = nullptr;
    static const llama_vocab *vocab = nullptr;
    static llama_sampler *smpl = nullptr;

    static std::string SAMPLE_GRAMMAR_STRING = R"(root ::= ("-*-" ws item)+
                                                  item ::= [^\n]+ "\n"
                                                  ws ::= [ \t]*
                                                  )";

    EXPORT const char *echo(const char *str)
    {
        static std::string result;
        result = std::string(str) + " from C++";
        return result.c_str();
    }

    EXPORT bool init(const char *model_path, int ngl)
    {
        fprintf(stderr, "%s: model file path %s\n", __func__, model_path);
        std::lock_guard<std::mutex> lock(model_mutex);

        if (model == nullptr)
        {
            // load dynamic backends
            ggml_backend_load_all();
        }
        else
        {
            _free_model();
        }

        // initialize the model
        if (!_init_model(model_path, ngl))
        {
            fprintf(stderr, "%s: error: failed to load the model\n", __func__);
            return false;
        }
        _init_samplers();
        return true;
    }

    EXPORT bool add_grammar(const char *grammar)
    {
        if (grammar != nullptr)
        {
            fprintf(stderr, "%s: Adding grammar: \n%s\n", __func__, grammar);
            _free_samplers();
            auto sparams = llama_sampler_chain_default_params();
            sparams.no_perf = true;
            smpl = llama_sampler_chain_init(sparams);
            auto smpl_gr = llama_sampler_init_grammar(vocab, grammar, "root");
            if (smpl_gr == nullptr)
            {
                fprintf(stderr, "%s: error: failed to create the grammar sampler\n", __func__);
                return false;
            }
            llama_sampler_chain_add(smpl, smpl_gr);
            llama_sampler_chain_add(smpl, llama_sampler_init_greedy());
            // llama_sampler_chain_add(smpl, llama_sampler_init_dist(1));
            return true;
        }
        else
        {
            fprintf(stderr, "%s: error: grammar is null\n", __func__);
            return false;
        }
    }

    EXPORT bool reset_context()
    {
        if (ctx != nullptr)
        {
            llama_free(ctx);
            ctx = nullptr;
        }
        if (smpl != nullptr)
        {
            llama_sampler_free(smpl);
            smpl = nullptr;
        }
        return true;
    }

    EXPORT const char *generate_text(const char *prompt, int n_predict)
    {
        static std::string result;
        result.clear();

        if (model == nullptr || vocab == nullptr)
        {
            fprintf(stderr, "%s: error: model is not initialized. call init() first\n", __func__);
            return nullptr;
        }
        fprintf(stderr, "%s: Prompt: %s\n", __func__, prompt);

        // tokenize the prompt
        const int n_prompt = -llama_tokenize(vocab, prompt, strlen(prompt), NULL, 0, true, true);
        std::vector<llama_token> prompt_tokens(n_prompt);
        if (llama_tokenize(vocab, prompt, strlen(prompt), prompt_tokens.data(), prompt_tokens.size(), true, true) < 0)
        {
            fprintf(stderr, "%s: error: failed to tokenize the prompt\n", __func__);
            return nullptr;
        }

        if (ctx == nullptr)
        {
            // initialize the context
            llama_context_params ctx_params = llama_context_default_params();
            ctx_params.n_ctx = n_prompt + n_predict - 1;
            ctx_params.n_batch = n_prompt;
            ctx_params.no_perf = true;
            ctx = llama_init_from_model(model, ctx_params);
            // ctx = llama_new_context_with_model(model, ctx_params);
            if (ctx == NULL)
            {
                fprintf(stderr, "%s: error: failed to create the llama_context\n", __func__);
                return nullptr;
            }
        }
        else
        {
            // llama_perf_context_reset(ctx);
            llama_kv_self_clear(ctx);
        }
        if (smpl == nullptr)
        {
            _init_samplers();
        }

        // prepare a batch for the prompt
        llama_batch batch = llama_batch_get_one(prompt_tokens.data(), prompt_tokens.size());

        // main loop
        const auto t_main_start = ggml_time_us();
        int n_decode = 0;
        llama_token new_token_id;
        for (int n_pos = 0; n_pos + batch.n_tokens < n_prompt + n_predict;)
        {
            if (llama_decode(ctx, batch))
            {
                fprintf(stderr, "%s : failed to eval, return code %d\n", __func__, 1);
                return nullptr;
            }
            n_pos += batch.n_tokens;
            new_token_id = llama_sampler_sample(smpl, ctx, -1);
            if (llama_vocab_is_eog(vocab, new_token_id))
            {
                break;
            }
            char buf[128];
            int n = llama_token_to_piece(vocab, new_token_id, buf, sizeof(buf), 0, true);
            if (n < 0)
            {
                fprintf(stderr, "%s: error: failed to convert token to piece\n", __func__);
                return nullptr;
            }
            result.append(buf, n);
            batch = llama_batch_get_one(&new_token_id, 1);
            n_decode += 1;
        }

        const auto t_main_end = ggml_time_us();

        fprintf(stderr, "%s: decoded %d tokens in %.2f s, speed: %.2f t/s\n",
                __func__, n_decode, (t_main_end - t_main_start) / 1000000.0f, n_decode / ((t_main_end - t_main_start) / 1000000.0f));

        // fprintf(stderr, "\n");
        // llama_perf_sampler_print(smpl);
        // llama_perf_context_print(ctx);
        // fprintf(stderr, "\n");

        return result.c_str();
    }

    //////////////////////////////////////////////////////////////////////////////////
    // initialize the sampler
    bool _init_samplers()
    {
        _free_samplers();
        auto sparams = llama_sampler_chain_default_params();
        sparams.no_perf = true;
        smpl = llama_sampler_chain_init(sparams);
        if (smpl == nullptr)
        {
            fprintf(stderr, "%s: error: failed to create the sampler chain\n", __func__);
            return false;
        }
        llama_sampler_chain_add(smpl, llama_sampler_init_greedy());
        return true;
    }

    bool _init_model(const char *model_path, int ngl)
    {
        llama_model_params model_params = llama_model_default_params();
        model_params.n_gpu_layers = ngl;

        model = llama_model_load_from_file(model_path, model_params);
        vocab = llama_model_get_vocab(model);

        if (model == nullptr)
        {
            fprintf(stderr, "%s: error: unable to load model\n", __func__);
            return false;
        }
        return true;
    }

    void _free_model()
    {
        llama_model_free(model);
        model = nullptr;
        _free_context();
        _free_samplers();
    }

    void _free_context()
    {
        if (ctx != nullptr)
        {
            llama_free(ctx);
            ctx = nullptr;
        }
    }

    void _free_samplers()
    {
        if (smpl != nullptr)
        {
            llama_sampler_free(smpl);
            smpl = nullptr;
        }
    }
}
