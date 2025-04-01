// libllama-chat.h
#ifndef LLAMA_CHAT_LIBRARY_H
#define LLAMA_CHAT_LIBRARY_H

#include <string>
#include <vector>
#include <map>
#include <functional>
#include <nlohmann/json.hpp> // For JSON handling

class LlamaChat {
public:
    LlamaChat(const std::string& model_path);
    ~LlamaChat();

    std::string chat(const std::string& prompt);

    nlohmann::json generateStructuredOutput(const std::string& prompt);

    void registerFunction(const std::string& function_name, std::function<nlohmann::json(const nlohmann::json&)> function);
    nlohmann::json callFunction(const std::string& function_name, const nlohmann::json& arguments);

private:
    std::string model_path_;
    void* ctx_ = nullptr; // Llama context
    std::map<std::string, std::function<nlohmann::json(const nlohmann::json&)>> functions_;

    // Internal llama.cpp related functions (simplified for example)
    void loadModel();
    std::string generateResponse(const std::string& prompt);
};

#endif // LLAMA_CHAT_LIBRARY_H