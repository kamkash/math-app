// llama_chat_library.cpp
#include "libllama-chat.h"
#include <iostream>
#include <vector>
#include <llama.h>
#include <nlohmann/json.hpp>

LlamaChat::LlamaChat(const std::string& model_path) : model_path_(model_path) {
    loadModel();
}

LlamaChat::~LlamaChat() {
    if (ctx_) {
        llama_free(ctx_);
    }
}

void LlamaChat::loadModel() {
    llama_model_params model_params = llama_model_default_params();
    ctx_ = llama_load_model_from_file(model_path_.c_str(), model_params);

    if (!ctx_) {
        std::cerr << "Failed to load model from " << model_path_ << std::endl;
        exit(1);
    }
}

std::string LlamaChat::generateResponse(const std::string& prompt) {
    // Simplified response generation using llama.cpp.
    // In a real application, you'd handle tokenization, evaluation, etc.
    llama_context_params ctx_params = llama_context_default_params();
    llama_context* lctx = llama_new_context(ctx_, ctx_params);
    if (!lctx){
        std::cerr << "Failed to create context" << std::endl;
        return "";
    }

    std::vector<llama_token> tokens = llama_tokenize(ctx_, prompt.c_str(), prompt.length(), true);

    llama_eval(lctx, tokens.data(), tokens.size(), 0, 1);

    std::string response;
    int n_tokens = 100; // Example: generate 100 tokens
    for (int i = 0; i < n_tokens; ++i) {
        llama_token token = llama_sample_token(lctx, nullptr, 0, 0, 0, 0, 0);
        if (token == llama_token_eos(ctx_)) {
            break;
        }
        response += llama_token_to_str(ctx_, token);
        llama_eval(lctx, &token, 1, tokens.size() + i, 1); //Add the generated token to the context.
    }
    llama_free(lctx);
    return response;
}

std::string LlamaChat::chat(const std::string& prompt) {
    return generateResponse(prompt);
}

nlohmann::json LlamaChat::generateStructuredOutput(const std::string& prompt) {
    // Example: Prompt the model to generate JSON.
    std::string json_prompt = prompt + "\nGenerate JSON output:";
    std::string json_response = generateResponse(json_prompt);

    try {
        return nlohmann::json::parse(json_response);
    } catch (nlohmann::json::parse_error& e) {
        std::cerr << "JSON parse error: " << e.what() << std::endl;
        return {}; // Return an empty JSON object on error
    }
}

void LlamaChat::registerFunction(const std::string& function_name, std::function<nlohmann::json(const nlohmann::json&)> function) {
    functions_[function_name] = function;
}

nlohmann::json LlamaChat::callFunction(const std::string& function_name, const nlohmann::json& arguments) {
    if (functions_.find(function_name) != functions_.end()) {
        return functions_[function_name](arguments);
    } else {
        std::cerr << "Function not found: " << function_name << std::endl;
        return {};
    }
}

