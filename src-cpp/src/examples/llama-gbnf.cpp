#include <iostream>
#include <vector>
#include <string>
#include <llama.h>

int main() {
    const std::string model_path = "path/to/your/model.gguf"; // Replace with your model path
    const std::string grammar_path = "path/to/your/grammar.gbnf"; // Replace with your grammar path
    const std::string prompt = "Generate a JSON object:";

    // Load the model
    llama_model_params model_params = llama_model_default_params();
    llama_model* model = llama_load_model_from_file(model_path.c_str(), model_params);
    if (!model) {
        std::cerr << "Failed to load model from " << model_path << std::endl;
        return 1;
    }

    // Load the grammar
    llama_grammar* grammar = llama_grammar_from_file(grammar_path.c_str());
    if (!grammar) {
        std::cerr << "Failed to load grammar from " << grammar_path << std::endl;
        llama_free_model(model);
        return 1;
    }

    // Create a context
    llama_context_params ctx_params = llama_context_default_params();
    llama_context* ctx = llama_new_context(model, ctx_params);
    if (!ctx) {
        std::cerr << "Failed to create context" << std::endl;
        llama_free_grammar(grammar);
        llama_free_model(model);
        return 1;
    }

    // Tokenize the prompt
    std::vector<llama_token> tokens = llama_tokenize(model, prompt.c_str(), prompt.length(), true);

    // Evaluate the prompt
    llama_eval(ctx, tokens.data(), tokens.size(), 0, 1);

    // Generate text with grammar constraints
    std::string generated_text;
    int n_tokens = 200; // Maximum number of tokens to generate
    for (int i = 0; i < n_tokens; ++i) {
        llama_token token;
        llama_token_data_array* candidates = llama_sample_begin(ctx, nullptr, 0, 0, 0, grammar);
        if (candidates->size == 0) {
          break;
        }

        token = llama_sample_token_from_array(ctx, candidates);
        llama_sample_end(ctx, candidates);

        if (token == llama_token_eos(model)) {
            break;
        }

        generated_text += llama_token_to_str(model, token);
        llama_eval(ctx, &token, 1, tokens.size() + i, 1);
    }
    std::cout << "Generated Text: " << generated_text << std::endl;

    // Clean up
    llama_free(ctx);
    llama_free_grammar(grammar);
    llama_free_model(model);

    return 0;
}